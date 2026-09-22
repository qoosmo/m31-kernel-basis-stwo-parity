# M31 Kernel-Basis Transform — Stable ARM64 Parity with Stwo

Algorizk research repository for a native Mersenne-31 kernel-basis transform
that reaches stable single-thread ARM64 performance parity with Stwo's
production SIMD Circle FFT baseline.

## Frozen result

Canonical Desk record:

- Lab: `LAB-010 — M31 Research Lab — Arithmetic, Transforms, Extensions & FPGA`
- Run: `#14 — M31-S05/kernel-basis-transform-stwo-parity-arm64-v0.20.1`
- Date frozen: 2026-09-01

Pinned Stwo commit:

`826591c6c371376810ca8213b5812d5daf6d5092`

At `N = 2^20 = 1,048,576`, over seven independent stability rounds:

- median kernel / Stwo ratio: **0.97951000**
- observed ratio range: **0.97663000 .. 0.99614900**
- median kernel transform: **6.924750 ms**
- median Stwo SIMD transform: **7.058000 ms**
- rounds within ±5% parity band: **7/7**

The supported conclusion is **stable performance parity**, not a claim of a
material speedup over Stwo.

## Final implementation path

1. kernel-basis shear giving a native 1M+2A butterfly;
2. bit-reversed schedule with twiddle reuse;
3. true radix-8 register fusion;
4. raw redundant M31 representation in `[0,P]`;
5. direct ARM64 NEON arithmetic;
6. fused final `k=1,k=0` four-element NEON microkernel.

## Reproduce

On Apple ARM64 with Rust/Cargo/Git installed:

```bash
./scripts/run-benchmark.command
./scripts/run-stability.command
```

The scripts fetch the pinned Stwo revision and apply the same build policy used
for the frozen result:

- `RAYON_NUM_THREADS=1`
- `target-cpu=native`
- `codegen-units=1`
- ThinLTO

## Scope and limitations

This result is a **native-transform benchmark**.

It does **not** include the canonical kernel-basis → sheared-basis conversion
inside transform timing. It is not an end-to-end prover benchmark and makes no
PCS claim.

Current evidence is specific to the recorded single-thread Apple ARM64
environment. Cross-architecture measurements are future work.

## Repository status

This repository is a public research release. No open-source license is granted
by the absence of a LICENSE file.

## Paper

The full paper will be developed under `paper/` after the experimental result
and repository provenance are frozen.
