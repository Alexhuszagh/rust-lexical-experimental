//! Fast lexical string-to-integer conversion routines.
//!
//! The default implementations are highly optimized both for simple
//! strings, as well as input with large numbers of digits. In order to
//! keep performance optimal for simple strings, we avoid overly branching
//! to minimize the number of branches (and therefore optimization checks).
//! Most of the branches in the code are resolved at compile-time, and
//! the resulting ASM is monitored to ensure there are no regressions. For
//! larger strings, a limited number of optimization checks are included
//! to try faster, multi-digit parsing algorithms. For 32-bit integers,
//! we try to parse 4 digits at a time, and for 64-bit and larger integers,
//! we try to parse 8 digits at a time. Attempting both checks leads to
//! significant performance penalties for simple strings, so only 1
//! optimization is used at at a time.
//!
//! In addition, a compact, fallback algorithm uses a naive, simple
//! algorithm, parsing only a single digit at a time. This avoid any
//! unnecessary branching and produces smaller binaries, but comes
//! at a significant performance penalty for integers with more digits.
//!
//! # Features
//!
//! * `std` - Use the standard library.
//! * `power-of-two` - Add support for parsing power-of-two integer strings.
//! * `radix` - Add support for strings of any radix.
//! * `format` - Add support for parsing custom integer formats.
//! * `compact` - Reduce code size at the cost of performance.
//! * `safe` - Ensure only memory-safe indexing is used.
//!
//! `safe` is a no-op, since all parsers are memory-safe by default.
//!
//! # Note
//!
//! Only documented functionality is considered part of the public API:
//! any of the modules, internal functions, or structs may change
//! release-to-release without major or minor version changes. Use
//! internal implementation details at your own risk.
//!
//! lexical-parse-integer mainly exists as an implementation detail for
//! lexical-core, although its API is stable. If you would like to use
//! a high-level API that writes to and parses from `String` and `&str`,
//! respectively, please look at [lexical](https://crates.io/crates/lexical)
//! instead. If you would like an API that supports multiple numeric
//! conversions, please look at [lexical-core](https://crates.io/crates/lexical-core)
//! instead.
//!
//! # Version Support
//!
//! The minimum, standard, required version is 1.51.0, for const generic
//! support. Older versions of lexical support older Rust versions.
//!
#![doc = include_str!("../docs/Algorithm.md")]
//!
#![doc = include_str!("../docs/Benchmarks.md")]
// We want to have the same safety guarantees as Rust core,
// so we allow unused unsafe to clearly document safety guarantees.
#![allow(unused_unsafe)]
#![cfg_attr(feature = "lint", warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod shared;

pub mod algorithm;
pub mod compact;
pub mod options;
pub mod parse;

mod api;

// Re-exports
pub use self::api::{FromLexical, FromLexicalWithOptions};
#[doc(inline)]
pub use self::options::{Options, OptionsBuilder};
pub use lexical_util::error::Error;
pub use lexical_util::format::{self, NumberFormatBuilder};
pub use lexical_util::options::ParseOptions;
pub use lexical_util::result::Result;
