# Literature / bibliography plan

Do not write novelty claims before this pass is complete.

## Search cluster A — Stwo and Circle STARKs
Collect:
- Stwo documentation and source;
- Circle STARK papers / technical notes;
- Circle FFT/domain construction references;
- public benchmark or implementation descriptions.

## Search cluster B — Mersenne-31 in proof systems
Collect:
- uses of p = 2^31 - 1 in STARK/proof-system implementations;
- optimized reduction formulas;
- SIMD implementations.

## Search cluster C — Recursive / additive polynomial bases
Collect work on:
- additive FFTs;
- novel polynomial bases for fast evaluation;
- recursive subspace / tower evaluation;
- basis conversions and shears.

The goal is not to force equivalence. The paper should identify precisely which
ideas are related and which are distinct.

## Search cluster D — SIMD finite-field arithmetic
Collect:
- ARM NEON finite-field arithmetic;
- redundant representations;
- delayed/lazy reduction;
- Mersenne-prime SIMD techniques.

## Search cluster E — Hardware transforms
For future-work discussion:
- FPGA FFT/NTT architectures;
- STARK prover acceleration;
- memory-layout and radix-fusion literature.

## Evidence rule

For each related-work claim, record:
- citation;
- exact statement supported;
- whether it concerns mathematics, implementation, or benchmarking;
- whether it predates this work;
- difference from the present transform.

Avoid phrases such as "first", "novel", or "state of the art" until the search
supports them.
