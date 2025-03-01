/*
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::codes::unary_tables;
use crate::traits::*;
use anyhow::{bail, Result};

// I'm not really happy about implementing it over a seekable stream instead of
// a slice but this way is more general and I checked that the compiler generate
// decent code.

/// An impementation of [`BitRead`] on a Seekable word stream [`WordRead`]
/// + [`WordStream`]
#[derive(Debug, Clone)]
pub struct UnbufferedBitStreamRead<BO: BitOrder, WR> {
    /// The stream which we will read words from
    data: WR,
    /// The index of the current bit we are ate
    bit_idx: usize,
    /// Make the compiler happy
    _marker: core::marker::PhantomData<BO>,
}

impl<BO: BitOrder, WR> UnbufferedBitStreamRead<BO, WR> {
    /// Create a new BitStreamRead on a generig WordRead
    pub fn new(data: WR) -> Self {
        Self {
            data,
            bit_idx: 0,
            _marker: core::marker::PhantomData::default(),
        }
    }
}

impl<WR: WordRead<Word = u64> + WordStream> BitRead<M2L> for UnbufferedBitStreamRead<M2L, WR> {
    type PeekType = u32;

    #[inline]
    fn skip_bits(&mut self, n_bits: usize) -> Result<()> {
        self.bit_idx += n_bits;
        Ok(())
    }

    #[inline]
    fn read_bits(&mut self, n_bits: usize) -> Result<u64> {
        if n_bits > 64 {
            bail!(
                "The n of bits to read has to be in [0, 64] and {} is not.",
                n_bits
            );
        }
        if n_bits == 0 {
            return Ok(0);
        }
        self.data.set_position(self.bit_idx / 64)?;
        let in_word_offset = self.bit_idx % 64;

        let res = if (in_word_offset + n_bits) <= 64 {
            // single word access
            let word = self.data.read_next_word()?.to_be();
            (word << in_word_offset) >> (64 - n_bits)
        } else {
            // double word access
            let high_word = self.data.read_next_word()?.to_be();
            let low_word = self.data.read_next_word()?.to_be();
            let shamt1 = 64 - n_bits;
            let shamt2 = 128 - in_word_offset - n_bits;
            ((high_word << in_word_offset) >> shamt1) | (low_word >> shamt2)
        };
        self.bit_idx += n_bits;
        Ok(res)
    }

    #[inline]
    fn peek_bits(&mut self, n_bits: usize) -> Result<u32> {
        if n_bits > 32 {
            bail!(
                "The n of bits to peek has to be in [0, 32] and {} is not.",
                n_bits
            );
        }
        if n_bits == 0 {
            return Ok(0);
        }
        self.data.set_position(self.bit_idx / 64)?;
        let in_word_offset = self.bit_idx % 64;

        let res = if (in_word_offset + n_bits) <= 64 {
            // single word access
            let word = self.data.read_next_word()?.to_be();
            (word << in_word_offset) >> (64 - n_bits)
        } else {
            // double word access
            let high_word = self.data.read_next_word()?.to_be();
            let low_word = self.data.read_next_word()?.to_be();
            let shamt1 = 64 - n_bits;
            let shamt2 = 128 - in_word_offset - n_bits;
            ((high_word << in_word_offset) >> shamt1) | (low_word >> shamt2)
        };
        Ok(res as u32)
    }

    #[inline]
    fn read_unary<const USE_TABLE: bool>(&mut self) -> Result<u64> {
        if USE_TABLE {
            if let Some(res) = unary_tables::read_table_m2l(self)? {
                return Ok(res);
            }
        }
        self.data.set_position(self.bit_idx / 64)?;
        let in_word_offset = self.bit_idx % 64;
        let mut bits_in_word = 64 - in_word_offset;
        let mut total = 0;

        let mut word = self.data.read_next_word()?.to_be();
        word <<= in_word_offset;
        loop {
            let zeros = word.leading_zeros() as usize;
            // the unary code fits in the word
            if zeros < bits_in_word {
                self.bit_idx += total + zeros + 1;
                return Ok(total as u64 + zeros as u64);
            }
            total += bits_in_word;
            bits_in_word = 64;
            word = self.data.read_next_word()?.to_be();
        }
    }
}

impl<WR: WordStream> BitSeek for UnbufferedBitStreamRead<L2M, WR> {
    fn get_position(&self) -> usize {
        self.bit_idx
    }

    fn seek_bit(&mut self, bit_index: usize) -> Result<()> {
        self.bit_idx = bit_index;
        Ok(())
    }
}

impl<WR: WordStream> BitSeek for UnbufferedBitStreamRead<M2L, WR> {
    fn get_position(&self) -> usize {
        self.bit_idx
    }

    fn seek_bit(&mut self, bit_index: usize) -> Result<()> {
        self.bit_idx = bit_index;
        Ok(())
    }
}

impl<WR: WordRead<Word = u64> + WordStream> BitRead<L2M> for UnbufferedBitStreamRead<L2M, WR> {
    type PeekType = u32;
    #[inline]
    fn skip_bits(&mut self, n_bits: usize) -> Result<()> {
        self.bit_idx += n_bits;
        Ok(())
    }

    #[inline]
    fn read_bits(&mut self, n_bits: usize) -> Result<u64> {
        if n_bits > 64 {
            bail!(
                "The n of bits to read has to be in [0, 64] and {} is not.",
                n_bits
            );
        }
        if n_bits == 0 {
            return Ok(0);
        }
        self.data.set_position(self.bit_idx / 64)?;
        let in_word_offset = self.bit_idx % 64;

        let res = if (in_word_offset + n_bits) <= 64 {
            // single word access
            let word = self.data.read_next_word()?.to_le();
            let shamt = 64 - n_bits;
            (word << (shamt - in_word_offset)) >> shamt
        } else {
            // double word access
            let low_word = self.data.read_next_word()?.to_le();
            let high_word = self.data.read_next_word()?.to_le();
            let shamt1 = 128 - in_word_offset - n_bits;
            let shamt2 = 64 - n_bits;
            ((high_word << shamt1) >> shamt2) | (low_word >> in_word_offset)
        };
        self.bit_idx += n_bits;
        Ok(res)
    }

    #[inline]
    fn peek_bits(&mut self, n_bits: usize) -> Result<u32> {
        if n_bits > 32 {
            bail!(
                "The n of bits to peek has to be in [0, 32] and {} is not.",
                n_bits
            );
        }
        if n_bits == 0 {
            return Ok(0);
        }
        self.data.set_position(self.bit_idx / 64)?;
        let in_word_offset = self.bit_idx % 64;

        let res = if (in_word_offset + n_bits) <= 64 {
            // single word access
            let word = self.data.read_next_word()?.to_le();
            let shamt = 64 - n_bits;
            (word << (shamt - in_word_offset)) >> shamt
        } else {
            // double word access
            let low_word = self.data.read_next_word()?.to_le();
            let high_word = self.data.read_next_word()?.to_le();
            let shamt1 = 128 - in_word_offset - n_bits;
            let shamt2 = 64 - n_bits;
            ((high_word << shamt1) >> shamt2) | (low_word >> in_word_offset)
        };
        Ok(res as u32)
    }

    #[inline]
    fn read_unary<const USE_TABLE: bool>(&mut self) -> Result<u64> {
        if USE_TABLE {
            if let Some(res) = unary_tables::read_table_l2m(self)? {
                return Ok(res);
            }
        }
        self.data.set_position(self.bit_idx / 64)?;
        let in_word_offset = self.bit_idx % 64;
        let mut bits_in_word = 64 - in_word_offset;
        let mut total = 0;

        let mut word = self.data.read_next_word()?.to_le();
        word >>= in_word_offset;
        loop {
            let zeros = word.trailing_zeros() as usize;
            // the unary code fits in the word
            if zeros < bits_in_word {
                self.bit_idx += total + zeros + 1;
                return Ok(total as u64 + zeros as u64);
            }
            total += bits_in_word;
            bits_in_word = 64;
            word = self.data.read_next_word()?.to_le();
        }
    }
}
