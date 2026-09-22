#![feature(portable_simd)]

use std::env;
use std::fs::File;
use std::hint::black_box;
use std::io::{BufWriter, Write};
use std::mem::transmute;
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::{
    uint32x2_t, uint32x4_t, uint64x2_t,
    vadd_u32, vaddq_u32, vaddq_u64, vandq_u64, vcombine_u32,
    vdup_n_u32, vdupq_n_u32, vdupq_n_u64,
    vget_high_u32, vget_low_u32, vld1_u32, vld1q_u32, vmin_u32, vminq_u32,
    vmovn_u64, vmull_u32, vshrq_n_u64, vst1q_u32, vsub_u32, vsubq_u32,
    vuzp1_u32, vuzp2_u32, vzip1_u32, vzip2_u32,
};
use std::time::Instant;

use stwo::core::fields::m31::BaseField;
use stwo::core::poly::circle::CanonicCoset;
use stwo::prover::backend::cpu::{CpuBackend, CpuCirclePoly};
use stwo::prover::backend::simd::column::BaseColumn;
use stwo::prover::backend::simd::fft::rfft::{fft, get_twiddle_dbls};
use stwo::prover::backend::simd::m31::{PackedBaseField, N_LANES};
use stwo::prover::poly::circle::PolyOps;

const P: u32 = 0x7fff_ffff;

#[inline(always)]
fn m31_add(a: u32, b: u32) -> u32 {
    let s = a as u64 + b as u64;
    let mut r = (s & P as u64) + (s >> 31);
    if r >= P as u64 { r -= P as u64; }
    r as u32
}
#[inline(always)]
fn m31_sub(a: u32, b: u32) -> u32 {
    if a >= b { a - b } else { a.wrapping_add(P).wrapping_sub(b) }
}
#[inline(always)]
fn m31_neg(a: u32) -> u32 { if a == 0 { 0 } else { P - a } }
#[inline(always)]
fn m31_mul(a: u32, b: u32) -> u32 {
    let x = a as u64 * b as u64;
    let mut r = (x & P as u64) + (x >> 31);
    r = (r & P as u64) + (r >> 31);
    if r >= P as u64 { r -= P as u64; }
    r as u32
}
#[inline(always)]
fn m31_square(a: u32) -> u32 { m31_mul(a, a) }
#[inline(always)]
fn phi_u32(x: u32) -> u32 { m31_sub(m31_square(x), 2) }
fn sqrt_m31(a: u32) -> Option<u32> {
    if a == 0 { return Some(0); }
    let mut r = a;
    for _ in 0..29 { r = m31_square(r); }
    if m31_square(r) == a { Some(r) } else { None }
}

fn bitrev(mut x: usize, bits: usize) -> usize {
    let mut r = 0usize;
    for _ in 0..bits { r = (r << 1) | (x & 1); x >>= 1; }
    r
}

/// D_i ordered (+a0,...,+ah-1,-a0,...,-ah-1).
fn build_split_tower(n: usize) -> Result<(Vec<Vec<u32>>, Vec<Vec<u32>>), String> {
    let depth = n.trailing_zeros() as usize;
    let mut cur = vec![0u32];
    let mut bottom = vec![cur.clone()];
    for lift in 0..depth {
        let mut pos = Vec::with_capacity(cur.len());
        for &beta in &cur {
            let s = sqrt_m31(m31_add(beta, 2)).ok_or_else(|| format!("non-square lift {lift}"))?;
            if s == 0 { return Err("zero sqrt".into()); }
            pos.push(s);
        }
        let mut prev = Vec::with_capacity(cur.len() * 2);
        prev.extend_from_slice(&pos);
        prev.extend(pos.iter().map(|&x| m31_neg(x)));
        cur = prev;
        bottom.push(cur.clone());
    }
    bottom.reverse();
    let mut levels = Vec::with_capacity(depth);
    for level in 0..depth {
        let d = &bottom[level];
        let next = &bottom[level + 1];
        let h = next.len();
        let pos = &d[..h];
        let neg = &d[h..];
        for j in 0..h {
            if neg[j] != m31_neg(pos[j]) || phi_u32(pos[j]) != next[j] || phi_u32(neg[j]) != next[j] {
                return Err("tower mismatch".into());
            }
        }
        levels.push(pos.to_vec());
    }
    Ok((bottom, levels))
}

fn to_bf_levels(levels: &[Vec<u32>]) -> Vec<Vec<BaseField>> {
    levels.iter().map(|v| v.iter().map(|&x| BaseField::from_u32_unchecked(x)).collect()).collect()
}

/// Original kernel BR-native transform: 1M + 3A per butterfly.
/// Coefficients are native kernel coefficients lambda, already bit-reversed.
fn kernel_br_1m3a(input_br: &[BaseField], levels_br: &[Vec<BaseField>], a: &mut [BaseField]) {
    let n = input_br.len();
    a.copy_from_slice(input_br);
    let m = n.trailing_zeros() as usize;
    for k in (0..m).rev() {
        let span = 1usize << k;
        let alphas = &levels_br[k];
        for (h, &alpha) in alphas.iter().enumerate() {
            let base = h << (k + 1);
            for l in 0..span {
                let li = base + l;
                let ri = li + span;
                let av = a[li];
                let bv = a[ri];
                let s = av + bv;
                let t = alpha * s;
                a[li] = bv + t;
                a[ri] = bv - t;
            }
        }
    }
}

/// Tensor-product shear from kernel coefficients lambda_y to product-level coordinates mu_z.
/// On each binary coordinate: [lambda_0, lambda_1] -> [D, C] = [lambda_1, lambda_0+lambda_1].
/// This conversion is correctness/preparation only and is NOT in the timed native-basis transform.
fn kernel_to_sheared(mut a: Vec<BaseField>) -> Vec<BaseField> {
    let n = a.len();
    let m = n.trailing_zeros() as usize;
    for bit in 0..m {
        let stride = 1usize << bit;
        let block = stride << 1;
        for base in (0..n).step_by(block) {
            for j in 0..stride {
                let i0 = base + j;
                let i1 = i0 + stride;
                let x = a[i0];
                let y = a[i1];
                a[i0] = y;       // D
                a[i1] = x + y;   // C
            }
        }
    }
    a
}

/// Sheared BR-native transform: 1M + 2A per butterfly.
/// Native coefficients are [D,C] recursively, in bit-reversed order.
/// U(+alpha)=D+alpha*C, U(-alpha)=D-alpha*C.
fn kernel_br_1m2a(input_br: &[BaseField], levels_br: &[Vec<BaseField>], a: &mut [BaseField]) {
    let n = input_br.len();
    a.copy_from_slice(input_br);
    let m = n.trailing_zeros() as usize;
    for k in (0..m).rev() {
        let span = 1usize << k;
        let alphas = &levels_br[k];
        for (h, &alpha) in alphas.iter().enumerate() {
            let base = h << (k + 1);
            for l in 0..span {
                let li = base + l;
                let ri = li + span;
                let d = a[li];
                let c = a[ri];
                let t = alpha * c;
                a[li] = d + t;
                a[ri] = d - t;
            }
        }
    }
}


/// Same native sheared BR transform, but executes 3 dependent FFT levels
/// inside each top block before moving to the next block.
/// Field operation count is IDENTICAL to kernel_br_1m2a: 1M + 2A per butterfly.
fn kernel_br_1m2a_fused3(
    input_br: &[BaseField],
    levels_br: &[Vec<BaseField>],
    a: &mut [BaseField],
) {
    let n = input_br.len();
    a.copy_from_slice(input_br);
    let m = n.trailing_zeros() as usize;
    let mut remaining = m;

    while remaining >= 3 {
        let k = remaining - 1;
        let span0 = 1usize << k;

        for (h, &alpha0) in levels_br[k].iter().enumerate() {
            let base0 = h << (k + 1);

            // Level k.
            for l in 0..span0 {
                let li = base0 + l;
                let ri = li + span0;
                let d = a[li];
                let c = a[ri];
                let t = alpha0 * c;
                a[li] = d + t;
                a[ri] = d - t;
            }

            // Level k-1: two child blocks.
            let span1 = 1usize << (k - 1);
            for q in 0..2usize {
                let alpha1 = levels_br[k - 1][(h << 1) + q];
                let base1 = base0 + q * (span1 << 1);
                for l in 0..span1 {
                    let li = base1 + l;
                    let ri = li + span1;
                    let d = a[li];
                    let c = a[ri];
                    let t = alpha1 * c;
                    a[li] = d + t;
                    a[ri] = d - t;
                }
            }

            // Level k-2: four grandchild blocks.
            let span2 = 1usize << (k - 2);
            for q in 0..4usize {
                let alpha2 = levels_br[k - 2][(h << 2) + q];
                let base2 = base0 + q * (span2 << 1);
                for l in 0..span2 {
                    let li = base2 + l;
                    let ri = li + span2;
                    let d = a[li];
                    let c = a[ri];
                    let t = alpha2 * c;
                    a[li] = d + t;
                    a[ri] = d - t;
                }
            }
        }

        remaining -= 3;
    }

    // Finish 1 or 2 leftover low levels.
    for k in (0..remaining).rev() {
        let span = 1usize << k;
        for (h, &alpha) in levels_br[k].iter().enumerate() {
            let base = h << (k + 1);
            for l in 0..span {
                let li = base + l;
                let ri = li + span;
                let d = a[li];
                let c = a[ri];
                let t = alpha * c;
                a[li] = d + t;
                a[ri] = d - t;
            }
        }
    }
}


