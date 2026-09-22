# Paper architecture

## Working title

**Fast Kernel-Basis Evaluation over Mersenne-31: Stable ARM64 Performance Parity with Stwo's Circle FFT**

Shorter internal title:

**M31 Kernel-Basis Transform — Stable ARM64 Parity with Stwo**

The working title is intentionally precise:
- the object implemented by Algorizk is a kernel-basis evaluation transform;
- the comparison baseline is Stwo's production SIMD Circle FFT;
- the measured parity claim is restricted to the recorded single-thread ARM64 setting.

## Central research question

Can the recursive Mersenne-31 kernel-basis evaluation transform be organized and
implemented so that its native transform cost is competitive with a highly
optimized production Circle FFT implementation?

## Core answer

Yes, under the recorded single-thread Apple ARM64 benchmark. The final native
kernel-basis transform reaches stable performance parity with the pinned Stwo
SIMD baseline through N = 2^20.

At N = 2^20, the frozen seven-round stability gate records:
- median kernel / Stwo ratio: 0.97951000;
- observed range: 0.97663000 .. 0.99614900;
- median kernel transform: 6.924750 ms;
- median Stwo SIMD transform: 7.058000 ms;
- seven of seven rounds inside the ±5% parity band.

The supported scientific statement is **stable performance parity**. The paper
must not turn the nominal median advantage into a material speedup claim.

---

# Proposed manuscript

## 1. Introduction

### 1.1 Motivation
- Mersenne-31 arithmetic is important in modern proof-system implementations.
- Stwo provides a strong production reference point for Circle-domain transforms.
- Alternative recursive polynomial bases are interesting only if their
  mathematical structure can survive contact with production-level performance.

### 1.2 Problem
Present the kernel-basis transform and the engineering question:
can its recursive evaluation structure be implemented at Stwo-class speed?

### 1.3 Contributions
State only evidence-backed contributions:

1. A recursive kernel-basis evaluation transform over M31 built from the affine
   map phi(X) = X^2 - 2 and paired preimages ±alpha.
2. A sheared recursive basis yielding a native butterfly with one field
   multiplication and two additions/subtractions.
3. A bit-reversed execution schedule that exposes twiddle reuse and removes the
   dominant scheduling penalty of the initial implementation.
4. An ARM64 implementation based on raw redundant M31 values in [0,P], direct
   NEON arithmetic, true radix-8 register fusion, and a fused final two-level
   microkernel.
5. A reproducible comparison against pinned Stwo commit
   826591c6c371376810ca8213b5812d5daf6d5092.
6. A seven-round stability study showing native-transform performance parity
   through N = 2^20.
7. An experimental record of optimizations that failed, including generic
   PackedM31 vectorization and cache/transpose hypotheses, clarifying which
   architectural choices actually matter.

### 1.4 Non-claims
Explicitly state:
- no PCS result;
- no end-to-end prover result;
- no claim that the two transforms use literally identical domains or bases;
- no cross-architecture parity claim;
- no parity claim including canonical kernel-basis → sheared-basis conversion.

## 2. Mathematical setting

### 2.1 The field M31
Define p = 2^31 - 1 and the Mersenne reduction identity.

### 2.2 Affine recursion
Define:
X_0 = X,
X_{i+1} = phi(X_i),
phi(X) = X^2 - 2.

Explain the paired-preimage structure:
if beta is at the next level, preimages satisfy alpha^2 = beta + 2 and are
±alpha.

### 2.3 Kernel basis
Define:
K_y(X) = product_i (X_i + y_i),
for y in {0,1}^m.

Explain the recursive coefficient split used by the original basis.

### 2.4 Relation to Circle doubling
Under X = 2Z:
phi(2Z) = 2(2Z^2 - 1).

Use this only to explain related recursive geometry. Do not state that the
kernel domains and Stwo Circle domains are identical.

## 3. From the original recursion to a 1M+2A butterfly

### 3.1 Original split
Write:
U = X A(phi) + (X+1) B(phi).

For paired evaluation points ±alpha:
w_+ = B + alpha(A+B),
w_- = B - alpha(A+B).

This costs one multiplication and three additions/subtractions if implemented
directly.

### 3.2 Sheared recursive basis
Define:
D = B,
C = A+B,
so:
U = D(phi) + X C(phi).

Then:
U(+alpha) = D + alpha C,
U(-alpha) = D - alpha C.

The native butterfly is therefore:
t = alpha C,
w_+ = D+t,
w_- = D-t,

for one multiplication and two additions/subtractions.

### 3.3 Basis-conversion boundary
Explain carefully that this is a recursive basis shear, not a free local
substitution.

The canonical kernel-basis to sheared-basis conversion is O(N log N) additions
and is excluded from the native-transform benchmark.

This limitation must be repeated in the benchmark methodology and conclusion.

## 4. Transform schedule

### 4.1 Natural schedule
Describe the first implementation and why its twiddle access pattern prevented
efficient reuse.

### 4.2 Bit-reversed conjugation
Derive the bit-reversed execution order and show that one twiddle is reused
across a contiguous butterfly block.

### 4.3 Complexity
State:
- N/2 butterflies per level;
- log2 N levels;
- native transform arithmetic count:
  (N/2) log N multiplications and
  N log N additions/subtractions
  after the 1M+2A shear.

