/*
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! # Elias Gamma
//! Optimal for Zipf of exponent 2
//! Elias’ γ universal coding of x ∈ N+ is obtained by representing x in binary
//! preceded by a unary representation of its length (minus one).
//! More precisely, to represent x we write in unary floor(log(x)) and then in
//! binary x - 2^ceil(log(x)) (on floor(log(x)) bits)
//!

use super::{fast_floor_log2, gamma_tables};
use crate::traits::*;
use anyhow::Result;

/// Returns how long the gamma code for `value` will be
///
/// `USE_TABLE` enables or disables the use of pre-computed tables
/// for decoding
#[must_use]
#[inline]
pub fn len_gamma<const USE_TABLE: bool>(mut value: u64) -> usize {
    if USE_TABLE {
        if let Some(idx) = gamma_tables::LEN.get(value as usize) {
            return *idx as usize;
        }
    }
    value += 1;
    let number_of_blocks_to_write = value.trailing_zeros();
    2 * number_of_blocks_to_write as usize + 1
}

/// Trait for objects that can read Gamma codes
pub trait GammaRead<BO: BitOrder>: BitRead<BO> {
    /// Read a gamma code from the stream.
    ///
    /// `USE_TABLE` enables or disables the use of pre-computed tables
    /// for decoding
    ///
    /// # Errors
    /// This function fails only if the BitRead backend has problems reading
    /// bits, as when the stream ended unexpectedly
    fn read_gamma<const USE_TABLE: bool>(&mut self) -> Result<u64>;
}

/// Common part of the M2L and L2M impl
///
/// # Errors
/// Forward `read_unary` and `read_bits` errors.
#[inline(always)]
fn default_read_gamma<BO: BitOrder, B: BitRead<BO>>(backend: &mut B) -> Result<u64> {
    let len = backend.read_unary::<false>()?;
    debug_assert!(len <= 64);
    Ok(backend.read_bits(len as usize)? + (1 << len) - 1)
}

impl<B: BitRead<M2L>> GammaRead<M2L> for B {
    #[inline]
    fn read_gamma<const USE_TABLE: bool>(&mut self) -> Result<u64> {
        if USE_TABLE {
            if let Some(res) = gamma_tables::read_table_m2l(self)? {
                return Ok(res);
            }
        }
        default_read_gamma(self)
    }
}
impl<B: BitRead<L2M>> GammaRead<L2M> for B {
    #[inline]
    fn read_gamma<const USE_TABLE: bool>(&mut self) -> Result<u64> {
        if USE_TABLE {
            if let Some(res) = gamma_tables::read_table_l2m(self)? {
                return Ok(res);
            }
        }
        default_read_gamma(self)
    }
}

/// Trait for objects that can write Gamma codes
pub trait GammaWrite<BO: BitOrder>: BitWrite<BO> {
    /// Write a value on the stream
    ///
    /// `USE_TABLE` enables or disables the use of pre-computed tables
    /// for decoding
    ///
    /// # Errors
    /// This function fails only if the BitWrite backend has problems writing
    /// bits, as when the stream ended unexpectedly
    fn write_gamma<const USE_TABLE: bool>(&mut self, value: u64) -> Result<()>;
}

impl<B: BitWrite<M2L>> GammaWrite<M2L> for B {
    #[inline]
    fn write_gamma<const USE_TABLE: bool>(&mut self, value: u64) -> Result<()> {
        if USE_TABLE {
            if gamma_tables::write_table_m2l(self, value)? {
                return Ok(());
            }
        }
        default_write_gamma(self, value)
    }
}
impl<B: BitWrite<L2M>> GammaWrite<L2M> for B {
    #[inline]
    fn write_gamma<const USE_TABLE: bool>(&mut self, value: u64) -> Result<()> {
        if USE_TABLE {
            if gamma_tables::write_table_l2m(self, value)? {
                return Ok(());
            }
        }
        default_write_gamma(self, value)
    }
}

/// Common part of the M2L and L2M impl
///
/// # Errors
/// Forward `read_unary` and `read_bits` errors.
#[inline(always)]
fn default_write_gamma<BO: BitOrder, B: BitWrite<BO>>(
    backend: &mut B,
    mut value: u64,
) -> Result<()> {
    value += 1;
    let number_of_bits_to_write = fast_floor_log2(value);
    debug_assert!(number_of_bits_to_write <= u8::MAX as _);
    // remove the most significant 1
    let short_value = value - (1 << number_of_bits_to_write);
    // Write the code
    backend.write_unary::<false>(number_of_bits_to_write as _)?;
    backend.write_bits(short_value, number_of_bits_to_write as usize)?;
    Ok(())
}
