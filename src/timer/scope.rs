// depura::time::scope
//
//! scope timer
//
// Current version is mostly based on
// https://github.com/PSeitz/rust_measure_time
//

use human_repr::HumanDuration;

/// An structure used to log its lifetime the moment it drops.
#[derive(Debug)]
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
macro_rules! print_time {
    ($($arg:tt)+) => {
        #[allow(unused_variables)]
        let time = $crate::ScopeTime::new(
            module_path!(),
            module_path!(),
            file!(),
            line!(),
            format!($($arg)+),
            log::LevelFilter::Off
        );
    } ;
}

/// Logs the time with the [`log!`][log::log] macro.
#[macro_export]
macro_rules! log_time {
    (target: $target:expr, $lvl:expr, $lvl2:expr, $($arg:tt)+) => (
        #[allow(unused_variables)]
        let time = if $crate::log_enabled!($lvl) {
            Some($crate::ScopeTime::new($target, module_path!(), file!(), line!(), format!($($arg)+), $lvl2) )
        } else{
            None
        };
    );
}

/// Logs the time with the [`error!`][log::error] macro.
#[macro_export]
macro_rules! error_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::Level::Error, $crate::LevelFilter::Error, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), $crate::Level::Error, $crate::LevelFilter::Error, $($arg)+) )
}
/// Logs the time with the [`warn!`][log::warn] macro.
#[macro_export]
macro_rules! warn_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::Level::Warn, $crate::LevelFilter::Warn, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), $crate::Level::Warn, $crate::LevelFilter::Warn, $($arg)+) )
}
/// Logs the time with the [`info!`][log::info] macro.
#[macro_export]
macro_rules! info_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::Level::Info, $crate::LevelFilter::Info, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), $crate::Level::Info, $crate::LevelFilter::Info, $($arg)+) )
}
/// Logs the time with the [`debug!`][log::debug] macro.
#[macro_export]
macro_rules! debug_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::Level::Debug, $crate::LevelFilter::Debug, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), $crate::Level::Debug, $crate::LevelFilter::Debug, $($arg)+) )
}
/// Logs the time with the [`trace!`][log::trace] macro.
#[macro_export]
macro_rules! trace_time {
    (target: $target:expr, $($arg:tt)+) => (
        $crate::log_time!(target: $target, $crate::Level::Trace, $crate::LevelFilter::Trace, $($arg)+)
    );
    ($($arg:tt)+) => ($crate::log_time!(target: module_path!(), $crate::Level::Trace, $crate::LevelFilter::Trace, $($arg)+) )
}

/// Determines if a message logged at the specified level in that module will
/// be logged.
///
/// This can be used to avoid expensive computation of log message arguments if
/// the message would be ignored anyway.
///
/// # Examples
///
/// ```edition2018
/// use log::Level::Debug;
/// use log::{debug, log_enabled};
///
/// # fn foo() {
/// if log_enabled!(Debug) {
///     let data = expensive_call();
///     debug!("expensive debug data: {} {}", data.x, data.y);
/// }
/// if log_enabled!(target: "Global", Debug) {
///    let data = expensive_call();
///    debug!(target: "Global", "expensive debug data: {} {}", data.x, data.y);
/// }
/// # }
/// # struct Data { x: u32, y: u32 }
/// # fn expensive_call() -> Data { Data { x: 0, y: 0 } }
/// # fn main() {}
/// ```
#[macro_export(local_inner_macros)]
macro_rules! log_enabled {
    (target: $target:expr, $lvl:expr) => {{
        let lvl = $lvl;
        lvl <= $crate::log_crate::STATIC_MAX_LEVEL
            && lvl <= $crate::log_crate::max_level()
            && $crate::log_crate::__private_api_enabled(lvl, $target)
    }};
    ($lvl:expr) => {
        log_enabled!(target: __log_module_path!(), $lvl)
    };
}

// The log macro above cannot invoke format_args directly because it uses
// local_inner_macros. A format_args invocation there would resolve to
// $crate::format_args which does not exist. Instead invoke format_args here
// outside of local_inner_macros so that it resolves (probably) to
// core::format_args or std::format_args. Same for the several macros that
// follow.
//
// This is a workaround until we drop support for pre-1.30 compilers. At that
// point we can remove use of local_inner_macros, use $crate:: when invoking
// local macros, and invoke format_args directly.
#[doc(hidden)]
#[macro_export]
macro_rules! __log_format_args {
    ($($args:tt)*) => {
        format_args!($($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_module_path {
    () => {
        module_path!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_file {
    () => {
        file!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_line {
    () => {
        line!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_key {
    // key1 = 42
    ($($args:ident)*) => {
        stringify!($($args)*)
    };
    // "key1" = 42
    ($($args:expr)*) => {
        $($args)*
    };
}

// TODO
// ///
// #[macro_export]
// macro_rules! scope_time {
//     () => {};
// }
