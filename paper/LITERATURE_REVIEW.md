# Literature review — 2026-09-01

This review is a claim-control document for the M31 kernel-basis parity paper.
It records what the literature supports and what it prevents us from claiming.

## 1. Circle domains and M31

### HLN23 — Reed-Solomon Codes over the Circle Group

Ulrich Haböck, Daniel Lubarov, Jacqueline Nabaglo.
IACR ePrint 2023/824.

URL: https://eprint.iacr.org/2023/824

What it establishes:
- Reed-Solomon-style domains can be built on the unit circle over a complex
  extension of a Mersenne prime field.
- Mersenne arithmetic is a central motivation because of machine-friendly
  reduction.
- Circle-group structure provides an FFT route despite poor two-adicity of
  `p-1`.

Impact on our paper:
- We cannot present the use of M31 with circle-like recursive geometry as new.
- This is a foundational predecessor for the M31/Circle direction.

## 2. Circle STARK / Circle FFT

### HLP24 — Circle STARKs

Ulrich Haböck, David Levit, Shahar Papini.
IACR ePrint 2024/278.

URL: https://eprint.iacr.org/2024/278

What it establishes:
- Circle STARKs use the circle group when `p+1` has large two-adicity.
- For M31, `p+1 = 2^31`.
- The Circle FFT first projects `(x,y)` to the x-axis, then recurses using
  `pi(x)=2x^2-1`.
- The Circle FFT coefficient basis is
  `y^{i0} x^{i1} pi(x)^{i2} ... pi^{n-2}(x)^{i_{n-1}}`.
- With precomputed twiddles, the transform has the classical ideal count:
  `(N/2) log2 N` multiplications and `N log2 N` additions.

Impact on our paper:
- Our final 1M+2A arithmetic count is not unique to our transform.
- It is meaningful that the kernel transform reaches this ideal count.
- The correct comparison is structural and implementation-level, not a claim
  that our basis and the Circle basis are identical.

## 3. S-two as the production reference

### CGHLLPS26 — S-two Whitepaper

Dan Carmon, Lior Goldberg, Ulrich Haböck, Leonardo Lerer, Ilya Lesokhin,
Shahar Papini, Shahar Samocha.
IACR ePrint 2026/532.

URL: https://eprint.iacr.org/2026/532

What it establishes:
- S-two is a current Circle-STARK system over M31.
- Witness polynomials are represented in Circle-FFT spaces.
- The whitepaper gives the Circle FFT basis and confirms the
  `(N/2) log N` multiplication / `N log N` addition count.
- The Stwo codebase is therefore an appropriate production-quality comparison
  target for a transform microbenchmark.

Impact on our paper:
- The baseline is scientifically relevant.
- We should call it a production implementation baseline, while carefully
  pinning the exact source revision used in our benchmark.

## 4. FFTs for q+1-smooth fields beyond Circle STARKs

### LiX24 — Fast Fourier transform via automorphism groups of rational function fields

Songsong Li, Chaoping Xing.
SODA 2024, pp. 3836–3859.
DOI: 10.1137/1.9781611977912.135
arXiv:2310.14462.

What it establishes:
- A general algebraic FFT framework includes multiplicative, additive, and
  `q+1`-smooth cases.
- The `q+1`-smooth case has `O(B n log n)` arithmetic complexity for
  B-smooth transform sizes.

Impact on our paper:
- We cannot frame `q+1`-smooth recursive FFT structure as unique to Circle
  STARKs or to our work.
- Our paper is a concrete basis/implementation study rather than a new general
  finite-field FFT framework.

### Hab24 — A note on the G-FFT

Ulrich Haböck.
IACR ePrint 2024/1036.

URL: https://eprint.iacr.org/2024/1036

What it establishes:
- It compares Li–Xing's G-FFT with Circle FFT.
- It emphasizes that twiddle parity/alternation determines concrete butterfly
  cost.
- It states the Circle FFT cost as:
  `m 2^{m-1} M + m 2^m A`.
- It explains the Circle basis as a recursive product of coordinate functions.

Impact on our paper:
- This is directly relevant to our 1M+2A discussion.
- The key point is not merely asymptotic complexity but selecting recursive
  coordinates/basis functions that give one twiddle multiplication per
  butterfly.

## 5. ECFFT

### BCKL21 — ECFFT Part I

Eli Ben-Sasson, Dan Carmon, Swastik Kopparty, David Levit.
arXiv:2107.08473.

What it establishes:
- Fast polynomial algorithms can be built over fields lacking large smooth
  multiplicative subgroups by using elliptic-curve structure.
- Recursive algebraic maps and evaluation sets can replace roots of unity.

### BCKL22 — ECFFT Part II

Eli Ben-Sasson, Dan Carmon, Swastik Kopparty, David Levit.
TCC 2022 / LNCS 13747, pp. 467–496.
DOI: 10.1007/978-3-031-22318-1_17.

Impact on our paper:
- ECFFT is part of the broader history leading to Circle FFT.
- Our transform should be positioned as another concrete recursive-basis
  realization, not as the first FFT without roots of unity.

## 6. Additive FFT and non-standard polynomial bases

### GaoMateer10

Shuhong Gao, Todd Mateer.
"Additive Fast Fourier Transforms Over Finite Fields."
IEEE Transactions on Information Theory 56(12), 6265–6272, 2010.
DOI: 10.1109/TIT.2010.2079016.

What it establishes:
- Recursive/additive FFT algorithms over finite fields predate the current ZK
  ecosystem.

