# Sections 6–8 evidence review

Status: empirical core drafted from the frozen experimental lineage.

## Frozen primary evidence

- Stwo SHA: `826591c6c371376810ca8213b5812d5daf6d5092`
- Algorizk tag: `v0.20.1-parity`
- Desk: `LAB-010 / run 14`
- frozen source SHA-256: `a3a5a14353bd038e5a1f2ccf44d42ca880f2f137040a40ff9851479541ba2c7c`
- final stability CSV: `results/stability-summary.csv`

## Final stability values copied directly from the frozen CSV

### logN 18
- median kernel/Stwo: `0.99619100`
- range: `0.99473000 .. 1.00353600`
- median kernel: `1.536916 ms`
- median Stwo: `1.542792 ms`
- within ±5%: `7/7`

### logN 19
- median kernel/Stwo: `0.97449500`
- range: `0.96902700 .. 0.97860300`
- median kernel: `3.218542 ms`
- median Stwo: `3.307583 ms`
- within ±5%: `7/7`

### logN 20
- median kernel/Stwo: `0.97951000`
- range: `0.97663000 .. 0.99614900`
- median kernel: `6.924750 ms`
- median Stwo: `7.058000 ms`
- within ±5%: `7/7`

## Historical benchmark rows

The optimization-history numbers in Section 8 are the recorded same-run values
from the v0.3–v0.20 experimental sequence.

Important publication rule:

**Do not use absolute time differences across the v0.12 build-policy boundary
as a speedup claim.**

The historical table labels:
- Regime A: original build policy;
- Regime B: native CPU + one codegen unit + ThinLTO for both kernel and Stwo.

Same-run ratios remain meaningful within each experiment.

## Negative-result wording constraints

Allowed:
- "the tested PackedM31 implementation was slower on the measured ARM64 host";
- "naive cache blocking worsened performance";
- "stage profiling did not support the transpose/locality hypothesis";
- "the final scalar tail was the dominant measured residual bottleneck."

Not allowed:
- "PackedM31 is bad";
- "cache blocking never works";
- "transpose methods cannot help";
- universal architectural claims beyond the tested implementation.

## Final claim

Use:

> stable native-transform performance parity with pinned Stwo SIMD

Do not use:

> beats Stwo

or:

> 2% faster than Stwo

without a new statistical experiment designed for a speedup claim.

## Remaining metadata before submission

- exact Mac model / chip;
- RAM;
- macOS;
- rustc/cargo versions;
- power/thermal protocol;
- exact final benchmark timestamp.

These are reproducibility metadata, not reasons to reopen transform tuning.
