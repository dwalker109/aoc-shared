use std::fmt::Display;

use logging_timer::timer;

/// Runs and emits both parts of an AOC day via provided closures, with timings (if RUST_LOG=debug).
pub fn solve<T: Display, F1: FnOnce() -> T, F2: FnOnce() -> T>(f1: F1, f2: F2) {
    env_logger::init();

    let r1 = {
        let _tmr = timer!("Part 1");
        f1()
    };
    println!("Part 1: {r1}");
    
    let r2 = {
        let _tmr = timer!("Part 2");
        f2()
    };
    println!("Part 2: {r2}");
}