#!/bin/bash
set -euo pipefail
VERSION="v0.20.0"
STWO_SHA="826591c6c371376810ca8213b5812d5daf6d5092"
ROOT="$HOME/algorizk-rd/benchmarks/kernel-vs-stwo-m31"
STWO="$ROOT/stwo"
RESULTS="$ROOT/results"
HERE="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$ROOT" "$RESULTS"
for c in git cargo rustc; do command -v "$c" >/dev/null || { echo "ERROR: missing $c"; exit 2; }; done
if [ ! -d "$STWO/.git" ]; then git clone --filter=blob:none --no-checkout https://github.com/starkware-libs/stwo.git "$STWO"; fi
cd "$STWO"
git fetch --depth 1 origin "$STWO_SHA" >/dev/null
git checkout --detach -f "$STWO_SHA" >/dev/null
git clean -fd >/dev/null

mkdir -p crates/stwo/examples
cp "$HERE/kernel_vs_stwo.rs" crates/stwo/examples/kernel_vs_stwo.rs
STAMP="$(date +%Y%m%d-%H%M%S)"
SINGLE="$RESULTS/kernel-fused-neon-tail2-$STAMP.csv"
echo "============================================================"
echo "ALGORIZK — KERNEL FUSED NEON LAST-2-LEVEL GATE — $VERSION"
echo "============================================================"
echo "PINNED_STWO_SHA=$STWO_SHA"
echo "ARCH=$(uname -m)"
echo "MIN_LOG=${MIN_LOG:-18} MAX_LOG=${MAX_LOG:-20}"
echo "BUILD=target-cpu=native, codegen-units=1, thin-LTO (applies to BOTH kernel and Stwo)"
echo
RAYON_NUM_THREADS=1 RUSTFLAGS="${RUSTFLAGS:--C target-cpu=native}" CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 CARGO_PROFILE_RELEASE_LTO=thin MIN_LOG="${MIN_LOG:-18}" MAX_LOG="${MAX_LOG:-20}" REPS="${REPS:-0}" BENCH_OUT="$SINGLE"   cargo run -p stwo --release --features prover --example kernel_vs_stwo

echo
echo "FAST VIEW"
python3 - "$SINGLE" <<'PY2' 2>/dev/null || true
import csv,sys
rows=list(csv.DictReader(open(sys.argv[1])))
print("logN      N OLD/T2 SC/T2 T2/SC T2/CPU T2/STWO OLD/STWO scalar_ms old_ms tail2_ms stwo_ms")
for x in rows:
 print(f"{x['log_n']:>4} {x['n']:>8} {float(x['old_over_tail2']):>6.3f} {float(x['scalar_over_tail2']):>5.3f} {float(x['tail2_over_scalar']):>5.3f} {float(x['tail2_over_cpu']):>6.3f} {float(x['tail2_over_stwo']):>7.3f} {float(x['old_over_stwo']):>8.3f} {int(x['scalar_r8_ns'])/1e6:>9.3f} {int(x['old_raw_neon_ns'])/1e6:>6.3f} {int(x['tail2_neon_ns'])/1e6:>8.3f} {int(x['stwo_simd_ns'])/1e6:>7.3f}")
PY2
echo
echo "SINGLE_CSV=$SINGLE"
