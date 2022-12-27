use std::{fmt::Display, time::Instant};

/// Runs and emits both parts of an AOC day via provided closures, with timings.
pub fn solve<T: Display, F1: FnOnce() -> T, F2: FnOnce() -> T>(f1: F1, f2: F2) {
    let (r, d) = {
        let _tmr = Instant::now();
        (f1(), Instant::now().duration_since(_tmr))
    };
    println!("Part 1: {r} ({:?})", d);

    let (r, d) = {
        let _tmr = Instant::now();
        (f2(), Instant::now().duration_since(_tmr))
    };
    println!("Part 2: {r} ({:?})", d);
}
