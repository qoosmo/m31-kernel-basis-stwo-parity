# Claims ledger

This file is the manuscript claim firewall. A statement may enter the abstract,
introduction, result section, or conclusion only if it is listed as SUPPORTED
or is later upgraded with new evidence.

## Supported now

### C1 — Correctness of the optimized transform on the tested correctness gate
The optimized raw-NEON/fused-tail implementation matches the scalar/original
kernel transform on the benchmark's exact small-N correctness gate.

Status: SUPPORTED.

### C2 — Native transform complexity
After the recursive basis shear, the transform uses one multiplication and two
additions/subtractions per butterfly, with N/2 butterflies per level and
log2(N) levels.

Status: SUPPORTED by derivation and implementation structure.

### C3 — Stable ARM64 native-transform parity with pinned Stwo
On the recorded single-thread Apple ARM64 configuration and build policy, the
final native kernel-basis transform has stable performance parity with the
pinned Stwo production SIMD transform through N=2^20.

At N=2^20:
- seven-round median kernel/Stwo = 0.97951000;
- range = 0.97663000 .. 0.99614900;
- seven of seven rounds inside ±5%.

Status: SUPPORTED.

Preferred wording:
"stable performance parity" or "Stwo-class native-transform throughput."

Avoid:
"faster than Stwo", "beats Stwo", "2% faster", unless a future statistical
study explicitly supports such a stronger statement.

### C4 — The final two scalar levels were a dominant residual bottleneck
Before the fused tail, stage profiling showed balanced high radix-8 groups and
a disproportionately expensive final two-level scalar tail.

Status: SUPPORTED by v0.19/v0.20 experiment sequence.

### C5 — Generic PackedM31 was not sufficient for this kernel
At the target size, the tested generic PackedM31 implementations remained slower
than the scalar winner.

Status: SUPPORTED for the tested implementations and machine.

Do not generalize to:
"PackedM31 is always slower" or "portable SIMD cannot work."

### C6 — Naive cache blocking / transpose hypothesis was not supported
The tested cache blocking worsened performance, and the stage profiler showed
balanced radix-8 groups rather than a high-level locality cliff.

Status: SUPPORTED for the tested transform/schedule.

## Explicitly unsupported now

### U1 — End-to-end prover parity
UNSUPPORTED.

### U2 — PCS performance or PCS advantage
UNSUPPORTED and outside current paper scope.

### U3 — Parity including canonical kernel-basis → sheared-basis conversion
UNSUPPORTED. Conversion is excluded from native-transform timing.

### U4 — Cross-platform parity
UNSUPPORTED until x86_64 / other ARM / GPU measurements exist.

### U5 — FPGA speedup or FPGA parity
UNSUPPORTED until an RTL/FPGA implementation is measured.

### U6 — Mathematical identity with Stwo Circle FFT
FALSE framing. The recursive geometries are related, but the native bases and
domains are not literally the same.

### U7 — Material speedup over Stwo
UNSUPPORTED by the current stability experiment. The result is parity.

## Claims requiring literature before publication

### L1 — Novelty of the specific kernel basis
Needs prior-art search.

### L2 — Novelty of the sheared recursive basis formulation
Needs prior-art search.

### L3 — Novelty of the ARM64 redundant-M31 microkernel
Needs comparison with Stwo and other public M31 implementations.

### L4 — Novelty of combining the mathematical transform and implementation path
Needs full related-work analysis.

Until that search is complete, use "we present" rather than "we introduce the
first" or "novel."

## Literature-pass update — 2026-09-01

The dedicated related-work search changes the interpretation of L1–L4.

### L1 — Exact kernel-basis formula

Searches did not locate the exact basis

`K_y(X)=prod_i(phi^i(X)+y_i)`

in the reviewed Circle FFT, ECFFT, G-FFT, additive-FFT, and alternative-basis
sources.

Status: **POTENTIALLY DISTINCTIVE, NOT YET A FIRST/NEWNESS CLAIM**.

A negative search result is insufficient to establish novelty.

### L2 — Sheared recursive basis

