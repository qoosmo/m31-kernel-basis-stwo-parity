# Mathematics review checklist — Sections 2–4

Status: first rigorous draft committed before literature claims.

## Established in the draft

- Definition of the affine map `phi(X)=X^2-2`.
- Definition of a split phi-tower without overclaiming existence to arbitrary depth.
- Recursive definition of the kernel basis.
- Inductive proof that the kernel family spans and is linearly independent.
- Unique sheared decomposition:
  `U(X)=D(phi(X))+X C(phi(X))`.
- Derivation of the paired 1M+2A butterfly.
- Recursive correctness proposition.
- Exact native arithmetic counts:
  - `(N/2) log2 N` multiplications;
  - `N log2 N` additions/subtractions.
- Recursive shear cost:
  - `(N/2) log2 N` additions.
- Explicit statement that shear conversion is excluded from transform timing.
- Bit-reversal definition and stage indexing.
- Proof sketch that the scheduled transform is a permutation-conjugated form of the same recursion.
- Twiddle reuse count per stage.
- Scaled relation `phi(2Z)=2(2Z^2-1)` to Circle x-coordinate doubling without identifying the bases/domains.

## Must be cross-checked against source before final submission

1. **Exact domain ordering notation.**
   The mathematics is ordering-independent; the final reproducibility appendix
   should match the implementation's concrete ordering names exactly.

2. **Maximum constructed tower depth.**
   Do not state a maximal M31 split-tower theorem unless separately proved.
   The current text correctly limits itself to explicitly constructed/tested sizes.

3. **Shear implementation count.**
   The recurrence gives `(N/2) log2 N` additions for the recursive algebraic shear.
   Verify the production helper performs exactly this transform before calling the
   number an implementation measurement.

4. **Circle terminology.**
   The relation to the Circle doubling map must be literature-cited in the final
   paper. Current text states only the algebraic scaling identity.

5. **Basis novelty.**
   No novelty claim is made. Prior-art search remains mandatory.

## Do not change without new evidence

- native-transform benchmark boundary;
- no PCS claim;
- no end-to-end prover claim;
- no claim that kernel and Stwo bases/domains are identical;
- stable parity rather than material speedup.
