# Reproducibility contract

## Upstream baseline

Repository: starkware-libs/stwo

Pinned commit:

`826591c6c371376810ca8213b5812d5daf6d5092`

## Build policy

- architecture: Apple ARM64 / AArch64
- single Rayon thread
- Rust release profile
- `-C target-cpu=native`
- release codegen units: 1
- ThinLTO

## Frozen source provenance

Canonical archived v0.20.1 bundle SHA-256:

`9ad6715e41f4dc428ff24f13d309da932630250a40d81b6db05c5c4f73ce589d`

Canonical Desk stability CSV SHA-256:

`2391777c4872def6ecb903b5666b5ec0b47b87fa6bff79249d366e073c201232`

Canonical Desk project brief SHA-256:

`dac321513862d6aeb12bc0f78b65958c71d7cce04166203547d0b4d1070cda26`

## Correctness gate

The benchmark verifies the optimized raw-NEON transform against the scalar
true-radix-8 transform and original kernel evaluation on small powers of two
before performance measurements are accepted.

## Statistical stability gate

The frozen parity result uses seven independent executions, each using nine
internal repetitions for benchmark points at logN 18..20. At logN=20, all
seven observed kernel/Stwo ratios remained within [0.95, 1.05].
