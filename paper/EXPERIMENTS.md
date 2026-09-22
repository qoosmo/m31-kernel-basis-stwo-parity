# Experimental evidence plan

## Frozen primary result

Pinned Stwo SHA:
`826591c6c371376810ca8213b5812d5daf6d5092`

Canonical Desk:
- LAB-010
- Run 14
- `M31-S05/kernel-basis-transform-stwo-parity-arm64-v0.20.1`

Frozen stability evidence:
`../results/stability-summary.csv`

## Table 1 — Final stability result

Columns:
- log2 N
- N
- median kernel time
- median Stwo SIMD time
- median kernel/Stwo
- min ratio
- max ratio
- rounds within ±5%

Primary rows:
- 18
- 19
- 20

## Table 2 — Optimization trajectory

Reconstruct from archived benchmark outputs / terminal records:

1. initial kernel vs Stwo SIMD;
2. same-field arithmetic correction;
3. in-place implementation;
4. bit-reversed schedule / twiddle reuse;
5. 1M+2A sheared butterfly;
6. scalar true radix-8;
7. packed variants;
8. raw/redundant M31 NEON;
9. fused final two levels;
10. seven-round stability result.

For every row record:
- version;
- N;
- kernel time;
- Stwo same-run time;
- ratio;
- exact change from previous version.

Use same-run ratios whenever available.

## Table 3 — Negative optimization results

Columns:
- hypothesis;
- experiment version;
- target metric;
- observed result;
- decision.

Required rows:
- custom field arithmetic;
- in-place memory;
- generic PackedM31;
- exact mul_twiddle on old packed schedule;
- true packed radix-8;
- cache blocking;
- transpose/locality hypothesis.

## Figure 1 — Recursive transform geometry

Show:
- beta at level i+1;
- paired preimages ±alpha at level i;
- butterfly mapping D,C to two values.

## Figure 2 — Sheared basis butterfly

Compare:
original:
B + alpha(A+B), B - alpha(A+B)

with sheared:
D + alpha C, D - alpha C.

Annotate 1M+3A → 1M+2A.

## Figure 3 — Optimization waterfall

Plot same-run kernel/Stwo ratio across the major implementation milestones.

Do not mix ratios produced under different build policies without labeling them.

## Figure 4 — v0.19 stage profile

At N=2^20:
- six three-level radix-8 groups;
- final two-level tail.

Purpose:
show that the high groups were balanced while the scalar tail dominated.

## Figure 5 — Stability distribution

Seven independent T2/STWO observations for N=2^20.

Show:
- parity line at 1.0;
- ±5% band;
- median.

## Missing metadata to collect before submission

- exact Mac model;
- exact Apple chip;
- physical/performance core count;
- RAM;
- macOS version;
- rustc version;
- cargo version;
- clang/LLVM details if materially relevant;
- whether power mode / thermal conditions were controlled;
- exact timestamp/date of final frozen run.

These are metadata tasks, not new transform tuning.

## Optional validation experiments

Only if needed for paper quality, not to reopen optimization:
- repeat on a second Apple ARM64 machine;
- repeat after clean reboot;
- x86_64 comparison as a new portability result.

Do not delay the first manuscript draft for optional experiments.
