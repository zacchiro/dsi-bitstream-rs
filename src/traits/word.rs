/*
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 * SPDX-FileCopyrightText: 2023 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use core::fmt::{Binary, Debug, Display, LowerHex};
use core::ops::*;

/// Trait with all the common operations that we need for a generic word
/// of memory
pub trait Word:
    Sized
    + Send
    + Sync
    + Debug
    + Display
    + LowerHex
    + Binary
    + Default
    + Clone
    + Copy
    + PartialOrd
    + Ord
    + PartialEq
    + Eq
    + Add<Output = Self>
    + AddAssign<Self>
    + BitAnd<Output = Self>
    + BitAndAssign<Self>
    + BitOr<Output = Self>
    + BitOrAssign<Self>
    + BitXor<Output = Self>
    + BitXorAssign<Self>
    + Div<Output = Self>
    + DivAssign<Self>
    + Mul<Output = Self>
    + MulAssign<Self>
    + Not<Output = Self>
    + Rem<Output = Self>
    + RemAssign<Self>
    + Shl<Output = Self>
    + ShlAssign<Self>
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<Output = Self>
    + ShrAssign<Self>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + Sub<Output = Self>
    + SubAssign<Self>
{
    /// Number of bits in the word
    const BITS: usize;
    /// Number of bytes in the word
    const BYTES: usize;
    /// The byte array form of the value = `[u8; Self::BYTES]`
    type BytesForm: AsRef<[u8]> + Copy;
    /// Zero represented by `Self`
    const ZERO: Self;
    /// One represented by `Self`
    const ONE: Self;
    /// Minimum value represented by `Self`
    const MIN: Self;
    /// Maximum value represented by `Self`
    const MAX: Self;

    /// Converts self to big endian from the target’s endianness.
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    fn to_be(self) -> Self;

    /// Converts self to little endian from the target’s endianness.
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    fn to_le(self) -> Self;

    /// Create a native endian integer value from its representation as a byte
    /// array in big endian.
    fn from_be_bytes(bytes: Self::BytesForm) -> Self;

    /// Create a native endian integer value from its representation as a byte
    /// array in little endian.
    fn from_le_bytes(bytes: Self::BytesForm) -> Self;

    /// Create a native endian integer value from its memory representation as
    /// a byte array in native endianness.
    /// As the target platform’s native endianness is used, portable code likely
    /// wants to use from_be_bytes or from_le_bytes, as appropriate instead.
    fn from_ne_bytes(bytes: Self::BytesForm) -> Self;

    /// Return the memory representation of this integer as a byte array in
    /// big-endian (network) byte order.
    fn to_be_bytes(self) -> Self::BytesForm;

    /// Return the memory representation of this integer as a byte array in
    /// little-endian byte order.
    fn to_le_bytes(self) -> Self::BytesForm;

    /// Return the memory representation of this integer as a byte array in
    /// native byte order.
    /// As the target platform’s native endianness is used, portable code should
    /// use to_be_bytes or to_le_bytes, as appropriate, instead.
    fn to_ne_bytes(self) -> Self::BytesForm;

    /// Returns the number of leading ones in the binary representation of self.
    fn leading_ones(self) -> usize;

    /// Returns the number of trailing zeros in the binary representation of self.
    fn leading_zeros(self) -> usize;

    /// Returns the number of trailing ones in the binary representation of self.
    fn trailing_ones(self) -> usize;

    /// Returns the number of trailing zeros in the binary representation of self.
    fn trailing_zeros(self) -> usize;

    /// Panic-free bitwise shift-left; yields self << mask(rhs), where mask
    /// removes any high-order bits of rhs that would cause the shift to exceed
    /// the bitwidth of the type.
    /// Note that this is not the same as a rotate-left; the RHS of a wrapping
    /// shift-left is restricted to the range of the type, rather than the bits
    /// shifted out of the LHS being returned to the other end. The primitive
    /// integer types all implement a rotate_left function, which may be what
    /// you want instead.
    fn wrapping_shl(self, rhs: usize) -> Self;
}

macro_rules! impl_word {
    ($($ty:ty),*) => {$(

impl Word for $ty {
    const BITS: usize = <$ty>::BITS as _;
    const BYTES: usize = core::mem::size_of::<$ty>() as _;
    type BytesForm = [u8; core::mem::size_of::<$ty>()];
    const MIN: Self = <$ty>::MIN as _;
    const MAX: Self = <$ty>::MAX as _;
    const ZERO: Self = 0;
    const ONE: Self = 1;

    #[inline(always)]
    fn to_be(self) -> Self{self.to_be()}
    #[inline(always)]
    fn to_le(self) -> Self{self.to_le()}
    #[inline(always)]
    fn from_be_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_be_bytes(bytes)}
    #[inline(always)]
    fn from_le_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_le_bytes(bytes)}
    #[inline(always)]
    fn from_ne_bytes(bytes: Self::BytesForm) -> Self {<$ty>::from_ne_bytes(bytes)}
    #[inline(always)]
    fn to_be_bytes(self) -> Self::BytesForm{self.to_be_bytes()}
    #[inline(always)]
    fn to_le_bytes(self) -> Self::BytesForm{self.to_le_bytes()}
    #[inline(always)]
    fn to_ne_bytes(self) -> Self::BytesForm{self.to_ne_bytes()}
    #[inline(always)]
    fn leading_ones(self) -> usize {self.leading_ones() as usize}
    #[inline(always)]
    fn leading_zeros(self) -> usize {self.leading_zeros() as usize}
    #[inline(always)]
    fn trailing_ones(self) -> usize {self.trailing_ones() as usize}
    #[inline(always)]
    fn trailing_zeros(self) -> usize{self.trailing_zeros() as usize}
    #[inline(always)]
    fn wrapping_shl(self, exp: usize) -> Self { self.wrapping_shl(exp as _)}
}

    )*};
}

impl_word!(u8, u16, u32, u64, u128, usize);