### LinAHC16

Sian-Jheng Lin, Tareq Y. Al-Naffouri, Yunghsiang S. Han, Wei-Ho Chung.
"Novel Polynomial Basis With Fast Fourier Transform and Its Application to
Reed-Solomon Erasure Codes."
IEEE Transactions on Information Theory 62(11), 6284–6299, 2016.
DOI: 10.1109/TIT.2016.2608892.

What it establishes:
- A deliberately non-standard polynomial basis can give `O(n log n)` FFT-like
  evaluation with small constants.
- Basis conversion is a first-class part of the design.

Impact on our paper:
- We must not claim that using a non-standard recursive basis for fast
  evaluation is itself novel.
- Our basis conversion boundary should be compared conceptually with this line
  of work.

### SamantaBadakhshanGong26

Susanta Samanta, Mohammadtaghi Badakhshan, Guang Gong.
"On the Additive FFT Techniques over Binary Extension Fields."
arXiv:2608.20855, August 2026.

What it establishes:
- A current general-basis additive FFT framework.
- Recursive structures can improve locality.
- In the specialized Cantor-basis setting, the authors obtain
  `(n/2) log2 n` multiplications for one algorithm.
- They explicitly discuss avoiding a separate basis-conversion/evaluation
  structure in a recursive implementation.

Impact on our paper:
- This very recent work reinforces that basis layout, recursion, conversion
  boundaries, and locality must be treated jointly.
- It motivates our Priority-1 future work: eliminate or fuse the kernel-to-shear
  conversion rather than merely optimizing the already-parity transform.

## 7. M31 SIMD implementations

### Stwo software

Repository: https://github.com/starkware-libs/stwo

The public Stwo codebase has CPU and SIMD backends and uses M31 as its base
field. Our benchmark pins:
`826591c6c371376810ca8213b5812d5daf6d5092`.

### Plonky3 software

Repository: https://github.com/Plonky3/Plonky3

Plonky3 supports Mersenne31 with NEON, AVX2, and AVX-512 paths and includes a
Mersenne circle-group FFT.

Impact on our paper:
- M31-specific SIMD arithmetic is established prior art.
- Redundant/delayed reduction and architecture-specific vectorization should
  not be presented as novel in isolation.
- Our contribution is the source-verified integration of such arithmetic into
  the specific kernel transform, plus the measured optimization trajectory and
  stable parity result.

## 8. Structural relation discovered during this literature pass

Define the original kernel basis

\[
K_y(X)=\prod_{i=0}^{m-1}(X_i+y_i),
\qquad
X_i=\phi^i(X),
\qquad
\phi(X)=X^2-2.
\]

At one coordinate, the kernel factor basis is

\[
\{X_i,\ X_i+1\}.
\]

The recursive shear changes coordinates to

\[
\{1,\ X_i\}.
\]

Applying the shear at every recursion level therefore produces the fully
sheared coordinate basis

\[
S_b(X)
=
\prod_{i=0}^{m-1}X_i^{b_i},
\qquad
b\in\{0,1\}^m.
\]

Now let `X=2Z` and `pi(Z)=2Z^2-1`. Inductively,

\[
X_i=2\pi^i(Z).
\]

Hence

\[
S_b(2Z)
=
2^{|b|}
\prod_{i=0}^{m-1}\pi^i(Z)^{b_i}.
\]

The univariate part of the Circle FFT basis uses precisely products of the
recursive coordinates

\[
x,\ \pi(x),\ \pi^2(x),\ldots
\]

with binary exponents.

### Interpretation

This is an important bridge, but not an identity of the full transforms.

- The full Circle FFT basis also has the initial `y` coordinate associated with
  circle-to-line projection.
- Our transform is univariate on a split tower.
- Domain ordering and exact twiddles differ.
- Our original kernel basis is not the Circle basis.

What is true:
- after full shear, our recursive coordinate basis is diagonally scaled from the
  same univariate coordinate-product pattern that appears after Circle
  projection;
- both recursions use scaled versions of the same quadratic doubling map;
- both achieve the ideal one-multiplication/two-addition butterfly count.

This structural bridge is a better scientific explanation of the parity result
than treating the two transforms as unrelated black boxes.

## 9. Novelty assessment

### Do not claim as novel
- use of M31;
- use of the circle group;
- q+1-smooth FFTs;
- non-standard polynomial bases for FFTs;
- O(n log n) recursive evaluation;
- one-multiplication/two-addition FFT butterfly counts;
- M31 SIMD/NEON arithmetic;
- redundant or delayed-reduction arithmetic in isolation.

### Potentially distinctive contribution
Subject to a final targeted prior-art search:

1. the specific kernel presentation
   `K_y(X)=prod_i(phi^i(X)+y_i)`;
2. its explicit Boolean shear into the recursive coordinate basis;
3. the exact bridge under `X=2Z` to the Circle line-recursion coordinate
   products;
4. the complete measured optimization trajectory from a >4x gap to stable
   parity with pinned production Stwo;
5. the negative-result evidence isolating which optimizations matter;
6. a reproducible ARM64 source artifact and seven-round parity gate.

### Publication wording

Use:
- "we study";
- "we present the following kernel-basis formulation";
- "we show that the recursive shear exposes...";
- "we identify a scaled correspondence...";
- "we demonstrate stable native-transform parity...".

Avoid until a stronger novelty search is complete:
- "we introduce the first...";
- "novel FFT";
- "new fastest FFT";
- "state of the art";
- "beats Stwo".
