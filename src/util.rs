use std::io::{stdin, Read};

#[macro_export]
macro_rules! time {
    ($body:block) => {
        let time = std::time::Instant::now();
        $body;
        println!("time elapsed: {:?}", time.elapsed());
    };
}

pub fn pause() {
    println!("press enter to continue");
    stdin().read_line(&mut String::new()).unwrap();
}