/// First explicit SIMD kernel gate.
/// Levels with span >= N_LANES use PackedM31 (16 lanes) with one broadcast alpha per block.
/// The final log2(N_LANES)=4 levels use the exact scalar 1M+2A butterfly in-place.
/// This isolates how much of the production SIMD gap disappears before any lane-shuffle tail kernel.
fn kernel_br_1m2a_simd(
    input_br: &BaseColumn,
    levels_br: &[Vec<BaseField>],
    a: &mut BaseColumn,
) {
    assert_eq!(input_br.length, a.length);
    assert_eq!(input_br.data.len(), a.data.len());
    a.data.copy_from_slice(&input_br.data);

    let n = input_br.length;
    let m = n.trailing_zeros() as usize;
    let lane_log = N_LANES.trailing_zeros() as usize;

    // SIMD-aligned upper levels.
    if m > lane_log {
        for k in (lane_log..m).rev() {
            let span = 1usize << k;
            debug_assert_eq!(span % N_LANES, 0);
            let span_vec = span / N_LANES;
            for (h, &alpha) in levels_br[k].iter().enumerate() {
                let alpha_v = PackedBaseField::broadcast(alpha);
                let base_vec = h * (span_vec << 1);
                for q in 0..span_vec {
                    let li = base_vec + q;
                    let ri = li + span_vec;
                    let d = a.data[li];
                    let c = a.data[ri];
                    let t = c * alpha_v;
                    a.data[li] = d + t;
                    a.data[ri] = d - t;
                }
            }
        }
    }

    // Scalar tail for the last 4 levels. A later gate can replace this with lane shuffles.
    let tail_levels = m.min(lane_log);
    let vals = a.as_mut_slice();
    for k in (0..tail_levels).rev() {
        let span = 1usize << k;
        for (h, &alpha) in levels_br[k].iter().enumerate() {
            let base = h << (k + 1);
            for l in 0..span {
                let li = base + l;
                let ri = li + span;
                let d = vals[li];
                let c = vals[ri];
                let t = alpha * c;
                vals[li] = d + t;
                vals[ri] = d - t;
            }
        }
    }
}


/// Gate 5: 3-layer fusion applied to the explicit PackedM31 upper-level schedule.
/// This keeps the same 1M+2A field operation count and same native BR order.
/// Only levels whose three spans are >= N_LANES are fused in packed form;
/// any remaining packed upper levels run singly, and the final 4 levels stay scalar.
fn kernel_br_1m2a_simd_fused3(
    input_br: &BaseColumn,
    levels_br: &[Vec<BaseField>],
    a: &mut BaseColumn,
) {
    assert_eq!(input_br.length, a.length);
    assert_eq!(input_br.data.len(), a.data.len());
    a.data.copy_from_slice(&input_br.data);

    let n = input_br.length;
    let m = n.trailing_zeros() as usize;
    let lane_log = N_LANES.trailing_zeros() as usize;

    // k_top is one past the highest unprocessed level.
    let mut k_top = m;

    // Fuse triples k,k-1,k-2 while all three are SIMD-aligned.
    while k_top >= lane_log + 3 {
        let k = k_top - 1;
        let span0_vec = (1usize << k) / N_LANES;

        for (h, &alpha0) in levels_br[k].iter().enumerate() {
            let alpha0_v = PackedBaseField::broadcast(alpha0);
            let base0 = h * (span0_vec << 1);

            // Level k.
            for q in 0..span0_vec {
                let li = base0 + q;
                let ri = li + span0_vec;
                let d = a.data[li];
                let c = a.data[ri];
                let t = c * alpha0_v;
                a.data[li] = d + t;
                a.data[ri] = d - t;
            }

            // Level k-1: two child blocks.
            let span1_vec = (1usize << (k - 1)) / N_LANES;
            for child in 0..2usize {
                let alpha1_v = PackedBaseField::broadcast(levels_br[k - 1][(h << 1) + child]);
                let base1 = base0 + child * (span1_vec << 1);
                for q in 0..span1_vec {
                    let li = base1 + q;
                    let ri = li + span1_vec;
                    let d = a.data[li];
                    let c = a.data[ri];
                    let t = c * alpha1_v;
                    a.data[li] = d + t;
                    a.data[ri] = d - t;
                }
            }

            // Level k-2: four grandchild blocks.
            let span2_vec = (1usize << (k - 2)) / N_LANES;
            for child in 0..4usize {
                let alpha2_v = PackedBaseField::broadcast(levels_br[k - 2][(h << 2) + child]);
                let base2 = base0 + child * (span2_vec << 1);
                for q in 0..span2_vec {
                    let li = base2 + q;
                    let ri = li + span2_vec;
                    let d = a.data[li];
                    let c = a.data[ri];
                    let t = c * alpha2_v;
                    a.data[li] = d + t;
                    a.data[ri] = d - t;
                }
            }
        }

        k_top -= 3;
    }

    // Finish any 1-2 SIMD-aligned upper levels not included in a triple.
    for k in (lane_log..k_top).rev() {
        let span_vec = (1usize << k) / N_LANES;
        for (h, &alpha) in levels_br[k].iter().enumerate() {
            let alpha_v = PackedBaseField::broadcast(alpha);
            let base = h * (span_vec << 1);
            for q in 0..span_vec {
                let li = base + q;
                let ri = li + span_vec;
                let d = a.data[li];
                let c = a.data[ri];
                let t = c * alpha_v;
                a.data[li] = d + t;
                a.data[ri] = d - t;
            }
        }
    }

    // Same scalar 4-level tail as v0.9: this gate isolates upper-level fusion only.
    let tail_levels = m.min(lane_log);
    let vals = a.as_mut_slice();
    for k in (0..tail_levels).rev() {
        let span = 1usize << k;
        for (h, &alpha) in levels_br[k].iter().enumerate() {
            let base = h << (k + 1);
            for l in 0..span {
                let li = base + l;
                let ri = li + span;
                let d = vals[li];
                let c = vals[ri];
                let t = alpha * c;
                vals[li] = d + t;
                vals[ri] = d - t;
            }
        }
    }
}



