# Verified optimization trajectory at N=2^20

| Version | Milestone | Kernel ms | Stwo SIMD ms | K/Stwo | Build regime |
|---|---|---:|---:|---:|---|
| v0.3 | initial recursive kernel | 44.798 | 10.605 | 4.224 | A |
| v0.4 | same Stwo BaseField arithmetic | 36.409 | 10.747 | 3.388 | A |
| v0.5 | in-place domain order | 33.755 | 10.568 | 3.194 | A |
| v0.6 | bit-reversed twiddle-reuse schedule | 15.462 | 10.579 | 1.462 | A |
| v0.7 | Boolean-zeta 1M+2A coordinates | 13.326 | 10.715 | 1.244 | A |
| v0.8 | three-level loop grouping | 12.732 | 10.846 | 1.174 | A |
| v0.12 | native codegen / scalar sweep | 10.029 | 6.963 | 1.440 | B |
| v0.13 | true scalar radix-8 register fusion | 9.450 | 7.970 | 1.186 | B |
| v0.18 | raw redundant M31 direct NEON | 9.141 | 7.060 | 1.295 | B |
| v0.20 | fused final two-level NEON tail | 6.917 | 6.921 | 0.999 | B |

**Build-regime boundary:** v0.12 begins regime B, where both kernel and Stwo
were built with native CPU targeting, one codegen unit and ThinLTO. Absolute
times should not be interpreted as one continuous series across this boundary;
same-run ratios remain the primary comparison.
