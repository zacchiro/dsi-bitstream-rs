/*
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

#![doc = include_str!("../README.md")]
// No warnings
//#![deny(warnings)]

// the code must be safe and shouldn't ever panic to be relayable
#![deny(clippy::todo)]
#![deny(clippy::panic)]
#![deny(clippy::panicking_unwrap)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
// for now we don't need any new feature but we might remove this in the future
#![deny(unstable_features)]
// no dead code
//#![deny(dead_code)]
#![deny(trivial_casts)]
#![deny(unconditional_recursion)]
#![deny(clippy::empty_loop)]
#![deny(unreachable_code)]
#![deny(unreachable_pub)]
#![deny(unreachable_patterns)]
#![deny(unused_macro_rules)]
//#![deny(unused_results)]

// the code must be documented and everything should have a debug print implementation
//#![deny(unused_doc_comments)]
//#![deny(missing_docs)]
//#![deny(clippy::missing_errors_doc)]
//#![deny(clippy::missing_panics_doc)]
//#![deny(clippy::missing_safety_doc)]
//#![deny(clippy::missing_doc_code_examples)]
//#![deny(clippy::missing_crate_level_docs)]
//#![deny(clippy::missing_docs_in_private_items)]
//#![deny(missing_debug_implementations)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod backends;
pub mod codes;
pub mod traits;

/// Prelude module to import everything from this crate
pub mod prelude {
    pub use crate::backends::*;
    pub use crate::codes::*;
    pub use crate::traits::*;
}
