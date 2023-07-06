// depura::time::scope
//
//! scope timer
//
// First version based on `measure_time` v0.8.2, Unlicense
// https://github.com/PSeitz/rust_measure_time
//

use human_repr::HumanDuration;

/// A structure used to log its lifetime the moment it drops.
#[derive(Debug)]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub struct ScopeTime {
    name: String,
    target: &'static str,
    module_path: &'static str,
    file: &'static str,
    line: u32,
    start: instant::Instant,
    level: log::LevelFilter,
}
impl ScopeTime {
    pub fn new<S: Into<String>>(
        target: &'static str,
        module_path: &'static str,
        file: &'static str,
        line: u32,
        name: S,
        level: log::LevelFilter,
    ) -> Self {
        Self {
            target,
            module_path,
            file,
            line,
            name: name.into(),
            start: instant::Instant::now(),
            level,
        }
    }
}

impl Drop for ScopeTime {
    fn drop(&mut self) {
        let time = self.start.elapsed().human_duration();

        if let Some(level) = self.level.to_level() {
            log::logger().log(
                &log::Record::builder()
                    .args(format_args!("{} {}", self.name, time))
                    .level(level)
                    .target(self.target)
                    .module_path(Some(self.module_path))
                    .file(Some(self.file))
                    .line(Some(self.line))
                    .build(),
            );
        } else {
            println!("{} took {}", self.name, time);
        }
    }
}

/// Logs the time with the [`print!`] macro.
#[macro_export]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
macro_rules! print_time {
    ($($arg:tt)+) => {
        #[allow(unused_variables)]
        let time = $crate::all::ScopeTime::new(
            module_path!(),
            module_path!(),
            file!(),
            line!(),
            format!($($arg)+),
            log::LevelFilter::Off
        );
    } ;
}
pub use print_time;

/// Logs the time with the [`log!`][log::log] macro.
#[macro_export]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
macro_rules! log_time {
    (target: $target:expr, $lvl:expr, $lvl2:expr, $($arg:tt)+) => (
        #[allow(unused_variables)]
        let time = if log::log_enabled!($lvl) {
            Some($crate::all::ScopeTime::new($target, module_path!(), file!(), line!(), format!($($arg)+), $lvl2) )
        } else{
            None
        };
    );
}
pub use log_time;

/// Logs the time with the [`error!`][log::error] macro.
#[macro_export]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
macro_rules! error_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, log::Level::Error, log::LevelFilter::Error, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), log::Level::Error, log::LevelFilter::Error, $($arg)+) )
}
pub use error_time;

/// Logs the time with the [`warn!`][log::warn] macro.
#[macro_export]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
macro_rules! warn_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, log::Level::Warn, log::LevelFilter::Warn, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), log::Level::Warn, log::LevelFilter::Warn, $($arg)+) )
}
pub use warn_time;

/// Logs the time with the [`info!`][log::info] macro.
#[macro_export]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
macro_rules! info_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, log::Level::Info, log::LevelFilter::Info, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), log::Level::Info, log::LevelFilter::Info, $($arg)+) )
}
pub use info_time;

/// Logs the time with the [`debug!`][log::debug] macro.
#[macro_export]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
macro_rules! debug_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, log::Level::Debug, log::LevelFilter::Debug, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), log::Level::Debug, log::LevelFilter::Debug, $($arg)+) )
}
pub use debug_time;

/// Logs the time with the [`trace!`][log::trace] macro.
#[macro_export]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
macro_rules! trace_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::all::Level::Trace, log::LevelFilter::Trace, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), log::Level::Trace, log::LevelFilter::Trace, $($arg)+) )
}
pub use trace_time;
