# M31 Kernel-Basis Transform — Stable ARM64 Parity with Stwo

Date frozen: 2026-09-01

Canonical scientific home: LAB-010 — M31 Research Lab — Arithmetic, Transforms, Extensions & FPGA
Primary namespace: M31-S05 — FFT / transforms over M31
Cross-reference: M31-S07 — STWO mapping

## Research result

Algorizk's native kernel-basis M31 transform reached stable single-thread ARM64
performance parity with the pinned Stwo production SIMD Circle FFT baseline.

Pinned Stwo commit:
826591c6c371376810ca8213b5812d5daf6d5092

Largest benchmark size:
N = 2^20 = 1,048,576

Stability gate:
- independent rounds: 7
- repetitions per benchmark point: 9
- median kernel / Stwo ratio: 0.97951
- observed ratio range: 0.97663 .. 0.996149
- median kernel transform: 6.92475 ms
- median Stwo SIMD: 7.058 ms
- rounds within ±5% parity band: 7/7

## Final implementation path

1. kernel-basis shear yielding a native 1M+2A butterfly;
2. bit-reversed schedule with twiddle reuse;
3. true radix-8 register fusion;
4. raw redundant M31 representation in [0,P];
5. direct ARM64 NEON arithmetic;
6. fused final k=1,k=0 four-element NEON microkernel.

## Important limitations

- This is a native-transform benchmark.
- Canonical kernel-basis -> sheared-basis conversion is not included in transform timing.
- No PCS claim.
- No end-to-end prover claim.
- No claim that the kernel is materially faster than Stwo; the supported conclusion is stable performance parity.
- Current evidence is single-thread ARM64 / Apple Silicon under the recorded build settings.

## Decision

STOP_TRANSFORM_TUNING.

Next work:
1. create the canonical GitHub repository and attach code provenance;
2. freeze a reproducibility package;
3. write the full research paper, including negative results and future work.
