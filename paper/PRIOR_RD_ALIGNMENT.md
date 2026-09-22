# Prior R&D alignment audit

## Canonical sources

### A. Boolean kernel basis paper

**Ali Mkhida, _The Boolean Kernel Basis and Its Low-Degree Filtration over
Arbitrary Fields_, Algorizk Labs, 2026.**

Canonical public research repository:

`https://github.com/qoosmo/kernel-basis-filtration`

Key inherited notation/results:
- `N=2^m`;
- `B^m={0,1}^m`;
- `|a|_2=sum_i a_i 2^i`;
- `K_y(X)=prod_i(X^(2^i)+y_i)`;
- `U=sum_y lambda_y K_y=sum_a u_a X^{|a|_2}`;
- `u_a=sum_{y >= complement(a)} lambda_y`;
- Boolean zeta/Möbius transform;
- local zeta matrix `Z_1=[[0,1],[1,1]]`;
- low-degree filtration in kernel coordinates.

### B. Kernel-basis multilinear PCS technical specification

**Ali Mkhida, _A Kernel-Basis Multilinear Polynomial Commitment Scheme:
Technical Specification_, Algorizk Labs, July 11, 2026.**

The current paper uses only its algebra/domain background, not its PCS claim.

Key inherited notation/results:
- level sequence `X_0=X`, `X_{i+1}=phi(X_i)`;
- `phi_mult(X)=X^2`;
- `phi_aff(X)=X^2-2`;
- `K_y(X)=prod_i(X_i+y_i)`;
- recursive structure `K_y(X)=(X_0+y_0)K_ybar(X_1)`;
- recursive evaluation
  `U(X)=X A(phi(X))+(X+1)B(phi(X))`;
- paired evaluation-domain theorem;
- affine Mersenne tower;
- rescaling `X=2Z` to the Circle line-doubling map;
- exact tower-depth theorem for Mersenne-type primes.

## Correction to the current manuscript

Earlier drafts of the M31/Stwo parity paper used the terms:
- "Boolean shear";
- matrix `T=[[0,1],[1,1]]`;
- new Boolean index `b`;
- "fully sheared basis" `S_b`.

These are withdrawn as primary mathematical notation.

The matrix was already the one-coordinate Boolean zeta transform in the
canonical kernel-basis work. In the affine recursion,

\[
D=B,\qquad C=A+B
\]

is therefore an implementation-level reuse of the prior zeta coordinates.

The manuscript should describe the transform as the **affine kernel-basis
transform evaluated in Boolean-zeta / native coordinates**, not as a new
independently defined polynomial basis.

## Novelty boundary

The performance result remains new within this project.

The following are not claimed as new here:
- the kernel basis itself;
- the zeta/Möbius transform;
- the affine level-sequence formalism;
- the map `X^2-2`;
- the Mersenne/Circle domain correspondence.

The paper's primary contribution is the performance realization and the
evidence explaining how that prior algebra reaches Stwo-class throughput.
