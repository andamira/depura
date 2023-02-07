//

use depura::*;

fn main() {
    let log_print = Logger::new("print 0")
        .level(LevelFilter::Error)
        .ignore(">target2");

    let log_write_1 = Logger::new("write 1")
        .file("log-multi-logger-0.log")
        .append(false)
        .level_debug()
        .allow(">target1");

    let log_write_2 = Logger::new("write 2")
        .file("log-multi-logger-1.log")
        .append(false)
        .target_level_debug()
        .allow(">target2")
        .allow(">target3");

    MultiLogger::new()
        .include(log_print)
        .include(log_write_1)
        .include(log_write_2)
        .init()
        .expect("multi_logger");

    // Check the filters

    error![target: ">target1", "for 1"];
    warn![target: ">target1", "for 1"];
    info![target: ">target1", "for 1"];
    debug![target: ">target1", "for 1"];
    trace![target: ">target1", "for 1"];

    error![target: ">target2", "for 2"];
    warn![target: ">target2", "for 2"];
    info![target: ">target2", "for 2"];
    debug![target: ">target2", "for 2"];
    trace![target: ">target2", "for 2"];

    error![target: ">target3", "for 3"];
    warn![target: ">target3", "for 3"];
    info![target: ">target3", "for 3"];
    debug![target: ">target3", "for 3"];
    trace![target: ">target3", "for 3"];
}
