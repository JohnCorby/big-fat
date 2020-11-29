//! misc utils to monitor stuff i guess

use std::io::{stdin, Read};
use std::time::{Duration, Instant};

pub fn time(f: impl FnOnce()) -> Duration {
    let time = Instant::now();
    f();
    let elapsed = time.elapsed();
    println!("time elapsed: {:?}", elapsed);
    elapsed
}

pub fn pause() {
    println!("press enter to continue");
    stdin().read_line(&mut String::new()).unwrap();
}
