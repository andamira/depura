// libera::lib
//
//!
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch
)]
//
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

// features safeguarding
#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't enable the `std` and `no-std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe` features at the same time.");

/// The *re-exported* [`log`][::log] crate:
#[doc(inline)]
pub use ::log as _log;

pub mod error;
pub mod logger;
pub mod timer;

// Selected root re-exports
#[doc(inline)]
pub use logger::*;

/// Everything is directly available here.
///
/// More precisely everything defined in this crate.
/// Not everything from the re-exported crates.
pub mod all {
    #[doc(inline)]
    pub use super::{error::*, logger::*, timer::*};

    pub use super::_log::{debug, error, info, log, trace, warn, Level, LevelFilter};
}
