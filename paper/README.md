# Paper workspace

The experimental transform result is frozen before manuscript drafting.

## Files

- `OUTLINE.md` — canonical manuscript architecture.
- `CLAIMS.md` — claim firewall: supported, unsupported, and literature-dependent claims.
- `EXPERIMENTS.md` — tables, figures, benchmark methodology, missing metadata.
- `FUTURE_WORK.md` — ranked future-work program.
- `BIBLIOGRAPHY_PLAN.md` — literature-search plan before novelty claims.
- `main.md` — manuscript skeleton.

## Drafting order

1. mathematical setting;
2. sheared 1M+2A butterfly;
3. transform schedule;
4. ARM64 implementation;
5. experimental methodology;
6. results;
7. negative results / discussion;
8. dedicated literature pass;
9. related work;
10. future work;
11. introduction;
12. abstract and conclusion last.

## Claim rule

`CLAIMS.md` is authoritative. Do not strengthen claims in prose without first
updating the evidence ledger.

## Frozen benchmark boundary

The headline result is native-transform parity with pinned Stwo on the recorded
single-thread ARM64 environment. The canonical kernel-basis → sheared-basis
conversion is outside the timed transform.

No PCS or end-to-end prover claim is part of this paper.
