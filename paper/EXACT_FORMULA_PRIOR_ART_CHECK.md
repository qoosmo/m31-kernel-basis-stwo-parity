# Exact-formula prior-art check — 2026-09-01

## Scope

This was a final narrow search for external prior art using the exact or close
forms

\[
K_y(X)=\prod_i(X^{2^i}+y_i)
\]

and

\[
K_y(X)=\prod_i(X_i+y_i),
\qquad
X_{i+1}=\phi(X_i),
\]

including searches combined with `polynomial basis`, `kernel basis`,
`Boolean zeta`, `Circle STARK`, and `X^2-2`.

## Result

No external source located in this search used the exact kernel-basis
construction above in the sense developed by the prior Algorizk work.

Search results containing the words "polynomial kernel" were predominantly
unrelated machine-learning or polynomial-matrix kernel material. Circle-FFT
sources describe the recursive Circle line map and coordinate basis, but did
not surface the exact Boolean kernel-product formula.

## Claim consequence

A negative search result does not prove universal novelty.

For the present M31/Stwo parity paper the conservative treatment is:

- cite the Boolean kernel basis and affine kernel/domain results as **prior
  Algorizk R&D**;
- do not claim that the present paper newly invents the kernel basis;
- do not use "first", "fastest", or "state of the art";
- make the present contribution the source-frozen Rust/ARM64 realization,
  verified optimization/ablation record, and stable native-transform parity
  with pinned Stwo.

This preserves the novelty of the earlier kernel-basis program without
misattributing it to the current systems paper.
