// depura::time::timeit
//
//! function timer
//

use human_repr::HumanDuration;
use log::{log, Level};

use std::time::{Duration, SystemTime};

//
fn timeit_inner<F: Fn() -> T, T>(f: F, text: &str) -> (T, Duration, String) {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();

    let text = if text.is_empty() {
        format!["closure at `{}:{}`", file![], line![]]
    } else {
        text.to_string()
    };

    (result, duration, text)
}

/// Prints the duration of a closure
#[doc(hidden)]
pub fn timeit_println<F: Fn() -> T, T>(f: F, text: &str) -> T {
    let (result, duration, text) = timeit_inner(f, text);
    println!("{text} took {}", duration.human_duration());
    result
}

/// Logs the duration of a closure.
#[doc(hidden)]
pub fn timeit_log<F: Fn() -> T, T>(
    target: Option<&'static str>,
    level: Level,
    f: F,
    text: &str,
) -> T {
    let (result, duration, text) = timeit_inner(f, text);

    if let Some(target) = target {
        log::log!(
            target: target,
            level,
            "{text} took {}",
            duration.human_duration()
        );
    } else {
        log!(level, "{text} took {}", duration.human_duration());
    }
    result
}

/// Times the duration of a closure, logging or printing it out.
#[macro_export]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
macro_rules! timeit {
    // print
    (print, || $closure:expr, $text:literal) => {
        $crate::timeit_println(|| $closure, $text)
    };
    (print, || $closure:expr, $($arg:tt)+) => {
        $crate::timeit_println(|| $closure, &format![$($arg)+])
    };

    // info
    (info, target: $target:expr, || $closure:expr) => {
        timeit![log::Level::Info, target: $target, || $closure ]
    };
    (info, target: $target:expr, || $closure:expr, $text:literal) => {
        timeit![log::Level::Info, target: $target, || $closure, $text]
    };
    (info, target: $target:expr, || $closure:expr, $($arg:tt)+) => {
        timeit![log::Level::Info, target: $target, || $closure, $($arg)+]
    };
    (info, || $closure:expr) => { timeit![log::Level::Info, || $closure ] };
    (info, || $closure:expr, $text:literal) => { timeit![log::Level::Info, || $closure, $text] };
    (info, || $closure:expr, $($arg:tt)+) => { timeit![log::Level::Info, || $closure, $($arg)+] };

    // error
    (error, target: $target:expr, || $closure:expr) => {
        timeit![log::Level::Error, target: $target, || $closure ]
    };
    (error, target: $target:expr, || $closure:expr, $text:literal) => {
        timeit![log::Level::Error, target: $target, || $closure, $text]
    };
    (error, target: $target:expr, || $closure:expr, $($arg:tt)+) => {
        timeit![log::Level::Error, target: $target, || $closure, $($arg)+]
    };
    (error, || $closure:expr) => { timeit![log::Level::Error, || $closure ] };
    (error, || $closure:expr, $text:literal) => { timeit![log::Level::Error, || $closure, $text] };
    (error, || $closure:expr, $($arg:tt)+) => { timeit![log::Level::Error, || $closure, $($arg)+] };

    // warn
    (warn, target: $target:expr, || $closure:expr) => {
        timeit![log::Level::Warn, target: $target, || $closure ]
    };
    (warn, target: $target:expr, || $closure:expr, $text:literal) => {
        timeit![log::Level::Warn, target: $target, || $closure, $text]
    };
    (warn, target: $target:expr, || $closure:expr, $($arg:tt)+) => {
        timeit![log::Level::Warn, target: $target, || $closure, $($arg)+]
    };
    (warn, || $closure:expr) => { timeit![log::Level::Warn, || $closure ] };
    (warn, || $closure:expr, $text:literal) => { timeit![log::Level::Warn, || $closure, $text] };
    (warn, || $closure:expr, $($arg:tt)+) => { timeit![log::Level::Warn, || $closure, $($arg)+] };

    // debug
    (debug, target: $target:expr, || $closure:expr) => {
        timeit![log::Level::Debug, target: $target, || $closure ]
    };
    (debug, target: $target:expr, || $closure:expr, $text:literal) => {
        timeit![log::Level::Debug, target: $target, || $closure, $text]
    };
    (debug, target: $target:expr, || $closure:expr, $($arg:tt)+) => {
        timeit![log::Level::Debug, target: $target, || $closure, $($arg)+]
    };
    (debug, || $closure:expr) => { timeit![log::Level::Debug, || $closure ] };
    (debug, || $closure:expr, $text:literal) => { timeit![log::Level::Debug, || $closure, $text] };
    (debug, || $closure:expr, $($arg:tt)+) => { timeit![log::Level::Debug, || $closure, $($arg)+] };

    // trace
    (trace, target: $target:expr, || $closure:expr) => {
        timeit![log::Level::Trace, target: $target, || $closure ]
    };
    (trace, target: $target:expr, || $closure:expr, $text:literal) => {
        timeit![log::Level::Trace, target: $target, || $closure, $text]
    };
    (trace, target: $target:expr, || $closure:expr, $($arg:tt)+) => {
        timeit![log::Level::Trace, target: $target, || $closure, $($arg)+]
    };
    (trace, || $closure:expr) => { timeit![log::Level::Trace, || $closure ] };
    (trace, || $closure:expr, $text:literal) => { timeit![log::Level::Trace, || $closure, $text] };
    (trace, || $closure:expr, $($arg:tt)+) => { timeit![log::Level::Trace, || $closure, $($arg)+] };

    // log
    ($level:expr, target: $target:expr, || $closure:expr) => {
        $crate::all::timeit_log(Some($target), $level, || $closure, "")
    };
    ($level:expr, target: $target:expr, || $closure:expr, $text:literal) => {
        $crate::all::timeit_log(Some($target), $level, || $closure, $text)
    };
    ($level:expr, target: $target:expr, || $closure:expr, $($arg:tt)+) => {
        $crate::all::timeit_log(Some($target), $level, || $closure, &format![$($arg)+])
    };
    //
    ($level:expr, || $closure:expr) => {
        $crate::all::timeit_log(None, $level, || $closure, "")
    };
    ($level:expr, || $closure:expr, $text:literal) => {
        $crate::all::timeit_log(None, $level, || $closure, $text)
    };
    ($level:expr, || $closure:expr, $($arg:tt)+) => {
        $crate::all::timeit_log(None, $level, || $closure, &format![$($arg)+])
    };
}
pub use timeit;
