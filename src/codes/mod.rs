/*
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! This modules contains all the logic to read and write codes. While it's used
//! by webgraph it's not a part of webgraph. In the future we should move this
//! to its own crate, while we decide on the logistic of where to put it,
//! it will stay here to go on with the developement of the library.
//!
//! **The convention is to read bits from the MSB to the LSB (M2L) of each byte.**
//!
//! #### Example:
//! The following stream of bits, to be read from left to right, from top to
//! bottom:
//! ```text
//! 01110110 01100000 11110001 11001101 10011111 10110101 01000011 00000000
//! 10000110 10011011 01110011 11111001 11100110 01100011 00101000 01110000
//! ```
//! is equivalent to the following stream of bytes:
//! ```text
//! M2L
//! 76 60 f1 cd 9f b5 43 00
//! 86 9b 73 f9 e6 63 28 70
//!
//! L2M
//! 6e 06 8f b3 f9 ad c2 00
//! 61 d9 ce 9f 67 c6 14 0e
//! ```
//! In code:
//! ```
//! use dsi_bitstream::prelude::*;
//! // file data
//! let data_m2l: [u8; 16] = [
//!     0x76, 0x60, 0xf1, 0xcd, 0x9f, 0xb5, 0x43, 0x00,
//!     0x86, 0x9b, 0x73, 0xf9, 0xe6, 0x63, 0x28, 0x70,
//! ];
//! // Read data as native endianess [`u64`]s, we can't just do a
//! // transmute because we have no guarantees on the alignement of data
//! let words_m2l = data_m2l.chunks(8)
//!     .map(|data| u64::from_ne_bytes(data.try_into().unwrap()))
//!     .collect::<Vec<_>>();
//!
//! let mut bitstream_m2l = <UnbufferedBitStreamRead<M2L, _>>::new(
//!     MemWordRead::new(&words_m2l)
//! );
//! assert_eq!(bitstream_m2l.read_bits(8).unwrap(), 0b0111_0110);
//! assert_eq!(bitstream_m2l.read_bits(4).unwrap(), 0b0110);
//! assert_eq!(bitstream_m2l.read_bits(4).unwrap(), 0b0000);
//! assert_eq!(bitstream_m2l.read_bits(10).unwrap(), 0b1111_0001_11);
//! assert_eq!(bitstream_m2l.read_bits(8).unwrap(), 0b00_1101_10);
//! assert_eq!(bitstream_m2l.read_bits(38).unwrap(), 0b01_1111_1011_0101_0100_0011_0000_0000_1000_0110);
//!
//! bitstream_m2l.seek_bit(0); // rewind the stream
//! assert_eq!(bitstream_m2l.read_bits(8).unwrap(), 0b0111_0110);
//! bitstream_m2l.seek_bit(0); // rewind the stream
//!
//! assert_eq!(bitstream_m2l.read_unary::<true>().unwrap(), 1);
//! assert_eq!(bitstream_m2l.read_unary::<true>().unwrap(), 0);
//! assert_eq!(bitstream_m2l.read_unary::<true>().unwrap(), 0);
//! assert_eq!(bitstream_m2l.read_unary::<true>().unwrap(), 1);
//! assert_eq!(bitstream_m2l.read_unary::<true>().unwrap(), 0);
//! assert_eq!(bitstream_m2l.read_unary::<true>().unwrap(), 2);
//! assert_eq!(bitstream_m2l.read_unary::<true>().unwrap(), 0);
//! assert_eq!(bitstream_m2l.read_unary::<true>().unwrap(), 5);
//! ```

mod gamma;

pub use gamma::{len_gamma, GammaRead, GammaWrite};

mod delta;
pub use delta::{len_delta, DeltaRead, DeltaWrite};

mod minimal_binary;
pub use minimal_binary::{len_minimal_binary, MinimalBinaryRead, MinimalBinaryWrite};

mod zeta;
pub use zeta::{len_zeta, ZetaRead, ZetaWrite};

pub mod delta_tables;
pub mod gamma_tables;
pub mod unary_tables;
pub mod zeta_tables;

#[must_use]
#[inline]
/// Return how long the unary code for `value` will be
///
/// `USE_TABLE` enables or disables the use of pre-computed tables
/// for decoding
pub fn len_unary<const USE_TABLE: bool>(value: u64) -> usize {
    // we can use the table but it's not useful at all
    // I implemented if for consistency with all the other codes
    if USE_TABLE {
        if let Some(idx) = unary_tables::LEN.get(value as usize) {
            return *idx as usize;
        }
    }
    (value + 1) as usize
}

#[inline(always)]
/// Return the floor of the base 2 logarithm of `value`,
/// which must be nonzero.
pub fn fast_floor_log2(value: u64) -> u32 {
    debug_assert!(value > 0, "log2(0) is undefined");
    63 - value.leading_zeros()
}