/// TRUE packed radix-8 gate.
/// Unlike v0.11, this keeps 8 PackedM31 vectors live across 3 dependent levels
/// and stores them only once at the end of the 3-level network.
fn kernel_br_1m2a_simd_true_r8(
    input_br: &BaseColumn,
    levels_br: &[Vec<BaseField>],
    a: &mut BaseColumn,
) {
    assert_eq!(input_br.length, a.length);
    assert_eq!(input_br.data.len(), a.data.len());
    a.data.copy_from_slice(&input_br.data);

    let n = input_br.length;
    let m = n.trailing_zeros() as usize;
    let lane_log = N_LANES.trailing_zeros() as usize;
    let mut top = m;

    // True 3-level register fusion while all three spans are SIMD aligned.
    while top >= lane_log + 3 {
        let k = top - 1;
        let s_vec = (1usize << (k - 2)) / N_LANES;

        for h in 0..levels_br[k].len() {
            let base_vec = h * (s_vec << 3); // 8 packed subspans.

            let a0  = PackedBaseField::broadcast(levels_br[k][h]);
            let a10 = PackedBaseField::broadcast(levels_br[k - 1][h << 1]);
            let a11 = PackedBaseField::broadcast(levels_br[k - 1][(h << 1) + 1]);
            let qh = h << 2;
            let a20 = PackedBaseField::broadcast(levels_br[k - 2][qh]);
            let a21 = PackedBaseField::broadcast(levels_br[k - 2][qh + 1]);
            let a22 = PackedBaseField::broadcast(levels_br[k - 2][qh + 2]);
            let a23 = PackedBaseField::broadcast(levels_br[k - 2][qh + 3]);

            for q in 0..s_vec {
                let i0 = base_vec + q;
                let i1 = i0 + s_vec;
                let i2 = i1 + s_vec;
                let i3 = i2 + s_vec;
                let i4 = i3 + s_vec;
                let i5 = i4 + s_vec;
                let i6 = i5 + s_vec;
                let i7 = i6 + s_vec;

                let x0 = a.data[i0]; let x1 = a.data[i1];
                let x2 = a.data[i2]; let x3 = a.data[i3];
                let x4 = a.data[i4]; let x5 = a.data[i5];
                let x6 = a.data[i6]; let x7 = a.data[i7];

                // Level k.
                let t0 = x4 * a0; let y0 = x0 + t0; let y4 = x0 - t0;
                let t1 = x5 * a0; let y1 = x1 + t1; let y5 = x1 - t1;
                let t2 = x6 * a0; let y2 = x2 + t2; let y6 = x2 - t2;
                let t3 = x7 * a0; let y3 = x3 + t3; let y7 = x3 - t3;

                // Level k-1.
                let t4 = y2 * a10; let z0 = y0 + t4; let z2 = y0 - t4;
                let t5 = y3 * a10; let z1 = y1 + t5; let z3 = y1 - t5;
                let t6 = y6 * a11; let z4 = y4 + t6; let z6 = y4 - t6;
                let t7 = y7 * a11; let z5 = y5 + t7; let z7 = y5 - t7;

                // Level k-2.
                let t8  = z1 * a20; let w0 = z0 + t8;  let w1 = z0 - t8;
                let t9  = z3 * a21; let w2 = z2 + t9;  let w3 = z2 - t9;
                let t10 = z5 * a22; let w4 = z4 + t10; let w5 = z4 - t10;
                let t11 = z7 * a23; let w6 = z6 + t11; let w7 = z6 - t11;

                a.data[i0] = w0; a.data[i1] = w1;
                a.data[i2] = w2; a.data[i3] = w3;
                a.data[i4] = w4; a.data[i5] = w5;
                a.data[i6] = w6; a.data[i7] = w7;
            }
        }
        top -= 3;
    }

    // If two SIMD-aligned levels remain, fuse them as radix-4.
    if top >= lane_log + 2 {
        let k = top - 1;
        let s_vec = (1usize << (k - 1)) / N_LANES;
        for h in 0..levels_br[k].len() {
            let base_vec = h * (s_vec << 2);
            let a0  = PackedBaseField::broadcast(levels_br[k][h]);
            let a10 = PackedBaseField::broadcast(levels_br[k - 1][h << 1]);
            let a11 = PackedBaseField::broadcast(levels_br[k - 1][(h << 1) + 1]);

            for q in 0..s_vec {
                let i0 = base_vec + q;
                let i1 = i0 + s_vec;
                let i2 = i1 + s_vec;
                let i3 = i2 + s_vec;

                let x0 = a.data[i0]; let x1 = a.data[i1];
                let x2 = a.data[i2]; let x3 = a.data[i3];

                let t0 = x2 * a0; let y0 = x0 + t0; let y2 = x0 - t0;
                let t1 = x3 * a0; let y1 = x1 + t1; let y3 = x1 - t1;

                let t2 = y1 * a10; let z0 = y0 + t2; let z1 = y0 - t2;
                let t3 = y3 * a11; let z2 = y2 + t3; let z3 = y2 - t3;

                a.data[i0] = z0; a.data[i1] = z1;
                a.data[i2] = z2; a.data[i3] = z3;
            }
        }
        top -= 2;
    }

    // One remaining SIMD-aligned level.
    if top > lane_log {
        let k = top - 1;
        let span_vec = (1usize << k) / N_LANES;
        for (h, &alpha) in levels_br[k].iter().enumerate() {
            let av = PackedBaseField::broadcast(alpha);
            let base_vec = h * (span_vec << 1);
            for q in 0..span_vec {
                let li = base_vec + q;
                let ri = li + span_vec;
                let d = a.data[li];
                let c = a.data[ri];
                let t = c * av;
                a.data[li] = d + t;
                a.data[ri] = d - t;
            }
        }
        top -= 1;
    }

    debug_assert_eq!(top, lane_log.min(m));

    // Same scalar lane-local tail as v0.9-v0.11.
    let vals = a.as_mut_slice();
    for k in (0..top).rev() {
        let span = 1usize << k;
        for (h, &alpha) in levels_br[k].iter().enumerate() {
            let base = h << (k + 1);
            for l in 0..span {
                let li = base + l;
                let ri = li + span;
                let d = vals[li];
                let c = vals[ri];
                let t = alpha * c;
                vals[li] = d + t;
                vals[ri] = d - t;
            }
        }
    }
}


const RAW_P: u32 = 0x7fff_ffff;

#[inline(always)]
fn raw_add(a: u32, b: u32) -> u32 {
    let c = a.wrapping_add(b);
    c.min(c.wrapping_sub(RAW_P))
}

#[inline(always)]
fn raw_sub(a: u32, b: u32) -> u32 {
    let c = a.wrapping_sub(b);
    c.min(c.wrapping_add(RAW_P))
}

#[inline(always)]
fn raw_mul(a: u32, b: u32) -> u32 {
    let z = (a as u64) * (b as u64);
    // First Mersenne fold gives r in [0, 2P).
    let r = ((z & RAW_P as u64) + (z >> 31)) as u32;
    // Restore the redundant invariant [0, P].
    r.min(r.wrapping_sub(RAW_P))
}

