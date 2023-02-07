// depura::error
//
//! Error types.
//

use log::{ParseLevelError, SetLoggerError};
use time::error::InvalidFormatDescription;

/// The *uiuiui* result type.
pub type Result<N> = core::result::Result<N, Error>;

/// The *depura* error type.
#[non_exhaustive]
pub enum Error {
    /* from the log crate*/
    SetLogger(SetLoggerError),
    ParseLevel(ParseLevelError),

    /* from the time crate */
    InvalidFormatDescription(InvalidFormatDescription),

    /// There are no loggers configured.
    NoLoggers,
}

mod core_impls {
    use super::*;
    use core::fmt;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::InvalidFormatDescription(i) => {
                    write!(f, "Error::InvalidFormatDescription({i})")
                }
                Error::SetLogger(_) => write!(f, "Error::SetLogger(SetLoggerError)"),
                Error::ParseLevel(_) => write!(f, "Error::ParseLevel(ParseLevelError)"),
                Error::NoLoggers => write!(f, "Error::NoLoggers"),
                // _ => write!(f, ""),
            }
        }
    }
    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(self, f)
        }
    }

    impl From<SetLoggerError> for Error {
        fn from(err: SetLoggerError) -> Self {
            Error::SetLogger(err)
        }
    }
    impl From<ParseLevelError> for Error {
        fn from(err: ParseLevelError) -> Self {
            Error::ParseLevel(err)
        }
    }
    impl From<InvalidFormatDescription> for Error {
        fn from(err: InvalidFormatDescription) -> Self {
            Error::InvalidFormatDescription(err)
        }
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use super::Error;

    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    impl std::error::Error for Error {}
}
