// depura::error
//
//! Error types.
//

use log::{ParseLevelError, SetLoggerError};
#[cfg(feature = "std")]
use time::error::InvalidFormatDescription;

/// The *depura* result type.
pub type DepuraResult<N> = core::result::Result<N, DepuraError>;

/// The *depura* error type.
#[non_exhaustive]
pub enum DepuraError {
    /* from the log crate*/
    SetLogger(SetLoggerError),
    ParseLevel(ParseLevelError),

    /* from the time crate */
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    InvalidFormatDescription(InvalidFormatDescription),

    /// There are no loggers configured.
    NoLoggers,
}

mod core_impls {
    use super::*;
    use core::fmt;

    impl fmt::Display for DepuraError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use DepuraError::*;
            match self {
                #[cfg(feature = "std")]
                InvalidFormatDescription(i) => {
                    write!(f, "DepuraError::InvalidFormatDescription({i})")
                }
                SetLogger(_) => write!(f, "DepuraError::SetLogger(SetLoggerError)"),
                ParseLevel(_) => write!(f, "DepuraError::ParseLevel(ParseLevelError)"),
                NoLoggers => write!(f, "DepuraError::NoLoggers"),
                // _ => write!(f, ""),
            }
        }
    }
    impl fmt::Debug for DepuraError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(self, f)
        }
    }

    impl From<SetLoggerError> for DepuraError {
        fn from(err: SetLoggerError) -> Self {
            DepuraError::SetLogger(err)
        }
    }
    impl From<ParseLevelError> for DepuraError {
        fn from(err: ParseLevelError) -> Self {
            DepuraError::ParseLevel(err)
        }
    }
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    impl From<InvalidFormatDescription> for DepuraError {
        fn from(err: InvalidFormatDescription) -> Self {
            DepuraError::InvalidFormatDescription(err)
        }
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use super::DepuraError;

    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    impl std::error::Error for DepuraError {}
}
