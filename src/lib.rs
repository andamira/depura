// depura::lib
//
//!
//

#![warn(clippy::all)]
//
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

// features safeguarding
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe` features at the same time.");

devela::deprecate_feature![old: "no-std", new: "no_std", since: "0.3.0"];

/// The *re-exported* [`log`][log_crate] crate:
pub use ::log as log_crate;

pub mod error;
pub mod logger;
pub mod timer;

// Selected root re-exports
#[doc(inline)]
pub use logger::*;

/// All the items are reexported here.
///
/// More precisely everything defined in this crate.
/// Not everything from the re-exported crates.
pub mod all {
    #[doc(inline)]
    pub use super::{error::*, logger::*, timer::*};

    pub use super::log_crate::{debug, error, info, log, trace, warn, Level, LevelFilter};
}