Keep operation counts separate from memory and implementation effects.

## 5. Implementation evolution

This section should be quantitative, not anecdotal.

### 5.1 Baseline scalar implementation
Initial same-field implementation and gap to Stwo.

### 5.2 In-place execution
Show that eliminating ping-pong memory was useful but not decisive.

### 5.3 Twiddle-reuse schedule
Show the major gain from bit-reversed scheduling.

### 5.4 Sheared 1M+2A butterfly
Show near scalar-CPU parity.

### 5.5 True register fusion
Distinguish real radix fusion from earlier loop grouping.

### 5.6 Raw redundant M31 representation
Explain values in [0,P], P as an alternate zero representation, and delayed
canonicalization.

### 5.7 Direct ARM64 NEON
Describe:
- four u32 lanes per NEON register;
- widening products;
- Mersenne folding;
- redundant add/sub;
- true radix-8 register fusion.

### 5.8 Fused final two-level microkernel
Explain why k=1 and k=0 became the dominant residual bottleneck and how one
four-element load/store block eliminates the scalar tail.

## 6. Negative results and eliminated hypotheses

This is a first-class section.

### 6.1 Custom scalar arithmetic
Only a modest fraction of the original gap.

### 6.2 Generic PackedM31
Packed vectorization was slower than the scalar winner at the target size.

### 6.3 Exact Stwo twiddle multiplier on the wrong schedule
No benefit by itself.

### 6.4 True packed radix-8
Still slower than scalar at N=2^20.

### 6.5 Cache blocking
Naive subtree blocking worsened performance.

### 6.6 Transpose/locality hypothesis
Stage profiling showed the radix-8 groups were balanced, directly rejecting
the hypothesis that the remaining gap came from high-level locality imbalance.

### 6.7 Actual residual bottleneck
The final two scalar levels consumed roughly 38.5% of the v0.18/v0.19
transform at N=2^20. Fusing/vectorizing them closed the gap.

## 7. Experimental methodology

### 7.1 Hardware/software environment
Record exact Apple ARM64 machine details in the final paper from the benchmark
host before submission.

### 7.2 Pinned Stwo baseline
Commit:
826591c6c371376810ca8213b5812d5daf6d5092.

### 7.3 Build settings
- target-cpu=native;
- release;
- codegen-units=1;
- ThinLTO;
- RAYON_NUM_THREADS=1.

### 7.4 Correctness gate
Explain the small-N exact comparisons against the scalar/original transform and
brute-force evaluation where available.

### 7.5 Timing protocol
Separate:
- preparation not timed;
- transform timing;
- input preservation/copy policy;
- canonicalization boundary;
- stability rounds.

### 7.6 Fairness
State that each algorithm is timed in its native coefficient representation.
Do not imply identical bases or domains.

## 8. Results

### 8.1 Optimization trajectory
Use one table showing the major milestones from the initial ~4.2x gap to parity.

### 8.2 Final large-N comparison
Primary table:
N = 2^18, 2^19, 2^20.

Report kernel time, Stwo time, and kernel/Stwo ratio.

### 8.3 Stability
Seven independent rounds.
Emphasize the median and observed range.

### 8.4 Bottleneck decomposition
Include the stage-profile evidence showing balanced radix-8 groups and the
dominant scalar tail before the final optimization.

## 9. Discussion

### 9.1 What produced parity
Argue from measurements:
- algebraic butterfly simplification;
- schedule/twiddle reuse;
- register fusion;
- representation;
- architecture-specific SIMD;
- low-level tail specialization.

### 9.2 What did not produce parity
Connect the negative results to general implementation lessons.

### 9.3 Meaning of comparison with Stwo
The result shows that this distinct native kernel-basis transform can achieve
Stwo-class transform throughput on the measured ARM64 platform.

It does not show mathematical equivalence between the transforms.

### 9.4 Cost outside the benchmark
Discuss the omitted basis conversion and why end-to-end integration is required
before any prover-level performance conclusion.

## 10. Related work

Research and cite, at minimum:
- Stwo / Circle STARK transform implementation;
- Circle STARK / Circle FFT literature;
- Mersenne-31 arithmetic and proof-system use;
- additive/recursive polynomial bases and fast evaluation where relevant;
- SIMD finite-field arithmetic relevant to the implementation.

Do not draft this section from memory. It requires a dedicated literature pass.

## 11. Future work

Summarize the ranked future-work program from `FUTURE_WORK.md`.

## 12. Conclusion

End with the narrow result:

A recursively defined M31 kernel-basis transform, after algebraic and
architecture-aware optimization, reaches stable native-transform parity with
the pinned Stwo SIMD Circle FFT on the recorded single-thread ARM64 platform
through N = 2^20.

Repeat the basis-conversion and end-to-end limitations in one sentence.

## Appendices

### A. Correctness derivation
Detailed recursive proof and basis shear.

### B. Domain construction
Affine tower and square-root lifting details.

### C. ARM64 M31 arithmetic
Redundant representation invariants and NEON formulas.

### D. Benchmark history
Complete v0.3–v0.20 optimization ledger.

### E. Reproducibility
Repository, tag, pinned Stwo SHA, commands, and hashes.