#[inline(always)]
fn raw_canon(a: u32) -> u32 {
    if a == RAW_P { 0 } else { a }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_add4(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    let c = unsafe { vaddq_u32(a, b) };
    let p = unsafe { vdupq_n_u32(RAW_P) };
    unsafe { vminq_u32(c, vsubq_u32(c, p)) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_sub4(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    let c = unsafe { vsubq_u32(a, b) };
    let p = unsafe { vdupq_n_u32(RAW_P) };
    unsafe { vminq_u32(c, vaddq_u32(c, p)) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_reduce_prod2(z: uint64x2_t) -> uint32x2_t {
    let mask = unsafe { vdupq_n_u64(RAW_P as u64) };
    let lo = unsafe { vandq_u64(z, mask) };
    let hi = unsafe { vshrq_n_u64::<31>(z) };
    // First fold is in [0, 2P). Narrow, then conditionally subtract P
    // lane-wise while preserving redundant 0 == P.
    let r = unsafe { vmovn_u64(vaddq_u64(lo, hi)) };
    let p = unsafe { vdup_n_u32(RAW_P) };
    unsafe { core::arch::aarch64::vmin_u32(r, core::arch::aarch64::vsub_u32(r, p)) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_mul4_scalar(a: uint32x4_t, b: u32) -> uint32x4_t {
    let bv = unsafe { vdup_n_u32(b) };
    let z0 = unsafe { vmull_u32(vget_low_u32(a), bv) };
    let z1 = unsafe { vmull_u32(vget_high_u32(a), bv) };
    let r0 = unsafe { neon_reduce_prod2(z0) };
    let r1 = unsafe { neon_reduce_prod2(z1) };
    unsafe { vcombine_u32(r0, r1) }
}


#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_add2(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    let c = unsafe { vadd_u32(a, b) };
    let p = unsafe { vdup_n_u32(RAW_P) };
    unsafe { vmin_u32(c, vsub_u32(c, p)) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_sub2(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    let c = unsafe { vsub_u32(a, b) };
    let p = unsafe { vdup_n_u32(RAW_P) };
    unsafe { vmin_u32(c, vadd_u32(c, p)) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_mul2(a: uint32x2_t, b: uint32x2_t) -> uint32x2_t {
    unsafe { neon_reduce_prod2(vmull_u32(a, b)) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_mul2_scalar(a: uint32x2_t, b: u32) -> uint32x2_t {
    let bv = unsafe { vdup_n_u32(b) };
    unsafe { neon_reduce_prod2(vmull_u32(a, bv)) }
}

/// Fused final k=1 and k=0 transform levels on one four-element block.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_tail2_block(
    ptr: *mut u32,
    base: usize,
    alpha1: u32,
    alpha0_left: u32,
    alpha0_right: u32,
) {
    let x = unsafe { vld1q_u32(ptr.add(base)) };
    let d = unsafe { vget_low_u32(x) };   // [x0,x1]
    let c = unsafe { vget_high_u32(x) };  // [x2,x3]

    // k=1: (0,2), (1,3).
    let t1 = unsafe { neon_mul2_scalar(c, alpha1) };
    let ylo = unsafe { neon_add2(d, t1) }; // [y0,y1]
    let yhi = unsafe { neon_sub2(d, t1) }; // [y2,y3]

    // k=0 needs [y0,y2] as left values and [y1,y3] as right values.
    let even = unsafe { vuzp1_u32(ylo, yhi) };
    let odd  = unsafe { vuzp2_u32(ylo, yhi) };
    let tw = [alpha0_left, alpha0_right];
    let twv = unsafe { vld1_u32(tw.as_ptr()) };

    let t0 = unsafe { neon_mul2(odd, twv) };
    let zout_even = unsafe { neon_add2(even, t0) }; // [z0,z2]
    let zout_odd  = unsafe { neon_sub2(even, t0) }; // [z1,z3]

    let lo = unsafe { vzip1_u32(zout_even, zout_odd) };
    let hi = unsafe { vzip2_u32(zout_even, zout_odd) };
    let out = unsafe { vcombine_u32(lo, hi) };
    unsafe { vst1q_u32(ptr.add(base), out) };
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_tail2_fused(ptr: *mut u32, n: usize, levels: &[Vec<u32>]) {
    debug_assert_eq!(levels[1].len(), n / 4);
    debug_assert_eq!(levels[0].len(), n / 2);
    for h in 0..(n / 4) {
        unsafe {
            neon_tail2_block(
                ptr,
                h << 2,
                levels[1][h],
                levels[0][h << 1],
                levels[0][(h << 1) + 1],
            );
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_radix8_block(
    ptr: *mut u32, base: usize, s: usize,
    a0: u32, a10: u32, a11: u32,
    a20: u32, a21: u32, a22: u32, a23: u32,
) {
    let mut l = 0usize;
    while l < s {
        let p0 = unsafe { ptr.add(base + l) };
        let p1 = unsafe { ptr.add(base + l + s) };
        let p2 = unsafe { ptr.add(base + l + 2*s) };
        let p3 = unsafe { ptr.add(base + l + 3*s) };
        let p4 = unsafe { ptr.add(base + l + 4*s) };
        let p5 = unsafe { ptr.add(base + l + 5*s) };
        let p6 = unsafe { ptr.add(base + l + 6*s) };
        let p7 = unsafe { ptr.add(base + l + 7*s) };

        let x0 = unsafe { vld1q_u32(p0) }; let x1 = unsafe { vld1q_u32(p1) };
        let x2 = unsafe { vld1q_u32(p2) }; let x3 = unsafe { vld1q_u32(p3) };
        let x4 = unsafe { vld1q_u32(p4) }; let x5 = unsafe { vld1q_u32(p5) };
        let x6 = unsafe { vld1q_u32(p6) }; let x7 = unsafe { vld1q_u32(p7) };

        let t0 = unsafe { neon_mul4_scalar(x4, a0) };
        let y0 = unsafe { neon_add4(x0,t0) }; let y4 = unsafe { neon_sub4(x0,t0) };
        let t1 = unsafe { neon_mul4_scalar(x5, a0) };
        let y1 = unsafe { neon_add4(x1,t1) }; let y5 = unsafe { neon_sub4(x1,t1) };
        let t2 = unsafe { neon_mul4_scalar(x6, a0) };
        let y2 = unsafe { neon_add4(x2,t2) }; let y6 = unsafe { neon_sub4(x2,t2) };
        let t3 = unsafe { neon_mul4_scalar(x7, a0) };
        let y3 = unsafe { neon_add4(x3,t3) }; let y7 = unsafe { neon_sub4(x3,t3) };

        let t4 = unsafe { neon_mul4_scalar(y2, a10) };
        let z0 = unsafe { neon_add4(y0,t4) }; let z2 = unsafe { neon_sub4(y0,t4) };
        let t5 = unsafe { neon_mul4_scalar(y3, a10) };
        let z1 = unsafe { neon_add4(y1,t5) }; let z3 = unsafe { neon_sub4(y1,t5) };
        let t6 = unsafe { neon_mul4_scalar(y6, a11) };
        let z4 = unsafe { neon_add4(y4,t6) }; let z6 = unsafe { neon_sub4(y4,t6) };
        let t7 = unsafe { neon_mul4_scalar(y7, a11) };
        let z5 = unsafe { neon_add4(y5,t7) }; let z7 = unsafe { neon_sub4(y5,t7) };

        let t8 = unsafe { neon_mul4_scalar(z1, a20) };
        let w0 = unsafe { neon_add4(z0,t8) }; let w1 = unsafe { neon_sub4(z0,t8) };
        let t9 = unsafe { neon_mul4_scalar(z3, a21) };
        let w2 = unsafe { neon_add4(z2,t9) }; let w3 = unsafe { neon_sub4(z2,t9) };
        let t10 = unsafe { neon_mul4_scalar(z5, a22) };
        let w4 = unsafe { neon_add4(z4,t10) }; let w5 = unsafe { neon_sub4(z4,t10) };
        let t11 = unsafe { neon_mul4_scalar(z7, a23) };
        let w6 = unsafe { neon_add4(z6,t11) }; let w7 = unsafe { neon_sub4(z6,t11) };

        unsafe {
            vst1q_u32(p0,w0); vst1q_u32(p1,w1); vst1q_u32(p2,w2); vst1q_u32(p3,w3);
            vst1q_u32(p4,w4); vst1q_u32(p5,w5); vst1q_u32(p6,w6); vst1q_u32(p7,w7);
        }
        l += 4;
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_level4(ptr: *mut u32, base: usize, span: usize, alpha: u32) {
    let mut l = 0usize;
    while l < span {
        let pl = unsafe { ptr.add(base + l) };
        let pr = unsafe { ptr.add(base + span + l) };
        let d = unsafe { vld1q_u32(pl) };
        let c = unsafe { vld1q_u32(pr) };
        let t = unsafe { neon_mul4_scalar(c, alpha) };
        unsafe {
            vst1q_u32(pl, neon_add4(d,t));
            vst1q_u32(pr, neon_sub4(d,t));
        }
        l += 4;
    }
}

#[cfg(target_arch = "aarch64")]
fn kernel_raw_neon_r8_inplace_old(a: &mut [u32], levels: &[Vec<u32>]) {
    let m = a.len().trailing_zeros() as usize;
    let ptr = a.as_mut_ptr();
    let mut top = m;

    while top >= 5 {
        let k = top - 1;
        let span8 = 1usize << (k - 2);
        for h in 0..levels[k].len() {
            let base = h << (k + 1);
            let q = h << 2;
            unsafe {
                neon_radix8_block(
                    ptr, base, span8,
                    levels[k][h],
                    levels[k-1][h<<1], levels[k-1][(h<<1)+1],
                    levels[k-2][q], levels[k-2][q+1],
                    levels[k-2][q+2], levels[k-2][q+3],
                );
            }
        }
        top -= 3;
    }

    while top > 2 {
        let k = top - 1;
        let span = 1usize << k;
        for h in 0..levels[k].len() {
            unsafe { neon_level4(ptr, h << (k+1), span, levels[k][h]); }
        }
        top -= 1;
    }

    for k in (0..top).rev() {
        let span = 1usize << k;
        for h in 0..levels[k].len() {
            let base = h << (k+1);
            let alpha = levels[k][h];
            for l in 0..span {
                let li = base+l;
                let ri = li+span;
                let d = a[li];
                let t = raw_mul(a[ri], alpha);
                a[li] = raw_add(d,t);
                a[ri] = raw_sub(d,t);
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
fn kernel_raw_neon_r8_old(input: &[u32], levels: &[Vec<u32>], out: &mut [u32]) {
    out.copy_from_slice(input);
    kernel_raw_neon_r8_inplace_old(out, levels);
}

#[cfg(target_arch = "aarch64")]
fn time_kernel_raw_neon_old(input: &[u32], levels: &[Vec<u32>], reps: usize) -> u128 {
    let mut out = vec![0u32; input.len()];
    for _ in 0..2 {
        kernel_raw_neon_r8_old(input, levels, &mut out);
        black_box(out.as_ptr());
    }
    let mut ts = Vec::with_capacity(reps);
    for _ in 0..reps {
        let t = Instant::now();
        kernel_raw_neon_r8_old(input, levels, &mut out);
        ts.push(t.elapsed().as_nanos());
        black_box(out.as_ptr());
    }
    median_ns(ts)
}


#[cfg(target_arch = "aarch64")]
fn kernel_raw_neon_r8_tail2_inplace(a: &mut [u32], levels: &[Vec<u32>]) {
    let n = a.len();
    let m = n.trailing_zeros() as usize;
    let ptr = a.as_mut_ptr();
    let mut top = m;

    while top >= 5 {
        let k = top - 1;
        let span8 = 1usize << (k - 2);
        for h in 0..levels[k].len() {
            let base = h << (k + 1);
            let q = h << 2;
            unsafe {
                neon_radix8_block(
                    ptr, base, span8,
                    levels[k][h],
                    levels[k-1][h<<1], levels[k-1][(h<<1)+1],
                    levels[k-2][q], levels[k-2][q+1],
                    levels[k-2][q+2], levels[k-2][q+3],
                );
            }
        }
        top -= 3;
    }

    while top > 2 {
        let k = top - 1;
        let span = 1usize << k;
        for h in 0..levels[k].len() {
            unsafe { neon_level4(ptr, h << (k+1), span, levels[k][h]); }
        }
        top -= 1;
    }

    if top == 2 {
        unsafe { neon_tail2_fused(ptr, n, levels); }
        top = 0;
    }

    // General fallback; not used by logN 18..20.
    for k in (0..top).rev() {
        let span = 1usize << k;
        for h in 0..levels[k].len() {
            let base = h << (k+1);
            let alpha = levels[k][h];
            for l in 0..span {
                let li = base+l;
                let ri = li+span;
                let d = a[li];
                let t = raw_mul(a[ri], alpha);
                a[li] = raw_add(d,t);
                a[ri] = raw_sub(d,t);
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
fn kernel_raw_neon_r8_tail2(input: &[u32], levels: &[Vec<u32>], out: &mut [u32]) {
    out.copy_from_slice(input);
    kernel_raw_neon_r8_tail2_inplace(out, levels);
}

#[cfg(target_arch = "aarch64")]
fn time_kernel_raw_neon_tail2(input: &[u32], levels: &[Vec<u32>], reps: usize) -> u128 {
    let mut out = vec![0u32; input.len()];
    for _ in 0..2 {
        kernel_raw_neon_r8_tail2(input, levels, &mut out);
        black_box(out.as_ptr());
    }
    let mut ts = Vec::with_capacity(reps);
    for _ in 0..reps {
        let t = Instant::now();
        kernel_raw_neon_r8_tail2(input, levels, &mut out);
        ts.push(t.elapsed().as_nanos());
        black_box(out.as_ptr());
    }
    median_ns(ts)
}


#[inline(always)]
unsafe fn raw_level(
    ptr: *mut BaseField,
    base: usize,
    span: usize,
    alpha: BaseField,
) {
    for l in 0..span {
        let li = base + l;
        let ri = li + span;
        let d = unsafe { *ptr.add(li) };
        let c = unsafe { *ptr.add(ri) };
        let t = alpha * c;
        unsafe {
            *ptr.add(li) = d + t;
            *ptr.add(ri) = d - t;
        }
    }
}

/// Raw-pointer/codegen gate. Same sheared BR-native 1M+2A transform.
/// F controls how many dependent levels are processed per top block.
/// No field-operation changes; this only removes slice bounds checks and sweeps fusion depth.
fn kernel_br_1m2a_raw_fused<const F: usize>(
    input_br: &[BaseField],
    levels_br: &[Vec<BaseField>],
    a: &mut [BaseField],
) {
    assert!(F >= 1);
    let n = input_br.len();
    a.copy_from_slice(input_br);
    let m = n.trailing_zeros() as usize;
    let mut top = m;
    let ptr = a.as_mut_ptr();

    while top >= F {
        let k = top - 1;
        let span0 = 1usize << k;
        let n_top_blocks = levels_br[k].len();
        for h in 0..n_top_blocks {
            let base0 = h << (k + 1);
            for stage in 0..F {
                let kk = k - stage;
                let span = 1usize << kk;
                let children = 1usize << stage;
                let alpha_base = h << stage;
                for child in 0..children {
                    let alpha = unsafe { *levels_br.get_unchecked(kk).get_unchecked(alpha_base + child) };
                    let base = base0 + child * (span << 1);
                    unsafe { raw_level(ptr, base, span, alpha); }
                }
            }
        }
        top -= F;
    }

    // Leftover low levels.
    for k in (0..top).rev() {
        let span = 1usize << k;
        let alphas = unsafe { levels_br.get_unchecked(k) };
        for h in 0..alphas.len() {
            let alpha = unsafe { *alphas.get_unchecked(h) };
            let base = h << (k + 1);
            unsafe { raw_level(ptr, base, span, alpha); }
        }
    }
}

fn kernel_raw_f2(i:&[BaseField], l:&[Vec<BaseField>], a:&mut[BaseField]) { kernel_br_1m2a_raw_fused::<2>(i,l,a) }
fn kernel_raw_f3(i:&[BaseField], l:&[Vec<BaseField>], a:&mut[BaseField]) { kernel_br_1m2a_raw_fused::<3>(i,l,a) }
fn kernel_raw_f4(i:&[BaseField], l:&[Vec<BaseField>], a:&mut[BaseField]) { kernel_br_1m2a_raw_fused::<4>(i,l,a) }
fn kernel_raw_f5(i:&[BaseField], l:&[Vec<BaseField>], a:&mut[BaseField]) { kernel_br_1m2a_raw_fused::<5>(i,l,a) }
fn kernel_raw_f6(i:&[BaseField], l:&[Vec<BaseField>], a:&mut[BaseField]) { kernel_br_1m2a_raw_fused::<6>(i,l,a) }

// TRUE register-fused radix-4: 2 dependent levels are performed on 4 live values
// before any of those values are stored back to memory.
#[inline(always)]
unsafe fn true_radix4_block(
    ptr: *mut BaseField,
    base: usize,
    s: usize,
    a0: BaseField,
    a10: BaseField,
    a11: BaseField,
) {
    for l in 0..s {
        let p0 = unsafe { ptr.add(base + l) };
        let p1 = unsafe { ptr.add(base + l + s) };
        let p2 = unsafe { ptr.add(base + l + 2*s) };
        let p3 = unsafe { ptr.add(base + l + 3*s) };

        let x0 = unsafe { *p0 };
        let x1 = unsafe { *p1 };
        let x2 = unsafe { *p2 };
        let x3 = unsafe { *p3 };

        // Top level: (0,2), (1,3), same alpha.
        let t0 = a0 * x2;
        let y0 = x0 + t0;
        let y2 = x0 - t0;
        let t1 = a0 * x3;
        let y1 = x1 + t1;
        let y3 = x1 - t1;

        // Child level: (0,1), (2,3).
        let t2 = a10 * y1;
        let z0 = y0 + t2;
        let z1 = y0 - t2;
        let t3 = a11 * y3;
        let z2 = y2 + t3;
        let z3 = y2 - t3;

        unsafe {
            *p0 = z0; *p1 = z1; *p2 = z2; *p3 = z3;
        }
    }
}

// TRUE register-fused radix-8: 3 dependent levels are performed on 8 live values
// before any of those values are stored back to memory.
// This is the scalar/auto-vectorizable analogue of Stwo's fft3 microkernel.
#[inline(always)]
unsafe fn true_radix8_block(
    ptr: *mut BaseField,
    base: usize,
    s: usize,
    a0: BaseField,
    a10: BaseField,
    a11: BaseField,
    a20: BaseField,
    a21: BaseField,
    a22: BaseField,
    a23: BaseField,
) {
    for l in 0..s {
        let p0 = unsafe { ptr.add(base + l) };
        let p1 = unsafe { ptr.add(base + l + s) };
        let p2 = unsafe { ptr.add(base + l + 2*s) };
        let p3 = unsafe { ptr.add(base + l + 3*s) };
        let p4 = unsafe { ptr.add(base + l + 4*s) };
        let p5 = unsafe { ptr.add(base + l + 5*s) };
        let p6 = unsafe { ptr.add(base + l + 6*s) };
        let p7 = unsafe { ptr.add(base + l + 7*s) };

        let x0 = unsafe { *p0 }; let x1 = unsafe { *p1 };
        let x2 = unsafe { *p2 }; let x3 = unsafe { *p3 };
        let x4 = unsafe { *p4 }; let x5 = unsafe { *p5 };
        let x6 = unsafe { *p6 }; let x7 = unsafe { *p7 };

        // Level 0: four butterflies, common alpha a0.
        let t0 = a0 * x4; let y0 = x0 + t0; let y4 = x0 - t0;
        let t1 = a0 * x5; let y1 = x1 + t1; let y5 = x1 - t1;
        let t2 = a0 * x6; let y2 = x2 + t2; let y6 = x2 - t2;
        let t3 = a0 * x7; let y3 = x3 + t3; let y7 = x3 - t3;

        // Level 1: two child alphas.
        let t4 = a10 * y2; let z0 = y0 + t4; let z2 = y0 - t4;
        let t5 = a10 * y3; let z1 = y1 + t5; let z3 = y1 - t5;
        let t6 = a11 * y6; let z4 = y4 + t6; let z6 = y4 - t6;
        let t7 = a11 * y7; let z5 = y5 + t7; let z7 = y5 - t7;

        // Level 2: four grandchild alphas.
        let t8  = a20 * z1; let w0 = z0 + t8;  let w1 = z0 - t8;
        let t9  = a21 * z3; let w2 = z2 + t9;  let w3 = z2 - t9;
        let t10 = a22 * z5; let w4 = z4 + t10; let w5 = z4 - t10;
        let t11 = a23 * z7; let w6 = z6 + t11; let w7 = z6 - t11;

        unsafe {
            *p0=w0; *p1=w1; *p2=w2; *p3=w3;
            *p4=w4; *p5=w5; *p6=w6; *p7=w7;
        }
    }
}

fn kernel_true_r4_inplace(a: &mut [BaseField], levels_br: &[Vec<BaseField>]) {
    let m = a.len().trailing_zeros() as usize;
    let ptr = a.as_mut_ptr();
    let mut top = m;

    while top >= 2 {
        let k = top - 1;
        let s = 1usize << (k - 1);
        for h in 0..levels_br[k].len() {
            let base = h << (k + 1);
            let a0  = unsafe { *levels_br.get_unchecked(k).get_unchecked(h) };
            let a10 = unsafe { *levels_br.get_unchecked(k-1).get_unchecked((h<<1)    ) };
            let a11 = unsafe { *levels_br.get_unchecked(k-1).get_unchecked((h<<1) + 1) };
            unsafe { true_radix4_block(ptr, base, s, a0, a10, a11); }
        }
        top -= 2;
    }

    for k in (0..top).rev() {
        let span = 1usize << k;
        let alphas = unsafe { levels_br.get_unchecked(k) };
        for h in 0..alphas.len() {
            let alpha = unsafe { *alphas.get_unchecked(h) };
            unsafe { raw_level(ptr, h << (k+1), span, alpha); }
        }
    }
}

fn kernel_true_r8_inplace(a: &mut [BaseField], levels_br: &[Vec<BaseField>]) {
    let m = a.len().trailing_zeros() as usize;
    let ptr = a.as_mut_ptr();
    let mut top = m;

    while top >= 3 {
        let k = top - 1;
        let s = 1usize << (k - 2);
        for h in 0..levels_br[k].len() {
            let base = h << (k + 1);
            let a0 = unsafe { *levels_br.get_unchecked(k).get_unchecked(h) };
            let a10 = unsafe { *levels_br.get_unchecked(k-1).get_unchecked((h<<1)    ) };
            let a11 = unsafe { *levels_br.get_unchecked(k-1).get_unchecked((h<<1) + 1) };
            let q = h << 2;
            let a20 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q    ) };
            let a21 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q + 1) };
            let a22 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q + 2) };
            let a23 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q + 3) };
            unsafe { true_radix8_block(ptr, base, s, a0, a10, a11, a20, a21, a22, a23); }
        }
        top -= 3;
    }

    if top == 2 {
        let k = 1usize;
        let s = 1usize;
        for h in 0..levels_br[k].len() {
            let base = h << 2;
            let a0  = unsafe { *levels_br.get_unchecked(1).get_unchecked(h) };
            let a10 = unsafe { *levels_br.get_unchecked(0).get_unchecked((h<<1)    ) };
            let a11 = unsafe { *levels_br.get_unchecked(0).get_unchecked((h<<1) + 1) };
            unsafe { true_radix4_block(ptr, base, s, a0, a10, a11); }
        }
    } else if top == 1 {
        let alphas = unsafe { levels_br.get_unchecked(0) };
        for h in 0..alphas.len() {
            unsafe { raw_level(ptr, h << 1, 1, *alphas.get_unchecked(h)); }
        }
    }
}

fn kernel_true_r4(input:&[BaseField], levels:&[Vec<BaseField>], a:&mut[BaseField]) {
    a.copy_from_slice(input);
    kernel_true_r4_inplace(a, levels);
}
fn kernel_true_r8(input:&[BaseField], levels:&[Vec<BaseField>], a:&mut[BaseField]) {
    a.copy_from_slice(input);
    kernel_true_r8_inplace(a, levels);
}

// Cache-blocked radix-8 schedule.
// High levels are executed globally. Once subtrees have size 2^block_log,
// each subtree is finished completely before moving to the next subtree.
// This is algebraically identical to the global level-by-level transform.
fn kernel_cache_r8_inplace(
    a: &mut [BaseField],
    levels_br: &[Vec<BaseField>],
    block_log: usize,
) {
    let m = a.len().trailing_zeros() as usize;
    let b = block_log.min(m);
    let ptr = a.as_mut_ptr();
    let mut top = m;

    // Global true-radix8 groups that stay strictly above the cache-block boundary.
    while top >= b + 3 {
        let k = top - 1;
        let span8 = 1usize << (k - 2);
        for h in 0..levels_br[k].len() {
            let base = h << (k + 1);
            let a0 = unsafe { *levels_br.get_unchecked(k).get_unchecked(h) };
            let a10 = unsafe { *levels_br.get_unchecked(k-1).get_unchecked(h << 1) };
            let a11 = unsafe { *levels_br.get_unchecked(k-1).get_unchecked((h << 1) + 1) };
            let q = h << 2;
            let a20 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q) };
            let a21 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q + 1) };
            let a22 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q + 2) };
            let a23 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q + 3) };
            unsafe { true_radix8_block(ptr, base, span8, a0, a10, a11, a20, a21, a22, a23); }
        }
        top -= 3;
    }

    // Any 1-2 global levels needed to land exactly on the block boundary.
    for k in (b..top).rev() {
        let span = 1usize << k;
        let alphas = unsafe { levels_br.get_unchecked(k) };
        for h in 0..alphas.len() {
            let alpha = unsafe { *alphas.get_unchecked(h) };
            unsafe { raw_level(ptr, h << (k + 1), span, alpha); }
        }
    }

    // Finish each low-bit subtree completely while it is hot in cache.
    let n_blocks = 1usize << (m - b);
    for hp in 0..n_blocks {
        let block_base = hp << b;
        let mut local_top = b;

        while local_top >= 3 {
            let k = local_top - 1;
            let span8 = 1usize << (k - 2);
            let local_h_count = 1usize << (b - k - 1);

            for hl in 0..local_h_count {
                // Global twiddle index = fixed high prefix || local high bits.
                let gh = (hp << (b - k - 1)) | hl;
                let base = block_base + (hl << (k + 1));

                let a0 = unsafe { *levels_br.get_unchecked(k).get_unchecked(gh) };
                let a10 = unsafe { *levels_br.get_unchecked(k-1).get_unchecked(gh << 1) };
                let a11 = unsafe { *levels_br.get_unchecked(k-1).get_unchecked((gh << 1) + 1) };
                let q = gh << 2;
                let a20 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q) };
                let a21 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q + 1) };
                let a22 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q + 2) };
                let a23 = unsafe { *levels_br.get_unchecked(k-2).get_unchecked(q + 3) };

                unsafe { true_radix8_block(ptr, base, span8, a0, a10, a11, a20, a21, a22, a23); }
            }
            local_top -= 3;
        }

        // Tail of 1-2 levels inside this block.
        for k in (0..local_top).rev() {
            let span = 1usize << k;
            let local_h_count = 1usize << (b - k - 1);
            for hl in 0..local_h_count {
                let gh = (hp << (b - k - 1)) | hl;
                let alpha = unsafe { *levels_br.get_unchecked(k).get_unchecked(gh) };
                let base = block_base + (hl << (k + 1));
                unsafe { raw_level(ptr, base, span, alpha); }
            }
        }
    }
}

fn kernel_cache_r8(
    input: &[BaseField],
    levels: &[Vec<BaseField>],
    a: &mut [BaseField],
    block_log: usize,
) {
    a.copy_from_slice(input);
    kernel_cache_r8_inplace(a, levels, block_log);
}

fn time_kernel_cache(
    input: &[BaseField],
    levels: &[Vec<BaseField>],
    reps: usize,
    block_log: usize,
) -> u128 {
    let mut a = vec![BaseField::from_u32_unchecked(0); input.len()];
    for _ in 0..2 {
        kernel_cache_r8(input, levels, &mut a, block_log);
        black_box(a.as_ptr());
    }
    let mut ts = Vec::with_capacity(reps);
    for _ in 0..reps {
        let t = Instant::now();
        kernel_cache_r8(input, levels, &mut a, block_log);
        ts.push(t.elapsed().as_nanos());
        black_box(a.as_ptr());
    }
    median_ns(ts)
}

fn bitrev_alpha_levels(levels: &[Vec<BaseField>]) -> Vec<Vec<BaseField>> {
    levels.iter().map(|v| {
        let bits = if v.len() <= 1 { 0 } else { v.len().trailing_zeros() as usize };
        (0..v.len()).map(|h| v[bitrev(h, bits)]).collect()
    }).collect()
}

fn bitrev_vec(v: &[BaseField]) -> Vec<BaseField> {
    let bits = v.len().trailing_zeros() as usize;
    (0..v.len()).map(|r| v[bitrev(r, bits)]).collect()
}

#[inline(always)]
fn phi_bf(x: BaseField) -> BaseField { x * x - BaseField::from_u32_unchecked(2) }

fn kernel_at(index: usize, m: usize, x: BaseField) -> BaseField {
    let mut xi = x;
    let mut acc = BaseField::from_u32_unchecked(1);
    for i in 0..m {
        let bit = (index >> (m - 1 - i)) & 1;
        acc = acc * (xi + BaseField::from_u32_unchecked(bit as u32));
        xi = phi_bf(xi);
    }
    acc
}

fn brute_eval(input: &[BaseField], domain: &[u32]) -> Vec<BaseField> {
    let m = input.len().trailing_zeros() as usize;
    domain.iter().map(|&xu| {
        let x = BaseField::from_u32_unchecked(xu);
        let mut sum = BaseField::from_u32_unchecked(0);
        for (i, &c) in input.iter().enumerate() { sum += c * kernel_at(i, m, x); }
        sum
    }).collect()
}

fn correctness_gate() -> Result<(), String> {
    for log_n in 1..=8u32 {
        let n = 1usize << log_n;
        let m = log_n as usize;
        let (tower, levels_u) = build_split_tower(n)?;
        let levels = to_bf_levels(&levels_u);
        let levels_br = bitrev_alpha_levels(&levels);
        let lambda: Vec<BaseField> = (0..n).map(|i| BaseField::from((17 * i + 3) % P as usize)).collect();
        let lambda_br = bitrev_vec(&lambda);
        let mu = kernel_to_sheared(lambda.clone());
        let mu_br = bitrev_vec(&mu);
        let zero = BaseField::from_u32_unchecked(0);
        let mut old = vec![zero; n];
        let mut shear = vec![zero; n];
        let mut fused = vec![zero; n];
        let mut raw2 = vec![zero; n];
        let mut raw3 = vec![zero; n];
        let mut raw4 = vec![zero; n];
        let mut raw5 = vec![zero; n];
        let mut raw6 = vec![zero; n];
        let mut true_r4 = vec![zero; n];
        let mut true_r8 = vec![zero; n];
        let mut cache_r8 = vec![zero; n];
        kernel_br_1m3a(&lambda_br, &levels_br, &mut old);
        kernel_br_1m2a(&mu_br, &levels_br, &mut shear);
        kernel_br_1m2a_fused3(&mu_br, &levels_br, &mut fused);
        kernel_raw_f2(&mu_br, &levels_br, &mut raw2);
        kernel_raw_f3(&mu_br, &levels_br, &mut raw3);
        kernel_raw_f4(&mu_br, &levels_br, &mut raw4);
        kernel_raw_f5(&mu_br, &levels_br, &mut raw5);
        kernel_raw_f6(&mu_br, &levels_br, &mut raw6);
        kernel_true_r4(&mu_br, &levels_br, &mut true_r4);
        kernel_true_r8(&mu_br, &levels_br, &mut true_r8);
        for b in 1..=m {
            kernel_cache_r8(&mu_br, &levels_br, &mut cache_r8, b);
            if cache_r8 != shear { return Err(format!("cache-r8 b={b} != scalar N={n}")); }
        }
        let input_simd = BaseColumn::from_cpu(&mu_br);
        let mut out_simd = input_simd.clone();
        let mut out_simd_f3 = input_simd.clone();
        kernel_br_1m2a_simd(&input_simd, &levels_br, &mut out_simd);
        kernel_br_1m2a_simd_fused3(&input_simd, &levels_br, &mut out_simd_f3);
        let brute_br = bitrev_vec(&brute_eval(&lambda, &tower[0]));
        if old != brute_br { return Err(format!("1M3A brute fail N={n}")); }
        if shear != brute_br { return Err(format!("1M2A/shear brute fail N={n}")); }
        if fused != brute_br { return Err(format!("1M2A/fused3 brute fail N={n}")); }
        if shear != old { return Err(format!("sheared != original N={n}")); }
        if fused != shear { return Err(format!("fused3 != unfused N={n}")); }
        if raw2 != shear { return Err(format!("raw f2 != scalar N={n}")); }
        if raw3 != shear { return Err(format!("raw f3 != scalar N={n}")); }
        if raw4 != shear { return Err(format!("raw f4 != scalar N={n}")); }
        if raw5 != shear { return Err(format!("raw f5 != scalar N={n}")); }
        if raw6 != shear { return Err(format!("raw f6 != scalar N={n}")); }
        if true_r4 != shear { return Err(format!("true radix4 != scalar N={n}")); }
        if true_r8 != shear { return Err(format!("true radix8 != scalar N={n}")); }
        #[cfg(target_arch = "aarch64")]
        {
            let input_raw: Vec<u32> = mu_br.iter().map(|x| x.0).collect();
            let levels_raw: Vec<Vec<u32>> = levels_br.iter()
                .map(|v| v.iter().map(|x| x.0).collect())
                .collect();
            let mut raw_out = vec![0u32; n];
            kernel_raw_neon_r8_old(&input_raw, &levels_raw, &mut raw_out);
            let got: Vec<BaseField> = raw_out.into_iter()
                .map(|x| BaseField::from_u32_unchecked(raw_canon(x)))
                .collect();
            if got != shear { return Err(format!("raw NEON != scalar N={n}")); }
            let mut tail2_out = vec![0u32; n];
            kernel_raw_neon_r8_tail2(&input_raw, &levels_raw, &mut tail2_out);
            let got_tail2: Vec<BaseField> = tail2_out.into_iter()
                .map(|x| BaseField::from_u32_unchecked(raw_canon(x)))
                .collect();
            if got_tail2 != shear { return Err(format!("raw NEON tail2 != scalar N={n}")); }
        }
        if out_simd.as_slice() != shear.as_slice() { return Err(format!("simd != scalar N={n}")); }
        if out_simd_f3.as_slice() != shear.as_slice() { return Err(format!("simd fused3 != scalar N={n}")); }
    }
    Ok(())
}

fn median_ns(mut xs: Vec<u128>) -> u128 { xs.sort_unstable(); xs[xs.len() / 2] }
fn reps_for(log_n: u32, requested: usize) -> usize {
    if requested != 0 { return requested; }
    match log_n { 0..=14 => 15, 15..=17 => 11, 18..=19 => 7, 20..=21 => 5, _ => 3 }
}
fn time_kernel(
    f: fn(&[BaseField], &[Vec<BaseField>], &mut [BaseField]),
    input: &[BaseField], levels: &[Vec<BaseField>], reps: usize,
) -> u128 {
    let z = BaseField::from_u32_unchecked(0);
    let mut a = vec![z; input.len()];
    for _ in 0..2 { f(input, levels, &mut a); black_box(a.as_ptr()); }
    let mut ts = Vec::with_capacity(reps);
    for _ in 0..reps {
        let t = Instant::now();
        f(input, levels, &mut a);
        ts.push(t.elapsed().as_nanos());
        black_box(a.as_ptr());
    }
    median_ns(ts)
}



fn time_kernel_destructive(
    inplace: fn(&mut [BaseField], &[Vec<BaseField>]),
    input: &[BaseField],
    levels: &[Vec<BaseField>],
    reps: usize,
) -> u128 {
    let mut a = input.to_vec();
    for _ in 0..2 {
        a.copy_from_slice(input);
        inplace(&mut a, levels);
        black_box(a.as_ptr());
    }
    let mut ts = Vec::with_capacity(reps);
    for _ in 0..reps {
        // Restore OUTSIDE the timed region. This measures a destructive-native transform API.
        a.copy_from_slice(input);
        let t = Instant::now();
        inplace(&mut a, levels);
        ts.push(t.elapsed().as_nanos());
        black_box(a.as_ptr());
    }
    median_ns(ts)
}

fn time_kernel_simd(
    input: &BaseColumn,
    levels: &[Vec<BaseField>],
    reps: usize,
) -> u128 {
    let mut a = input.clone();
    for _ in 0..2 {
        kernel_br_1m2a_simd(input, levels, &mut a);
        black_box(a.data.as_ptr());
    }
    let mut ts = Vec::with_capacity(reps);
    for _ in 0..reps {
        let t = Instant::now();
        kernel_br_1m2a_simd(input, levels, &mut a);
        ts.push(t.elapsed().as_nanos());
        black_box(a.data.as_ptr());
    }
    median_ns(ts)
}


fn time_kernel_simd_fused3(
    input: &BaseColumn,
    levels: &[Vec<BaseField>],
    reps: usize,
) -> u128 {
    let mut a = input.clone();
    for _ in 0..2 {
        kernel_br_1m2a_simd_fused3(input, levels, &mut a);
        black_box(a.data.as_ptr());
    }
    let mut ts = Vec::with_capacity(reps);
    for _ in 0..reps {
        let t = Instant::now();
        kernel_br_1m2a_simd_fused3(input, levels, &mut a);
        ts.push(t.elapsed().as_nanos());
        black_box(a.data.as_ptr());
    }
    median_ns(ts)
}


fn time_kernel_simd_true_r8(
    input: &BaseColumn,
    levels: &[Vec<BaseField>],
    reps: usize,
) -> u128 {
    let mut a = input.clone();
    for _ in 0..2 {
        kernel_br_1m2a_simd_true_r8(input, levels, &mut a);
        black_box(a.data.as_ptr());
    }
    let mut ts = Vec::with_capacity(reps);
    for _ in 0..reps {
        let t = Instant::now();
        kernel_br_1m2a_simd_true_r8(input, levels, &mut a);
        ts.push(t.elapsed().as_nanos());
        black_box(a.data.as_ptr());
    }
    median_ns(ts)
}


fn time_stwo_cpu(log_n: u32, reps: usize) -> u128 {
    let n = 1usize << log_n;
    let domain = CanonicCoset::new(log_n).circle_domain();
    let tw = CpuBackend::precompute_twiddles(domain.half_coset);
    let coeffs: Vec<BaseField> = (0..n).map(BaseField::from).collect();
    let poly = CpuCirclePoly::new(coeffs);
    let mut buffer = vec![BaseField::from(0usize); n];
    {
        let e = CpuBackend::evaluate_into(&poly, domain, &tw, buffer);
        buffer = e.values;
        black_box(buffer.as_ptr());
    }
    let mut ts = Vec::with_capacity(reps);
    for _ in 0..reps {
        let t = Instant::now();
        let e = CpuBackend::evaluate_into(&poly, domain, &tw, buffer);
        ts.push(t.elapsed().as_nanos());
        buffer = e.values;
        black_box(buffer.as_ptr());
    }
    median_ns(ts)
}

fn time_stwo_simd(log_n: u32, reps: usize) -> u128 {
    let domain = CanonicCoset::new(log_n).circle_domain();
    let tw = get_twiddle_dbls(domain.half_coset);
    let refs = tw.iter().map(|x| x.as_slice()).collect::<Vec<_>>();
    let values: BaseColumn = (0..domain.size()).map(BaseField::from).collect();
    let mut target = values.clone().data;
    for _ in 0..2 {
        unsafe {
            fft(
                transmute::<*const PackedBaseField, *const u32>(values.data.as_ptr()),
                transmute::<*mut PackedBaseField, *mut u32>(target.as_mut_ptr()),
                black_box(&refs),
                log_n as usize,
            )
        }
        black_box(target.as_ptr());
    }
    let mut ts = Vec::with_capacity(reps);
    for _ in 0..reps {
        let t = Instant::now();
        unsafe {
            fft(
                transmute::<*const PackedBaseField, *const u32>(values.data.as_ptr()),
                transmute::<*mut PackedBaseField, *mut u32>(target.as_mut_ptr()),
                black_box(&refs),
                log_n as usize,
            )
        }
        ts.push(t.elapsed().as_nanos());
        black_box(target.as_ptr());
    }
    median_ns(ts)
}

fn ms(ns: u128) -> f64 { ns as f64 / 1e6 }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    correctness_gate().map_err(|e| format!("CORRECTNESS_GATE_FAIL: {e}"))?;
    let min_log: u32 = env::var("MIN_LOG").ok().and_then(|x| x.parse().ok()).unwrap_or(18);
    let max_log: u32 = env::var("MAX_LOG").ok().and_then(|x| x.parse().ok()).unwrap_or(20);
    let req: usize = env::var("REPS").ok().and_then(|x| x.parse().ok()).unwrap_or(0);
    let out = env::var("BENCH_OUT").unwrap_or_else(|_| "kernel-vs-stwo-v20.csv".into());

    println!("============================================================");
    println!("KERNEL BASIS vs STWO — FUSED NEON LAST-2-LEVEL GATE v0.20.0");
    println!("============================================================");
    println!("FIELD=M31");
    println!("CORRECTNESS=PASS N=2..256: OLD_RAW_NEON = TAIL2_NEON = scalar R8 = original kernel");
    println!("DIAGNOSIS_v0.19=R8 groups balanced; last two scalar levels cost 38.5-41.4%");
    println!("OLD_RAW_NEON=v0.18.2 raw NEON with scalar k=1,k=0 tail");
    println!("TAIL2_NEON=same high schedule; k=1,k=0 fused in one four-element NEON microkernel");
    println!("TAIL2_MEMORY=one 128-bit load + one 128-bit store per four elements");
    println!("REPRESENTATION=raw redundant M31 [0,P]");
    println!("RAYON_NUM_THREADS=1\n");

    #[cfg(not(target_arch = "aarch64"))]
    return Err("v0.20 requires aarch64".into());

    let mut csv = BufWriter::new(File::create(&out)?);
    writeln!(csv, "log_n,n,reps,scalar_r8_ns,old_raw_neon_ns,tail2_neon_ns,stwo_cpu_ns,stwo_simd_ns,old_over_tail2,scalar_over_tail2,tail2_over_scalar,tail2_over_cpu,tail2_over_stwo,old_over_stwo")?;

    println!("logN        N reps scalar_ms oldNEON_ms tail2_ms OLD/T2 SC/T2 T2/SC T2/CPU T2/STWO OLD/STWO");
    println!("---- --------- ---- --------- ---------- -------- ------- ----- ----- ------ ------- --------");

    for log_n in min_log..=max_log {
        let n = 1usize << log_n;
        let reps = reps_for(log_n, req);

        let (_tower, levels_u) = build_split_tower(n)?;
        let levels = to_bf_levels(&levels_u);
        let levels_br = bitrev_alpha_levels(&levels);

        let lambda: Vec<BaseField> = (0..n).map(BaseField::from).collect();
        let mu = kernel_to_sheared(lambda);
        let mu_br = bitrev_vec(&mu);

        let scalar = time_kernel(kernel_true_r8, &mu_br, &levels_br, reps);

        #[cfg(target_arch = "aarch64")]
        let (old_neon, tail2) = {
            let input_raw: Vec<u32> = mu_br.iter().map(|x| x.0).collect();
            let levels_raw: Vec<Vec<u32>> = levels_br.iter()
                .map(|v| v.iter().map(|x| x.0).collect())
                .collect();
            (
                time_kernel_raw_neon_old(&input_raw, &levels_raw, reps),
                time_kernel_raw_neon_tail2(&input_raw, &levels_raw, reps),
            )
        };

        let cpu = time_stwo_cpu(log_n, reps);
        let stwo = time_stwo_simd(log_n, reps);

        let old_t2 = old_neon as f64 / tail2 as f64;
        let sc_t2 = scalar as f64 / tail2 as f64;
        let t2_sc = tail2 as f64 / scalar as f64;
        let t2_cpu = tail2 as f64 / cpu as f64;
        let t2_stwo = tail2 as f64 / stwo as f64;
        let old_stwo = old_neon as f64 / stwo as f64;

        println!("{:>4} {:>9} {:>4} {:>9.3} {:>10.3} {:>8.3} {:>7.3} {:>5.3} {:>5.3} {:>6.3} {:>7.3} {:>8.3}",
            log_n,n,reps,ms(scalar),ms(old_neon),ms(tail2),
            old_t2,sc_t2,t2_sc,t2_cpu,t2_stwo,old_stwo);

        writeln!(csv,"{log_n},{n},{reps},{scalar},{old_neon},{tail2},{cpu},{stwo},{old_t2:.6},{sc_t2:.6},{t2_sc:.6},{t2_cpu:.6},{t2_stwo:.6},{old_stwo:.6}")?;
    }

    csv.flush()?;
    println!("\nRESULT_CSV={out}");
    println!("TARGET: T2/STWO <= 1.00 => kernel matches/beats Stwo SIMD.");
    println!("TAIL_FIX_PASS: OLD/T2 >= 1.20 => final scalar tail was the dominant bottleneck.");
    println!("IF_T2_STWO_0_95_TO_1_05: parity achieved; repeat stability and stop transform tuning.");
    println!("IF_T2_STWO_GT_1_05: profile the new fused tail vs high groups, then optimize only the remaining microkernel.");
    Ok(())
}
