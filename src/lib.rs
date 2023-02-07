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

pub mod error;

#[cfg(feature = "std")]
mod logger;
mod timer;

pub use self::timer::*;
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub use logger::{Logger, MultiLogger};

/* re-exports */

/// *(from [`log`][::log]).*
pub use log::{debug, error, info, log, trace, warn, Level, LevelFilter};
