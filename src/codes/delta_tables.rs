// THIS FILE HAS BEEN GENERATED WITH THE SCRIPT code_tables_generator.py
// ~~~~~~~~~~~~~~~~~~~ DO NOT MODIFY ~~~~~~~~~~~~~~~~~~~~~~
// Pre-computed constants used to speedup the reading and writing of delta codes
use crate::traits::{BitRead, BitWrite, UpcastableInto, L2M, M2L};
use anyhow::Result;
/// How many bits are needed to read the tables in this
pub const READ_BITS: usize = 5;
/// The len we assign to a code that cannot be decoded through the table
pub const MISSING_VALUE_LEN: u8 = 255;
/// Maximum value writable using the table(s)
pub const WRITE_MAX: u64 = 255;

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
///Table containing the values used to speed up the reading of delta codes
pub const READ_M2L: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
///Table contaings the lens used to speed up the reading of delta codes
pub const READ_LEN_M2L: &[u8] = &[
    255, 255, 255, 255, 255, 255, 255, 255, 4, 4, 4, 4, 5, 5, 5, 5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1,
];
///Table containing the values used to speed up the reading of delta codes
pub const READ_L2M: &[u8] = &[
    0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0, 2, 0, 0, 0, 6, 0,
];
///Table contaings the lens used to speed up the reading of delta codes
pub const READ_LEN_L2M: &[u8] = &[
    255, 1, 4, 1, 255, 1, 5, 1, 255, 1, 4, 1, 255, 1, 5, 1, 255, 1, 4, 1, 255, 1, 5, 1, 255, 1, 4,
    1, 255, 1, 5, 1,
];
///Table used to speed up the writing of delta codes
pub const WRITE_M2L: &[u16] = &[
    1, 4, 5, 12, 13, 14, 15, 32, 33, 34, 35, 36, 37, 38, 39, 80, 81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204,
    205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223,
    448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459, 460, 461, 462, 463, 464, 465, 466,
    467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479, 480, 481, 482, 483, 484, 485,
    486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499, 500, 501, 502, 503, 504,
    505, 506, 507, 508, 509, 510, 511, 1024, 1025, 1026, 1027, 1028, 1029, 1030, 1031, 1032, 1033,
    1034, 1035, 1036, 1037, 1038, 1039, 1040, 1041, 1042, 1043, 1044, 1045, 1046, 1047, 1048, 1049,
    1050, 1051, 1052, 1053, 1054, 1055, 1056, 1057, 1058, 1059, 1060, 1061, 1062, 1063, 1064, 1065,
    1066, 1067, 1068, 1069, 1070, 1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1079, 1080, 1081,
    1082, 1083, 1084, 1085, 1086, 1087, 1088, 1089, 1090, 1091, 1092, 1093, 1094, 1095, 1096, 1097,
    1098, 1099, 1100, 1101, 1102, 1103, 1104, 1105, 1106, 1107, 1108, 1109, 1110, 1111, 1112, 1113,
    1114, 1115, 1116, 1117, 1118, 1119, 1120, 1121, 1122, 1123, 1124, 1125, 1126, 1127, 1128, 1129,
    1130, 1131, 1132, 1133, 1134, 1135, 1136, 1137, 1138, 1139, 1140, 1141, 1142, 1143, 1144, 1145,
    1146, 1147, 1148, 1149, 1150, 1151, 2304,
];
///Table used to speed up the writing of delta codes
pub const WRITE_LEN_M2L: &[u16] = &[
    1, 4, 4, 5, 5, 5, 5, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 15,
];
///Table used to speed up the writing of delta codes
pub const WRITE_L2M: &[u16] = &[
    1, 2, 10, 6, 14, 22, 30, 4, 36, 68, 100, 132, 164, 196, 228, 12, 44, 76, 108, 140, 172, 204,
    236, 268, 300, 332, 364, 396, 428, 460, 492, 20, 52, 84, 116, 148, 180, 212, 244, 276, 308,
    340, 372, 404, 436, 468, 500, 532, 564, 596, 628, 660, 692, 724, 756, 788, 820, 852, 884, 916,
    948, 980, 1012, 28, 60, 92, 124, 156, 188, 220, 252, 284, 316, 348, 380, 412, 444, 476, 508,
    540, 572, 604, 636, 668, 700, 732, 764, 796, 828, 860, 892, 924, 956, 988, 1020, 1052, 1084,
    1116, 1148, 1180, 1212, 1244, 1276, 1308, 1340, 1372, 1404, 1436, 1468, 1500, 1532, 1564, 1596,
    1628, 1660, 1692, 1724, 1756, 1788, 1820, 1852, 1884, 1916, 1948, 1980, 2012, 2044, 8, 136,
    264, 392, 520, 648, 776, 904, 1032, 1160, 1288, 1416, 1544, 1672, 1800, 1928, 2056, 2184, 2312,
    2440, 2568, 2696, 2824, 2952, 3080, 3208, 3336, 3464, 3592, 3720, 3848, 3976, 4104, 4232, 4360,
    4488, 4616, 4744, 4872, 5000, 5128, 5256, 5384, 5512, 5640, 5768, 5896, 6024, 6152, 6280, 6408,
    6536, 6664, 6792, 6920, 7048, 7176, 7304, 7432, 7560, 7688, 7816, 7944, 8072, 8200, 8328, 8456,
    8584, 8712, 8840, 8968, 9096, 9224, 9352, 9480, 9608, 9736, 9864, 9992, 10120, 10248, 10376,
    10504, 10632, 10760, 10888, 11016, 11144, 11272, 11400, 11528, 11656, 11784, 11912, 12040,
    12168, 12296, 12424, 12552, 12680, 12808, 12936, 13064, 13192, 13320, 13448, 13576, 13704,
    13832, 13960, 14088, 14216, 14344, 14472, 14600, 14728, 14856, 14984, 15112, 15240, 15368,
    15496, 15624, 15752, 15880, 16008, 16136, 16264, 24,
];
///Table used to speed up the writing of delta codes
pub const WRITE_LEN_L2M: &[u16] = &[
    1, 4, 4, 5, 5, 5, 5, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 15,
];
///Table used to speed up the skipping of delta codes
pub const LEN: &[u8] = &[
    1, 4, 4, 5, 5, 5, 5, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 15,
];
