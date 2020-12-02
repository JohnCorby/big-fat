use std::io::stdin;
use std::path::Path;

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

pub fn file_name(path: &Path) -> &str {
    path.file_name().unwrap().to_str().unwrap()
}
