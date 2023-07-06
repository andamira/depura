// depura::timer
//
//! Time related logging.
//

#[cfg(feature = "std")]
mod timeit;
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub use timeit::*;

#[cfg(feature = "std")]
mod scope;
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub use scope::*;
