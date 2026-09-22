# Research status

Status: **frozen transform result / paper preparation**

Canonical claim:

> On the recorded single-thread Apple ARM64 benchmark, the optimized native
> M31 kernel-basis transform reaches stable performance parity with the pinned
> Stwo production SIMD Circle FFT at sizes through 2^20.

Largest-size stability result:

- N: 2^20
- median kernel/Stwo: 0.97951000
- range: 0.97663000 .. 0.99614900
- parity-band rounds: 7/7

Do not claim:

- a material speedup over Stwo;
- end-to-end prover parity;
- PCS performance;
- parity including kernel-basis shear conversion;
- cross-platform parity without new measurements.
