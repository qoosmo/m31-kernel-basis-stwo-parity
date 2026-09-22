# Future-work program

Future work is ranked by scientific value and by how directly it addresses the
limitations of the current result.

## Priority 1 — End-to-end basis-boundary cost

### Question
What happens when the canonical kernel-basis → sheared-basis conversion is
included in an actual pipeline?

### Work
- benchmark the O(N log N) shear explicitly;
- determine whether input data can be generated directly in the sheared basis;
- search for fusion of conversion with upstream/downstream stages;
- quantify break-even conditions.

### Why it matters
This is the largest explicit boundary in the current parity claim.

## Priority 2 — Cross-architecture portability

### Targets
- x86_64 AVX2;
- x86_64 AVX-512 where available;
- additional ARM64 systems.

### Question
Is parity a structural property of the transform or an Apple-ARM64-specific
implementation result?

### Deliverable
Same correctness and stability protocol with architecture-specific raw M31
microkernels.

## Priority 3 — FPGA realization

### Goal
Translate the proven transform structure, not the abandoned software paths.

### Candidate architecture
- sheared 1M+2A butterfly;
- twiddle-reuse / bit-reversed schedule;
- radix fusion adapted to BRAM/register constraints;
- Mersenne-31 multiplier/reduction;
- streaming treatment of the final low levels.

### Measurements
- throughput;
- latency;
- Fmax;
- LUT;
- FF;
- DSP;
- BRAM/URAM;
- area-time / energy where measurable.

No FPGA performance claim belongs in the present paper until these measurements
exist.

## Priority 4 — Extension-field compatibility

Determine how the recursive transform interacts with M31 extension elements:
- representation;
- multiplication cost;
- twiddle storage;
- SIMD/RTL mapping;
- whether the same shear remains advantageous.

This connects naturally to M31-S06.

## Priority 5 — Integration with Stwo-relevant workloads

Use M31-S07 to identify where a kernel-basis transform could be used in a real
workload without forcing unnecessary basis conversion.

Questions:
- which prover data naturally admits the kernel/sheared representation?
- can transformations be fused with trace or polynomial preparation?
- which stages dominate once the transform itself is no longer slower?

This is an integration study, not a PCS claim.

## Priority 6 — Formalization

Formalize:
- recursive domain construction;
- basis shear equivalence;
- transform correctness;
- redundant M31 arithmetic invariants.

Lean is a natural candidate for the mathematical statements, while Rust remains
the canonical executable implementation.

## Priority 7 — Parallel scaling

The current headline result is intentionally single-threaded.

Future evaluation:
- thread-level scaling;
- memory-bandwidth saturation;
- NUMA effects on larger systems;
- comparison with Stwo parallel execution under matched thread counts.

## Priority 8 — Larger transforms

M31 recursive depth allows larger sizes than currently benchmarked, subject to
memory and domain construction.

Measure beyond 2^20 only after:
- reproducible host metadata is frozen;
- allocation and preparation costs are clearly separated;
- comparison remains fair.

## Stop rule

Do not reopen low-level ARM64 transform tuning merely to chase a few percentage
points. The current result has passed the parity stability gate.

New engineering work must address a new scientific question:
integration, portability, hardware, formalization, or scaling.
