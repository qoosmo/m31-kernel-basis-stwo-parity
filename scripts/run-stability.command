#!/bin/bash
set -euo pipefail

VERSION="v0.20.1"
STWO_SHA="826591c6c371376810ca8213b5812d5daf6d5092"
ROOT="$HOME/algorizk-rd/benchmarks/kernel-vs-stwo-m31"
STWO="$ROOT/stwo"
RESULTS="$ROOT/results"
HERE="$(cd "$(dirname "$0")" && pwd)"

ROUNDS="${ROUNDS:-7}"
REPS="${REPS:-9}"
MIN_LOG="${MIN_LOG:-18}"
MAX_LOG="${MAX_LOG:-20}"

mkdir -p "$ROOT" "$RESULTS"
for c in git cargo rustc python3; do
  command -v "$c" >/dev/null || { echo "ERROR: missing $c"; exit 2; }
done

if [ ! -d "$STWO/.git" ]; then
  git clone --filter=blob:none --no-checkout https://github.com/starkware-libs/stwo.git "$STWO"
fi

cd "$STWO"
git fetch --depth 1 origin "$STWO_SHA" >/dev/null
git checkout --detach -f "$STWO_SHA" >/dev/null
git clean -fd >/dev/null

mkdir -p crates/stwo/examples
cp "$HERE/kernel_vs_stwo.rs" crates/stwo/examples/kernel_vs_stwo.rs

STAMP="$(date +%Y%m%d-%H%M%S)"
RUN_DIR="$RESULTS/kernel-fused-neon-tail2-stability-$STAMP"
mkdir -p "$RUN_DIR"

echo "============================================================"
echo "ALGORIZK — KERNEL/STWO PARITY STABILITY GATE — $VERSION"
echo "============================================================"
echo "PINNED_STWO_SHA=$STWO_SHA"
echo "ARCH=$(uname -m)"
echo "TRANSFORM_SOURCE=UNCHANGED_FROM_v0.20.0"
echo "ROUNDS=$ROUNDS REPS=$REPS MIN_LOG=$MIN_LOG MAX_LOG=$MAX_LOG"
echo "RAYON_NUM_THREADS=1"
echo "BUILD=target-cpu=native, codegen-units=1, thin-LTO"
echo

export RAYON_NUM_THREADS=1
export RUSTFLAGS="${RUSTFLAGS:--C target-cpu=native}"
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1
export CARGO_PROFILE_RELEASE_LTO=thin

# Build once. Subsequent rounds execute the same binary.
cargo build -p stwo --release --features prover --example kernel_vs_stwo
BIN="target/release/examples/kernel_vs_stwo"

for r in $(seq 1 "$ROUNDS"); do
  CSV="$RUN_DIR/round-$(printf '%02d' "$r").csv"
  echo
  echo "================ ROUND $r / $ROUNDS ================"
  MIN_LOG="$MIN_LOG" MAX_LOG="$MAX_LOG" REPS="$REPS" BENCH_OUT="$CSV" "$BIN"
  # Small spacing makes thermal drift visible rather than hammering launches back-to-back.
  sleep 1
done

SUMMARY="$RUN_DIR/stability-summary.csv"

python3 - "$RUN_DIR" "$SUMMARY" "$MIN_LOG" "$MAX_LOG" <<'PY'
import csv, glob, os, statistics, sys

run_dir, summary_path, min_log, max_log = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
files = sorted(glob.glob(os.path.join(run_dir, "round-*.csv")))
if not files:
    raise SystemExit("ERROR: no round CSVs")

by_log = {k: [] for k in range(min_log, max_log+1)}
for fn in files:
    with open(fn, newline="") as f:
        for row in csv.DictReader(f):
            k = int(row["log_n"])
            if k in by_log:
                by_log[k].append(row)

print()
print("STABILITY SUMMARY")
print("=================")
print("logN      N rounds median(T2/STWO) min      max      median_tail2_ms median_stwo_ms within_5pct")

out_rows=[]
for k in range(min_log, max_log+1):
    rows=by_log[k]
    ratios=[float(x["tail2_over_stwo"]) for x in rows]
    tail=[int(x["tail2_neon_ns"])/1e6 for x in rows]
    stwo=[int(x["stwo_simd_ns"])/1e6 for x in rows]
    med=statistics.median(ratios)
    lo=min(ratios); hi=max(ratios)
    mt=statistics.median(tail); ms=statistics.median(stwo)
    within=sum(0.95 <= x <= 1.05 for x in ratios)
    print(f"{k:>4} {int(rows[0]['n']):>8} {len(rows):>6} {med:>15.4f} {lo:>8.4f} {hi:>8.4f} {mt:>16.3f} {ms:>14.3f} {within:>5}/{len(rows)}")
    out_rows.append({
        "log_n":k, "n":int(rows[0]["n"]), "rounds":len(rows),
        "median_tail2_over_stwo":f"{med:.8f}",
        "min_tail2_over_stwo":f"{lo:.8f}",
        "max_tail2_over_stwo":f"{hi:.8f}",
        "median_tail2_ms":f"{mt:.6f}",
        "median_stwo_ms":f"{ms:.6f}",
        "within_5pct":within,
    })

with open(summary_path,"w",newline="") as f:
    w=csv.DictWriter(f,fieldnames=list(out_rows[0].keys()))
    w.writeheader(); w.writerows(out_rows)

# Final decision is based on the largest N.
last=out_rows[-1]
med=float(last["median_tail2_over_stwo"])
lo=float(last["min_tail2_over_stwo"])
hi=float(last["max_tail2_over_stwo"])
within=int(last["within_5pct"])
rounds=int(last["rounds"])

print()
print("FINAL DECISION @ LARGEST N")
print("==========================")
print(f"MEDIAN_T2_STWO={med:.4f}")
print(f"RANGE_T2_STWO={lo:.4f}..{hi:.4f}")
print(f"WITHIN_5PCT={within}/{rounds}")

if 0.97 <= med <= 1.03 and within >= max(5, rounds-2):
    print("DECISION=PARITY_STABLE")
    print("ACTION=STOP_TRANSFORM_TUNING")
elif 0.95 <= med <= 1.05:
    print("DECISION=PARITY_PROVISIONAL")
    print("ACTION=STOP_OPTIMIZATION__DOCUMENT_NOISE_AND_REPEAT_ONLY_IF_NEEDED")
else:
    print("DECISION=PARITY_NOT_STABLE")
    print("ACTION=DO_NOT_CLAIM_PARITY__INSPECT_RUN_TO_RUN_DRIFT")

print()
print("SUMMARY_CSV="+summary_path)
print("RUN_DIR="+run_dir)
PY

echo
echo "No transform code was changed in this stability gate."
