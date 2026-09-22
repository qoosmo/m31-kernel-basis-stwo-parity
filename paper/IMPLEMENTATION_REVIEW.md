# Section 5 implementation provenance

Section 5 was drafted against the exact frozen source:

- repository path: `src/kernel_vs_stwo.rs`
- source SHA-256: `a3a5a14353bd038e5a1f2ccf44d42ca880f2f137040a40ff9851479541ba2c7c`
- repository tag: `v0.20.1-parity`
- Desk Lab: `LAB-010`
- Desk run: `14`

## Source functions described

- `raw_add`
- `raw_sub`
- `raw_mul`
- `raw_canon`
- `neon_add4`
- `neon_sub4`
- `neon_reduce_prod2`
- `neon_mul4_scalar`
- `neon_add2`
- `neon_sub2`
- `neon_mul2`
- `neon_mul2_scalar`
- `neon_radix8_block`
- `neon_level4`
- `neon_tail2_block`
- `neon_tail2_fused`
- `kernel_raw_neon_r8_tail2_inplace`
- `time_kernel_raw_neon_tail2`

## Exact source-derived facts

1. Hot transform values are raw `u32`, not `M31`/`PackedM31`.
2. Redundant interval is `[0,P]`, with `P=0x7fff_ffff`.
3. Multiplication uses a 64-bit product, Mersenne fold
   `(z & P) + (z >> 31)`, and one final conditional subtraction.
4. Four-lane multiplication by a scalar twiddle uses two `vmull_u32`
   operations because each widening intrinsic consumes two u32 lanes.
5. A true radix-8 block keeps eight four-lane input vectors live through
   three transform levels and writes only final vectors.
6. The fused tail consumes exactly the final `k=1,k=0` levels for the target
   benchmark sizes.
7. The tail uses `vuzp1/vuzp2` and `vzip1/vzip2` to rearrange lanes.
8. The timing wrapper includes `out.copy_from_slice(input)`.
9. Canonicalization is excluded from timing.
10. Generic scalar fallback exists but is not used by logN 18..20.

## Claims intentionally not made in Section 5

- that these intrinsics are globally optimal for ARM64;
- that this representation is novel;
- that the implementation beats Stwo;
- that the same performance holds on x86_64 or FPGA;
- any PCS or end-to-end prover performance claim.

Those questions belong to literature review or future experiments.
