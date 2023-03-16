// depura::logger
//
//!
//

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
mod single;
#[cfg(feature = "std")]
pub use single::Logger;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
mod multi;
#[cfg(feature = "std")]
pub use multi::MultiLogger;
