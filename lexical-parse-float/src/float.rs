//! Extended helper trait for generic float types.
//!
//! This adapted from the Rust implementation, based on the fast-float-rust
//! implementation, and is similarly subject to an Apache2.0/MIT license.

#![doc(hidden)]

#[cfg(all(not(feature = "std"), feature = "compact"))]
use crate::libm::{powd, powf};
use crate::limits::ExactFloat;
#[cfg(not(feature = "compact"))]
use crate::table::{get_small_f32_power, get_small_f64_power, get_small_int_power};
use lexical_util::num::Float;

/// Helper trait to add more float characteristics for parsing floats.
pub trait RawFloat: Float + ExactFloat {
    // Maximum mantissa for the fast-path (`1 << 53` for f64).
    const MAX_MANTISSA_FAST_PATH: u64 = 2_u64 << Self::MANTISSA_SIZE;

    /// Minimum exponent that for a fast path case, or `-⌊(MANTISSA_SIZE+1)/log2(r)⌋`
    /// where `r` is the radix with powers-of-two removed.
    #[inline(always)]
    fn min_exponent_fast_path(radix: u32) -> i64 {
        Self::exponent_limit(radix).0
    }

    /// Maximum exponent that for a fast path case, or `⌊(MANTISSA_SIZE+1)/log2(r)⌋`
    /// where `r` is the radix with powers-of-two removed.
    #[inline(always)]
    fn max_exponent_fast_path(radix: u32) -> i64 {
        Self::exponent_limit(radix).1
    }

    // Maximum exponent that can be represented for a disguised-fast path case.
    // This is `max_exponent_fast_path(radix) + ⌊(MANTISSA_SIZE+1)/log2(radix)⌋`
    #[inline(always)]
    fn max_exponent_disguised_fast_path(radix: u32) -> i64 {
        Self::max_exponent_fast_path(radix) + Self::mantissa_limit(radix)
    }

    /// Get a small power-of-radix for fast-path multiplication.
    ///
    /// # Safety
    ///
    /// Safe as long as the exponent is smaller than the table size.
    unsafe fn pow_fast_path(exponent: usize, radix: u32) -> Self;

    /// Get a small, integral power-of-radix for fast-path multiplication.
    ///
    /// # Safety
    ///
    /// Safe as long as the exponent is smaller than the table size.
    #[inline(always)]
    unsafe fn int_pow_fast_path(exponent: usize, radix: u32) -> u64 {
        // SAFETY: safe as long as the exponent is smaller than the radix table.
        #[cfg(not(feature = "compact"))]
        return unsafe { get_small_int_power(exponent, radix) };

        #[cfg(feature = "compact")]
        return (radix as u64).wrapping_pow(exponent as u32);
    }
}

impl RawFloat for f32 {
    #[inline(always)]
    unsafe fn pow_fast_path(exponent: usize, radix: u32) -> Self {
        // SAFETY: safe as long as the exponent is smaller than the radix table.
        #[cfg(not(feature = "compact"))]
        return unsafe { get_small_f32_power(exponent, radix) };

        #[cfg(feature = "compact")]
        return powf(radix as f32, exponent as f32);
    }
}

impl RawFloat for f64 {
    #[inline(always)]
    unsafe fn pow_fast_path(exponent: usize, radix: u32) -> Self {
        // SAFETY: safe as long as the exponent is smaller than the radix table.
        #[cfg(not(feature = "compact"))]
        return unsafe { get_small_f64_power(exponent, radix) };

        #[cfg(feature = "compact")]
        return powd(radix as f64, exponent as f64);
    }
}

/// Helper trait to add more float characteristics for the Eisel-Lemire algorithm.
pub trait LemireFloat: RawFloat {
    // Round-to-even only happens for negative values of q
    // when q ≥ −4 in the 64-bit case and when q ≥ −17 in
    // the 32-bitcase.
    //
    // When q ≥ 0,we have that 5^q ≤ 2m+1. In the 64-bit case,we
    // have 5^q ≤ 2m+1 ≤ 2^54 or q ≤ 23. In the 32-bit case,we have
    // 5^q ≤ 2m+1 ≤ 2^25 or q ≤ 10.
    //
    // When q < 0, we have w ≥ (2m+1)×5^−q. We must have that w < 2^64
    // so (2m+1)×5^−q < 2^64. We have that 2m+1 > 2^53 (64-bit case)
    // or 2m+1 > 2^24 (32-bit case). Hence,we must have 2^53×5^−q < 2^64
    // (64-bit) and 2^24×5^−q < 2^64 (32-bit). Hence we have 5^−q < 2^11
    // or q ≥ −4 (64-bit case) and 5^−q < 2^40 or q ≥ −17 (32-bitcase).
    //
    // Thus we have that we only need to round ties to even when
    // we have that q ∈ [−4,23](in the 64-bit case) or q∈[−17,10]
    // (in the 32-bit case). In both cases,the power of five(5^|q|)
    // fits in a 64-bit word.
    const MIN_EXPONENT_ROUND_TO_EVEN: i32;
    const MAX_EXPONENT_ROUND_TO_EVEN: i32;

    /// Minimum normal exponent value `-(1 << (EXPONENT_SIZE - 1)) + 1`.
    const MINIMUM_EXPONENT: i32;

    /// Smallest decimal exponent for a non-zero value.
    const SMALLEST_POWER_OF_TEN: i32;

    /// Largest decimal exponent for a non-infinite value.
    const LARGEST_POWER_OF_TEN: i32;
}

impl LemireFloat for f32 {
    const MIN_EXPONENT_ROUND_TO_EVEN: i32 = -17;
    const MAX_EXPONENT_ROUND_TO_EVEN: i32 = 10;
    const MINIMUM_EXPONENT: i32 = -127;
    const SMALLEST_POWER_OF_TEN: i32 = -65;
    const LARGEST_POWER_OF_TEN: i32 = 38;
}

impl LemireFloat for f64 {
    const MIN_EXPONENT_ROUND_TO_EVEN: i32 = -4;
    const MAX_EXPONENT_ROUND_TO_EVEN: i32 = 23;
    const MINIMUM_EXPONENT: i32 = -1023;
    const SMALLEST_POWER_OF_TEN: i32 = -342;
    const LARGEST_POWER_OF_TEN: i32 = 308;
}

#[cfg(all(feature = "std", feature = "compact"))]
pub fn powf(x: f32, y: f32) -> f32 {
    x.powf(y)
}

#[cfg(all(feature = "std", feature = "compact"))]
pub fn powd(x: f64, y: f64) -> f64 {
    x.powf(y)
}