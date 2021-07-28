//! Shared trait and methods for writing integers.

use lexical_util::num::as_cast;

/// Select the back-end.
#[cfg(feature = "compact")]
use crate::compact::Compact;
#[cfg(not(feature = "compact"))]
use crate::decimal::Decimal;
#[cfg(all(not(feature = "compact"), feature = "power-of-two"))]
use crate::radix::Radix;

/// Write integer trait, implemented in terms of the compact back-end.
#[cfg(feature = "compact")]
pub trait WriteInteger: Compact {
    /// Forward write integer parameters to an unoptimized backend.
    /// Preconditions: `value` must be non-negative and unsigned.
    ///
    /// # Safety
    ///
    /// Safe as long as the buffer can hold `FORMATTED_SIZE` elements
    /// (or `FORMATTED_SIZE_DECIMAL` for decimal).
    unsafe fn write_integer<U, const RADIX: u32>(self, buffer: &mut [u8]) -> usize
    where
        U: Compact,
    {
        let value: U = as_cast(self);
        unsafe { value.compact(RADIX, buffer) }
    }
}

/// Write integer trait, implemented in terms of the optimized, decimal back-end.
#[cfg(all(not(feature = "compact"), not(feature = "power-of-two")))]
pub trait WriteInteger: Decimal {
    /// Forward write integer parameters to an optimized backend.
    /// Preconditions: `value` must be non-negative and unsigned.
    ///
    /// # Safety
    ///
    /// Safe as long as the buffer can hold `FORMATTED_SIZE_DECIMAL` elements.
    #[inline]
    unsafe fn write_integer<U, const __: u32>(self, buffer: &mut [u8]) -> usize
    where
        U: Decimal,
    {
        let value: U = as_cast(self);
        unsafe { value.decimal(buffer) }
    }
}

/// Write integer trait, implemented in terms of the optimized, decimal or radix back-end.
#[cfg(all(not(feature = "compact"), feature = "power-of-two"))]
pub trait WriteInteger: Decimal + Radix {
    /// Forward write integer parameters to an optimized backend.
    /// Preconditions: `value` must be non-negative and unsigned.
    ///
    /// # Safety
    ///
    /// Safe as long as the buffer can hold `FORMATTED_SIZE` elements
    /// (or `FORMATTED_SIZE_DECIMAL` for decimal).
    #[inline]
    unsafe fn write_integer<U, const RADIX: u32>(self, buffer: &mut [u8]) -> usize
    where
        U: Decimal + Radix,
    {
        let value: U = as_cast(self);
        if RADIX == 10 {
            unsafe { value.decimal(buffer) }
        } else {
            unsafe { value.radix::<RADIX>(buffer) }
        }
    }
}

macro_rules! write_integer_impl {
    ($($t:ty)*) => ($(
        impl WriteInteger for $t {}
    )*)
}

write_integer_impl! { u8 u16 u32 u64 u128 usize }