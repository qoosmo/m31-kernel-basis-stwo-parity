# Archived optimization evidence verification

All values below are read from existing benchmark CSVs. No benchmark was rerun.

The approximate numbers already present in the manuscript were used only to
identify the corresponding archived run among possible reruns. The evidence
values below are copied from the selected CSV rows.

| Version | Type | Milestone | Kernel ms | Stwo SIMD ms | K/Stwo | Source SHA-256 |
|---|---|---|---:|---:|---:|---|
| v0.3 | trajectory | initial recursive kernel | 44.798 | 10.605 | 4.224 | `104ccbe4c729b0bd5400c98b8b79fdaad47da256289ba8f25dbcca05e4a7c1e0` |
| v0.4 | trajectory | same Stwo BaseField arithmetic | 36.409 | 10.747 | 3.388 | `e10c0bd5814489369d405549db639597499e3225bd508d9823e3c0fe27b2cd81` |
| v0.5 | trajectory | in-place domain order | 33.755 | 10.568 | 3.194 | `7e4ccc45876fbe02e916249fa912688fb28f63e34ab4071ebe6b54921518c26d` |
| v0.6 | trajectory | bit-reversed twiddle-reuse schedule | 15.462 | 10.579 | 1.462 | `bba394bc339e8623057789dcaab1a3a1bd62e3befd531c3efb659e667d0a7ca1` |
| v0.7 | trajectory | Boolean-zeta 1M+2A coordinates | 13.326 | 10.715 | 1.244 | `b9347b9cc88492d55add9f58406a8240d680638bc5695c13023f708f4b3a58cc` |
| v0.8 | trajectory | three-level loop grouping | 12.732 | 10.846 | 1.174 | `549ffd27be1f181c753ecc5f0f5960f33ea17910eaf283539f30a5363af63ce4` |
| v0.12 | trajectory | native codegen / scalar sweep | 10.029 | 6.963 | 1.440 | `206fb6d46e1ac2fca5fc8f2a7425d3b7b84a56f92114b2ec443df33804fb7db8` |
| v0.13 | trajectory | true scalar radix-8 register fusion | 9.450 | 7.970 | 1.186 | `67aaba6eeda229dc7ae7ab1cd78ecd6aee42a037173b3f66fb3864d28e653023` |
| v0.14 | negative | naive cache blocking | 11.248 | 7.105 | 1.583 | `2f88e514f24add12b90e0fc706b3f0a9258ad6a0540ead970b261a3e73fb1e2a` |
| v0.15 | negative | generic PackedM31 true radix-8 | 10.777 | 7.046 | 1.529 | `854979e98b2e72cb6fe7ec048265697324af27d39bb5ba5a6f85346777ba8acf` |
| v0.17 | negative | PackedM31 radix-8 + exact Stwo twiddle mul | 10.855 | 6.978 | 1.556 | `8cc6cea6f2af9bebd705b9bd3396c305421934bf9f9a2651d48dfab311486ba6` |
| v0.18 | trajectory | raw redundant M31 direct NEON | 9.141 | 7.060 | 1.295 | `4d928972ff33cf45ea78203179d3deacd5fadea19acf765ff12b66685eb030dd` |
| v0.20 | trajectory | fused final two-level NEON tail | 6.917 | 6.921 | 0.999 | `cc8a457e7d622b0ab6f62525045598957a29fae7b986c42daa22c61107ccf3d1` |

## v0.19 stage-profile evidence

- source: `$HOME/algorizk-rd/benchmarks/kernel-vs-stwo-m31/results/kernel-raw-neon-stage-profile-20260831-232253.csv`
- SHA-256: `d668d84684e3b6e8d1d4dcf36a26a4e49a8ebfebb604c039a42efa8276751764`
- full raw-NEON transform: `9.146 ms`
- Stwo SIMD: `7.027 ms`
- final two-level scalar tail: `3.484 ms`
- tail share: `38.5%`

## Evidence decision

**PASS — the major manuscript optimization milestones are tied to hashed archived CSVs.**

The next gate may generate:
- ratio trajectory figure, with the v0.12 build-regime boundary marked;
- negative-ablation table/figure;
- v0.19 stage-profile figure.
