// depura::logger::single
//
//!
//

#![allow(dead_code)]

use std::path::{Path, PathBuf};

use log::*;
use simplelog::*;
use time::format_description;

use crate::{error::Result, logger::MultiLogger};

/// An individual logger builder.
#[derive(Clone, Debug)]
pub struct Logger {
    /// the name of the log
    pub(crate) name: String,

    /// The path for the log file to use.
    ///
    /// If `None`, the log will be printed to *stdout*.
    pub(crate) file_path: Option<PathBuf>,
    /// Whether to append to the file instead of overwriting it.
    pub(crate) file_append: bool,
    /// Truncates the file to 0 length.
    pub(crate) file_truncate: bool,
    /// Whether to create the file if it doesn't already exist.
    pub(crate) file_create: bool,

    /// The chosen main level to filter.
    pub(crate) level: LevelFilter,
    ///
    pub(crate) target_level: LevelFilter,
    // more...
    pub(crate) allowed: Vec<&'static str>,
    pub(crate) ignored: Vec<&'static str>,

    ///
    // https://docs.rs/simplelog/latest/simplelog/struct.ConfigBuilder.html
    pub(crate) config: ConfigBuilder,

    /// The datetime format to use in headers.
    pub(crate) datetime_fmt: Vec<FormatItem<'static>>,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            name: String::new(),
            file_path: None,
            file_append: true,
            file_truncate: true,
            file_create: true,

            level: LevelFilter::max(),      // the most verbose logging
            target_level: LevelFilter::Off, // CHECK

            allowed: vec![],
            ignored: vec![],

            config: default_config(),

            datetime_fmt: format_description::parse(Logger::DATETIME_FMT).unwrap(),
        }
    }
}
// Creates a new default_config, which can be further customized
fn default_config() -> ConfigBuilder {
    ConfigBuilder::new()
        .set_time_level(LevelFilter::Error) // Error*
        .set_max_level(LevelFilter::Error) // Error*
        .set_thread_level(LevelFilter::Off) // Debug
        .set_target_level(LevelFilter::Off) // Debug* // set with print_target
        .set_location_level(LevelFilter::Off) // Trace*
        // https://time-rs.github.io/book/api/format-description.html
        .set_time_format_custom(format_description!(
            "[hour]:[minute]:[second].[subsecond digits:5]" // *4
        ))
        .set_time_offset_to_local()
        .expect("couldn't set local offset")
        //
        .set_thread_mode(ThreadLogMode::Both)
        .set_level_padding(LevelPadding::Right)
        .clone()
}

impl Logger {
    /// The default date formatting for headers.
    pub const DATETIME_FMT: &'static str = "[year]-[month]-[day]_[hour]:[minute]:[second].[subsecond digits:4]_[offset_hour sign:mandatory]:[offset_minute]";
}

impl Logger {
    /// Returns a new logger builder.
    pub fn new(name: &str) -> Self {
        Self::default().name(name)
    }

    /// Configures the name.
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    //

    /// Configures the main level filter.
    pub fn level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }
    /// No log messages will pass.
    ///
    /// Equivalent to `LevelFilter::Off.
    pub fn level_off(mut self) -> Self {
        self.level = LevelFilter::Off;
        self
    }
    /// Only `Error` level log messages will pass.
    ///
    /// Equivalent to `LevelFilter::Error`.
    pub fn level_error(mut self) -> Self {
        self.level = LevelFilter::Error;
        self
    }
    /// Only `Error` and `Warn` level log messages will pass.
    ///
    /// Equivalent to `LevelFilter::Warn`.
    pub fn level_warn(mut self) -> Self {
        self.level = LevelFilter::Warn;
        self
    }
    /// Only `Error`, `Warn` and `Info` level log messages will pass.
    ///
    /// Equivalent to `LevelFilter::Info`.
    pub fn level_info(mut self) -> Self {
        self.level = LevelFilter::Info;
        self
    }
    /// Only `Error`, `Warn`, `Info` and `Debug` level log messages will pass.
    ///
    /// Equivalent to `LevelFilter::Debug`.
    pub fn level_debug(mut self) -> Self {
        self.level = LevelFilter::Debug;
        self
    }
    /// `Error`, `Warn`, `Info`, `Debug` and `Trace` level log messages will pass.
    ///
    /// Equivalent to `LevelFilter::Trace`.
    pub fn level_trace(mut self) -> Self {
        self.level = LevelFilter::Trace;
        self
    }

    //

    /// Set at which level and above (more verbose) the target shall be logged.
    ///
    /// Default: Off
    pub fn target_level(mut self, target_level: LevelFilter) -> Self {
        self.target_level = target_level;
        self
    }
    /// No *target* strings will be logged.
    ///
    /// Equivalent to `LevelFilter::Off.
    pub fn target_level_off(mut self) -> Self {
        self.target_level = LevelFilter::Off;
        self
    }
    /// All *target* strings will be logged.
    ///
    /// Equivalent to `LevelFilter::Error.
    pub fn target_level_all(mut self) -> Self {
        self.target_level = LevelFilter::Error;
        self
    }
    /// All *target* strings will be logged.
    ///
    /// Equivalent to `LevelFilter::Error.
    pub fn target_level_error(mut self) -> Self {
        self.target_level = LevelFilter::Error;
        self
    }
    /// Only `Warn`, `Info`, `Debug` and `Trace` *target* strings will be logged.
    /// All *target* strings except `Error`, will be logged.
    ///
    /// Equivalent to `LevelFilter::Warn.
    pub fn target_level_warn(mut self) -> Self {
        self.target_level = LevelFilter::Warn;
        self
    }
    /// Only `Info`, `Debug` and `Trace` *target* strings will be logged.
    ///
    /// Equivalent to `LevelFilter::Info.
    pub fn target_level_info(mut self) -> Self {
        self.target_level = LevelFilter::Info;
        self
    }
    /// Only `Debug` and `Trace` *target* strings will be logged.
    ///
    /// Equivalent to `LevelFilter::Debug.
    pub fn target_level_debug(mut self) -> Self {
        self.target_level = LevelFilter::Debug;
        self
    }
    /// Only `Trace` *target* strings will be logged.
    ///
    /// Equivalent to `LevelFilter::Trace.
    pub fn target_level_trace(mut self) -> Self {
        self.target_level = LevelFilter::Trace;
        self
    }

    /* file */

    /// Configures an output file.
    ///
    /// If no output file is configured, log will be printed to the terminal.
    pub fn file(mut self, path: impl AsRef<Path>) -> Self {
        self.file_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Unsets the output file.
    pub fn unset_file(mut self) -> Self {
        self.file_path = None;
        self
    }

    /// Determines wether the log will be appended to the file.
    pub fn append(mut self, append: bool) -> Self {
        self.file_append = append;
        self
    }

    /// Determines wether the log file will be truncated to 0 length.
    pub fn truncate(mut self, truncate: bool) -> Self {
        self.file_truncate = truncate;
        self
    }

    /* filter */

    /// Pushes a filter string to the *only allowed if matches* list.
    pub fn allow(mut self, allow: &'static str) -> Self {
        self.allowed.push(allow);
        self
    }
    /// Pushes a filter string to the list of *ignored if matches* list.
    pub fn ignore(mut self, ignore: &'static str) -> Self {
        self.ignored.push(ignore);
        self
    }

    /// Initializes a single logger for its immediate use.
    ///
    /// Errors if there's another logger initialized.
    pub fn init(self) -> Result<()> {
        MultiLogger::new().include(self).init()
    }
}
