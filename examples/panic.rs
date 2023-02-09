// - https://doc.rust-lang.org/stable/std/panic/struct.PanicInfo.html

use std::panic;

fn main() {
    // A)
    // panic::set_hook(Box::new(|info| {
    //     if let Some(s) = info.payload().downcast_ref::<&str>() {
    //         println!("panic occurred: {s:?}");
    //     } else {
    //         println!("panic occurred");
    //     }
    // }));
    //
    // panic!("Normal panic");

    // B)

    // panic::set_hook(Box::new(|s| { println!("Custom panic hook {s:?}"); }));
    //
    // // assert
    //
    // // make a panic
    // let mut a = vec![1][1];

    // let result = panic::catch_unwind(|| {
    //     println!("hello!");
    // });
    // dbg![&result];
    // assert!(result.is_ok());
    //
    // let result = panic::catch_unwind(|| {
    //     panic!("oh no!");
    // });
    // dbg![&result];
    // assert!(result.is_err());

    assert![panic::catch_unwind(|| println!("hello!")).is_ok()];
    assert![panic::catch_unwind(|| panic!("oh no!")).is_err()];

    println!("hey");
}
