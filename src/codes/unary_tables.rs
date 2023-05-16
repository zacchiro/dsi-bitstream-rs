// THIS FILE HAS BEEN GENERATED WITH THE SCRIPT code_tables_generator.py
// ~~~~~~~~~~~~~~~~~~~ DO NOT MODIFY ~~~~~~~~~~~~~~~~~~~~~~
// Pre-computed constants used to speedup the reading and writing of unary codes
use crate::traits::{BitRead, BitWrite, UpcastableInto, L2M, M2L};
use anyhow::Result;
/// How many bits are needed to read the tables in this
pub const READ_BITS: usize = 5;
/// The len we assign to a code that cannot be decoded through the table
pub const MISSING_VALUE_LEN: u8 = 255;
/// Maximum value writable using the table(s)
pub const WRITE_MAX: u64 = 63;

#[inline(always)]
/// Autogenerated function to lookup a read table, if the result is `Some` the
/// value was found, otherwise we were not able to decode the value and you
/// should fallback to the default implementation
///
/// # Errors
/// This function errors if it wasn't able to skip_bits
pub fn read_table_l2m<B: BitRead<L2M>>(backend: &mut B) -> Result<Option<u64>> {
    if let Ok(idx) = backend.peek_bits(READ_BITS) {
        let idx: u64 = idx.upcast();
        let len = READ_LEN_L2M[idx as usize];
        if len != MISSING_VALUE_LEN {
            backend.skip_bits_after_table_lookup(len as usize)?;
            return Ok(Some(READ_L2M[idx as usize] as u64));
        }
    }
    Ok(None)
}

#[inline(always)]
/// Autogenerated function to lookup a read table, if the result is `Some` the
/// value was found, otherwise we were not able to decode the value and you
/// should fallback to the default implementation
///
/// # Errors
/// This function errors if it wasn't able to skip_bits
pub fn write_table_l2m<B: BitWrite<L2M>>(backend: &mut B, value: u64) -> Result<bool> {
    Ok(if let Some(bits) = WRITE_L2M.get(value as usize) {
        backend.write_bits(*bits as u64, WRITE_LEN_L2M[value as usize] as usize)?;
        true
    } else {
        false
    })
}

#[inline(always)]
/// Autogenerated function to lookup a read table, if the result is `Some` the
/// value was found, otherwise we were not able to decode the value and you
/// should fallback to the default implementation
///
/// # Errors
/// This function errors if it wasn't able to skip_bits
pub fn read_table_m2l<B: BitRead<M2L>>(backend: &mut B) -> Result<Option<u64>> {
    if let Ok(idx) = backend.peek_bits(READ_BITS) {
        let idx: u64 = idx.upcast();
        let len = READ_LEN_M2L[idx as usize];
        if len != MISSING_VALUE_LEN {
            backend.skip_bits_after_table_lookup(len as usize)?;
            return Ok(Some(READ_M2L[idx as usize] as u64));
        }
    }
    Ok(None)
}

#[inline(always)]
/// Autogenerated function to lookup a read table, if the result is `Some` the
/// value was found, otherwise we were not able to decode the value and you
/// should fallback to the default implementation
///
/// # Errors
/// This function errors if it wasn't able to skip_bits
pub fn write_table_m2l<B: BitWrite<M2L>>(backend: &mut B, value: u64) -> Result<bool> {
    Ok(if let Some(bits) = WRITE_M2L.get(value as usize) {
        backend.write_bits(*bits as u64, WRITE_LEN_M2L[value as usize] as usize)?;
        true
    } else {
        false
    })
}
///Table containing the values used to speed up the reading of unary codes
pub const READ_M2L: &[u8] = &[
    0, 4, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
///Table contaings the lens used to speed up the reading of unary codes
pub const READ_LEN_M2L: &[u8] = &[
    255, 5, 4, 4, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1,
];
///Table containing the values used to speed up the reading of unary codes
pub const READ_L2M: &[u8] = &[
    0, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0, 4, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 2, 0, 1, 0,
];
///Table contaings the lens used to speed up the reading of unary codes
pub const READ_LEN_L2M: &[u8] = &[
    255, 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2,
    1,
];
///Table used to speed up the writing of unary codes
pub const WRITE_M2L: &[u64] = &[
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];
///Table used to speed up the writing of unary codes
pub const WRITE_LEN_M2L: &[u64] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
];
///Table used to speed up the writing of unary codes
pub const WRITE_L2M: &[u64] = &[
    1,
    2,
    4,
    8,
    16,
    32,
    64,
    128,
    256,
    512,
    1024,
    2048,
    4096,
    8192,
    16384,
    32768,
    65536,
    131072,
    262144,
    524288,
    1048576,
    2097152,
    4194304,
    8388608,
    16777216,
    33554432,
    67108864,
    134217728,
    268435456,
    536870912,
    1073741824,
    2147483648,
    4294967296,
    8589934592,
    17179869184,
    34359738368,
    68719476736,
    137438953472,
    274877906944,
    549755813888,
    1099511627776,
    2199023255552,
    4398046511104,
    8796093022208,
    17592186044416,
    35184372088832,
    70368744177664,
    140737488355328,
    281474976710656,
    562949953421312,
    1125899906842624,
    2251799813685248,
    4503599627370496,
    9007199254740992,
    18014398509481984,
    36028797018963968,
    72057594037927936,
    144115188075855872,
    288230376151711744,
    576460752303423488,
    1152921504606846976,
    2305843009213693952,
    4611686018427387904,
    9223372036854775808,
];
///Table used to speed up the writing of unary codes
pub const WRITE_LEN_L2M: &[u64] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
];
///Table used to speed up the skipping of unary codes
pub const LEN: &[u8] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
];
