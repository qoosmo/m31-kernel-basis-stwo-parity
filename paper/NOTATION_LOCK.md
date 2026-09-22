# Notation lock — inherited kernel-polynomial R&D

This file is normative for the M31/Stwo parity manuscript.

## 1. Canonical Boolean notation

Use

\[
N=2^m,\qquad \mathbb B^m=\{0,1\}^m.
\]

For a Boolean exponent vector

\[
a=(a_0,\ldots,a_{m-1})\in\mathbb B^m,
\]

use the established binary-value notation

\[
|a|_2=\sum_{i=0}^{m-1}a_i2^i.
\]

The kernel index is always

\[
y=(y_0,\ldots,y_{m-1})\in\mathbb B^m.
\]

Do **not** introduce a second Boolean basis index `b` for this paper.

## 2. Original Boolean kernel basis

The prior Algorizk kernel-basis paper defines

\[
K_y(X)=\prod_{i=0}^{m-1}(X^{2^i}+y_i).
\]

For

\[
U(X)=\sum_{y\in\mathbb B^m}\lambda_yK_y(X)
    =\sum_{a\in\mathbb B^m}u_aX^{|a|_2},
\]

the established coefficient formula is

\[
u_a=\sum_{y\ge\bar a}\lambda_y.
\]

This is the Boolean zeta transform followed by complement.

For one coordinate,

\[
Z_1=
\begin{pmatrix}
0&1\\
1&1
\end{pmatrix},
\]

so

\[
(\lambda_0,\lambda_1)
\longmapsto
(\lambda_1,\lambda_0+\lambda_1).
\]

Use `Z_1` / Boolean zeta coordinates, not a newly named `T` matrix.

## 3. Canonical affine level sequence

The July 2026 kernel-basis technical specification defines a degree-two level
map

\[
\phi:\mathbb F\to\mathbb F
\]

and

\[
X_0=X,\qquad X_{i+1}=\phi(X_i).
\]

The two named instances are

\[
\phi_{\mathrm{mult}}(X)=X^2,
\qquad
\phi_{\mathrm{aff}}(X)=X^2-2.
\]

The current M31/Stwo paper uses the second instance.

## 4. Affine kernel basis

Keep the existing definition

\[
K_y(X)=\prod_{i=0}^{m-1}(X_i+y_i).
\]

When ambiguity with the original squaring instance matters in prose, call this
the **affine specialization of the kernel basis**. Do not invent a second
symbol unless the final typeset paper truly requires it.

## 5. Recursive branch notation

The canonical recursive split is:

- `A = lambda(0, .)`
- `B = lambda(1, .)`

with associated lower-level kernel polynomials `A(X_1)` and `B(X_1)`.

Then

\[
U(X)
=
X\,A(\phi(X))
+
(X+1)B(\phi(X)).
\]

For the affine paired preimages `+alpha,-alpha` of `beta`:

\[
U(+\alpha)
=
B(\beta)+\alpha(A(\beta)+B(\beta)),
\]

\[
U(-\alpha)
=
B(\beta)-\alpha(A(\beta)+B(\beta)).
\]

## 6. Performance coordinates

For implementation, define

\[
D=B,\qquad C=A+B.
\]

Then

\[
U(X)=D(\phi(X))+XC(\phi(X)),
\]

and

\[
U(+\alpha)=D(\beta)+\alpha C(\beta),
\qquad
U(-\alpha)=D(\beta)-\alpha C(\beta).
\]

Interpretation:

\[
(D,C)=(B,A+B)
\]

is exactly the local Boolean-zeta coordinate map

\[
(\lambda_0,\lambda_1)
\mapsto
(\lambda_1,\lambda_0+\lambda_1).
\]

Therefore the manuscript may use the implementation shorthand
"local zeta coordinates", but must not present this matrix as a newly invented
basis transform.

## 7. Circle relation

The prior affine-domain work already proves, for

\[
\phi_{\mathrm{aff}}(X)=X^2-2,
\]

that under

\[
X=2Z
\]

one obtains

\[
\phi_{\mathrm{aff}}(2Z)=2(2Z^2-1).
\]

Thus the affine domain tower agrees with the Circle-STARK line-domain recursion
up to rescaling.

This is **prior Algorizk R&D** in the present manuscript.

Any new discussion may refine its implementation consequences, but must not
claim the domain conjugacy itself as a result first obtained here.

## 8. Results provenance

### Prior Algorizk results
- Boolean kernel basis.
- basis theorem.
- Boolean zeta/Möbius coefficient transform.
- low-degree filtration theorem.
- general degree-two level-sequence formulation.
- affine Mersenne specialization `phi_aff(X)=X^2-2`.
- recursive evaluation formula.
- paired affine domain construction.
- affine/Circle domain equivalence.
- exact Mersenne affine tower depth result.

### Results of the present paper
- performance-oriented use of local zeta coordinates to expose the 1M+2A
  butterfly in the frozen M31 implementation;
- bit-reversed twiddle-reuse schedule;
- measured optimization/elimination sequence;
- raw redundant M31 direct-NEON implementation;
- true radix-8 register fusion;
- fused final two-level microkernel;
- stable single-thread ARM64 native-transform parity with pinned Stwo SIMD.

No PCS or end-to-end prover result is claimed here.
