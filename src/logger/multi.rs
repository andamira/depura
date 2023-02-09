// depura::logger::multi
//
//!
//

use std::{fs::OpenOptions, io::Write};

use log::*;
use simplelog::*;
use time::{format_description, OffsetDateTime};

use crate::{
    error::{DepuraError, DepuraResult as Result},
    logger::Logger,
};

/// A combination of multiple loggers.
#[derive(Clone, Debug)]
pub struct MultiLogger {
    /// The cloned logger builders. Useful for header construction on init.
    loggers: Vec<Logger>,

    /* settings for overriding the individual loggers */
    /// The chosen main level to filter.
    level: Option<LevelFilter>,
    target_level: Option<LevelFilter>,
    // ...
    allowed: Vec<&'static str>,
    ignored: Vec<&'static str>,

    /// The datetime format to use in headers.
    datetime_fmt: Vec<FormatItem<'static>>,
}

impl Default for MultiLogger {
    fn default() -> Self {
        Self {
            loggers: vec![],
            datetime_fmt: format_description::parse(Logger::DATETIME_FMT).unwrap(),

            level: None,
            target_level: None,
            // ...
            allowed: vec![],
            ignored: vec![],
        }
    }
}

impl MultiLogger {
    /// Returns a new multi-logger builder. Finish it with init() to use
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes the multi-logger for its immediate use.
    pub fn init(self) -> Result<()> {
        // let hpreline = "# "; // MAYBE
        let hsepline = format!["# {}\n", "-".repeat(80)];

        let hdatetime = format![
            "# {}\n",
            OffsetDateTime::now_local()
                .expect("couldn't fetch current local datetime")
                .format(&self.datetime_fmt)
                .expect("couldn't format current datetime")
        ];

        let mut inner_loggers: Vec<Box<dyn SharedLogger>> = vec![];

        /* prepare each logger */

        if self.loggers.is_empty() {
            return Err(DepuraError::NoLoggers);
        }

        for mut logger in self.loggers {
            /* build config */

            // override level
            let level = self.level.unwrap_or(logger.level);

            // override target_level
            let target_level = self.target_level.unwrap_or(logger.target_level);
            logger.config.set_target_level(target_level);

            // override allowed list
            let allowed = if self.allowed.is_empty() {
                logger.allowed
            } else {
                self.allowed.clone()
            };
            let ignored = if self.ignored.is_empty() {
                logger.ignored
            } else {
                self.ignored.clone()
            };

            for a in allowed.iter() {
                logger.config.add_filter_allow_str(a);
            }
            for i in ignored.iter() {
                logger.config.add_filter_ignore_str(i);
            }

            let config = logger.config.build();

            /* build header */

            let hallowed = if allowed.is_empty() {
                String::new()
            } else {
                format!["# allowed: \"{}\".\n", allowed.join("\", \"")]
            };
            let hignored = if ignored.is_empty() {
                String::new()
            } else {
                format!["# ignored: \"{}\".\n", ignored.join("\", \"")]
            };

            let hprefix = format![
                "{0}# Starting log: \"{1}\"\n{hdatetime}{2}{3}",
                hsepline, logger.name, hallowed, hignored,
            ];

            // A) write to a file
            if let Some(file_path) = logger.file_path {
                let mut file = OpenOptions::new();

                file.write(true)
                    .create(logger.file_create)
                    .append(logger.file_append);

                let mut file = file.open(file_path).expect("creating logfile");

                // to avoid the issue: https://github.com/rust-lang/rust/issues/34347
                if !logger.file_append && logger.file_truncate {
                    file.set_len(0).expect("set_len(0)")
                }

                file.write_fmt(format_args!["{hprefix}{hsepline}"])
                    .unwrap_or_else(|_| panic!["couldn't write header to «{file:?}»"]);

                inner_loggers.push(WriteLogger::new(level, config, file));

            // B) print to screen
            } else {
                print!("{hprefix}{hsepline}");

                inner_loggers.push(SimpleLogger::new(level, config));
            }
            // C) more?
        }
        CombinedLogger::init(inner_loggers).map_err(|e| e.into())
    }

    /// Includes a new logger to the multi-logger.
    pub fn include(mut self, logger: Logger) -> Self {
        self.loggers.push(logger);
        self
    }

    /// Changes the time format_description
    ///
    /// Follow the [format description][0] from the [`time`][1] crate.
    ///
    /// [0]: https://time-rs.github.io/book/api/format-description.html
    /// [1]: https://docs.rs/time/latest/time/format_description/index.html
    pub fn datetime_format(mut self, format: &'static str) -> Result<Self> {
        self.datetime_fmt = format_description::parse(format)?;
        Ok(self)
    }

    /// Overrides all the individual [`Logger`]'s level.
    pub fn level(mut self, level: impl Into<Option<LevelFilter>>) -> Self {
        self.level = level.into();
        self
    }

    /// Overrides all the individual [`Logger`]'s target level.
    pub fn target_level(mut self, target_level: impl Into<Option<LevelFilter>>) -> Self {
        self.target_level = target_level.into();
        self
    }
    /// All *target* strings will be logged.
    ///
    /// Equivalent to `LevelFilter::Error.
    pub fn target_level_all(mut self) -> Self {
        self.target_level = LevelFilter::Error.into();
        self
    }

    /// Overrides all the individual *only allowed if matches* list.
    pub fn allow(mut self, allow: &'static str) -> Self {
        self.allowed.push(allow);
        self
    }
    /// Overrides all the individual *ignored if matches* list.
    pub fn ignore(mut self, ignore: &'static str) -> Self {
        self.ignored.push(ignore);
        self
    }
}
