# Figure and table provenance

## Generated at this gate

### `paper/figures/stability-parity.svg`

Derived **only** from:

`results/stability-summary.csv`

SHA-256:

`2391777c4872def6ecb903b5666b5ec0b47b87fa6bff79249d366e073c201232`

Content:
- logN 18, 19, 20;
- seven-round median kernel/Stwo ratio;
- observed min/max ratio;
- parity line at 1.00;
- predeclared ±5% parity band.

No remembered or hand-entered historical benchmark values are used.

### `paper/tables/final-stability.md`
### `paper/tables/final-stability.csv`

Direct publication-table projection of the same frozen CSV.

## Deliberately not generated yet

The optimization-waterfall figure is deferred until every historical v0.3–v0.20
row is verified against durable archived logs. The manuscript currently contains
those historical values, but this gate does not promote them into a figure
without independent provenance.

## Verified optimization-history figures

### `paper/figures/optimization-trajectory.svg`

Derived only from:

`paper/evidence/optimization-verified.csv`

SHA-256:

`b7adee0d10c1700fe1fbf578e6db5b36db5259f2c31512123edd7f376c1e4c44`

The figure uses the verified same-run kernel/Stwo ratio at N=2^20 for:
v0.3, v0.4, v0.5, v0.6, v0.7, v0.8, v0.12, v0.13, v0.18 and v0.20.

The v0.12 build-regime boundary is drawn explicitly. Absolute timing continuity
across that boundary is not claimed.

### `paper/figures/negative-ablations.svg`

Also derived only from `optimization-verified.csv`; it shows the verified
v0.14, v0.15 and v0.17 negative branches.

### `paper/figures/stage-profile-v0.19.svg`

Derived only from:

`paper/evidence/stage-profile-v0.19-logN20.csv`

SHA-256:

`50ff6be15f58946119426b8eade012b456b8e0221cb6549caa99a2a5f5360427`

It visualizes the verified v0.19 stage timings and shares, including the
dominant final two-level scalar tail.
