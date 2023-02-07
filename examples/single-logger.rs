//
//

use depura::*;

fn main() {
    Logger::new("target 0")
        .level_warn()
        .init()
        .expect("logger print init");

    // Logger::new("target 0")
    //     .file("log-builder-0")
    //     .file_append(true)
    //     .init()
    //     .expect("logger file init");

    error![target: ">target1", "for 1"];
    warn![target: ">target1", "for 1"];
    info![target: ">target1", "for 1"];
    debug![target: ">target1", "for 1"];
    trace![target: ">target1", "for 1"];
}