The fully recursive shear exposes the coordinate basis

`S_b(X)=prod_i X_i^{b_i}`.

Under `X=2Z`, this becomes diagonally scaled from

`prod_i pi^i(Z)^{b_i}`,

which is the univariate recursive coordinate-product pattern appearing in the
Circle FFT line recursion.

Status: **SUPPORTED STRUCTURAL BRIDGE; WEAK AS A STANDALONE NOVELTY CLAIM**.

The paper should emphasize the bridge rather than pretending the sheared basis
is unrelated to Circle FFT.

### L3 — ARM64 redundant M31 microkernel

M31 SIMD, NEON, Mersenne reduction, and delayed/redundant arithmetic have prior
software art in Stwo and Plonky3.

Status: **NOT NOVEL IN ISOLATION**.

Potential contribution is the transform-specific integration, profiling and
measured elimination of inferior implementation paths.

### L4 — Combined mathematical/engineering result

The literature search did not identify a prior study beginning from this exact
kernel basis and documenting the path to stable native-transform parity with
Stwo.

Status: **STRONGEST CURRENT CONTRIBUTION CANDIDATE**, pending final targeted
prior-art search.

Preferred contribution language:
- "we study";
- "we show";
- "we identify";
- "we demonstrate stable parity".

Still prohibited without stronger evidence:
- "first";
- "novel FFT";
- "fastest";
- "state of the art".


## Prior-R&D reconciliation — 2026-09-01

This section supersedes any wording in the earlier claim ledger that treated
the kernel basis, affine recursion, or local `[[0,1],[1,1]]` transform as newly
introduced by the M31/Stwo parity paper.

### P1 — Boolean kernel basis

Prior Algorizk result:

\[
K_y(X)=\prod_i(X^{2^i}+y_i).
\]

Prior paper:
*The Boolean Kernel Basis and Its Low-Degree Filtration over Arbitrary Fields*.

Status in this paper: **BACKGROUND / INHERITED RESULT**.

### P2 — Boolean zeta/Möbius transform

Prior Algorizk result:

\[
u_a=\sum_{y\ge\bar a}\lambda_y.
\]

Local zeta map:

\[
Z_1=
\begin{pmatrix}
0&1\\
1&1
\end{pmatrix}.
\]

Status in this paper: **BACKGROUND / INHERITED RESULT**.

The performance coordinates

\[
(D,C)=(B,A+B)
\]

are an implementation reuse of this local map.

### P3 — affine level sequence and M31 specialization

Prior Algorizk technical result:

\[
X_0=X,\qquad X_{i+1}=\phi(X_i),
\]

with

\[
\phi_{\rm aff}(X)=X^2-2,
\]

and

\[
K_y(X)=\prod_i(X_i+y_i).
\]

Status in this paper: **BACKGROUND / INHERITED RESULT**.

### P4 — recursive affine evaluation

Prior Algorizk technical result:

\[
U(X)=X A(\phi(X))+(X+1)B(\phi(X)).
\]

Status in this paper: **BACKGROUND / INHERITED RESULT**.

### P5 — affine/Circle domain equivalence

Prior Algorizk technical result under `X=2Z`.

Status in this paper: **BACKGROUND / INHERITED RESULT**.

Do not claim the conjugacy/domain correspondence as first obtained here.

### N1 — present-paper contribution

Current strongest contribution:

**A source-frozen Rust/ARM64 realization of the inherited affine kernel
transform, using its Boolean-zeta/native coordinates, with measured
optimization evidence culminating in stable single-thread parity with pinned
Stwo SIMD through `N=2^20`.**

Status: **SUPPORTED**.

### N2 — negative optimization record

The elimination sequence (generic packed SIMD, exact twiddle multiplier on the
wrong schedule, cache blocking, rejected transpose hypothesis, scalar-tail
diagnosis) is part of the present empirical contribution.

Status: **SUPPORTED**.

### N3 — no PCS claim

The earlier PCS technical specification is provenance for algebra/domain
results only. The present manuscript does not claim a PCS or end-to-end prover
result.

Status: **HARD GUARDRAIL**.
