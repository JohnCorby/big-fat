use rayon::prelude::*;
use std::io::stdin;
use std::path::Path;

#[macro_export]
macro_rules! time {
    ($body:block) => {{
        let time = ::std::time::Instant::now();
        $body;
        time.elapsed()
    }};
}

#[macro_export]
macro_rules! try_assert {
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            return ::std::result::Result::Err(format!($($arg)*));
        }
    };
}

pub fn pause() {
    println!("press enter to continue");
    stdin().read_line(&mut String::new()).unwrap();
}

pub fn file_name(path: &Path) -> &str {
    path.file_name().unwrap_or_default().to_str().unwrap()
}
pub fn file_extension(path: &Path) -> &str {
    path.extension().unwrap_or_default().to_str().unwrap()
}

/// for some reason, clion doesnt get the return type of into_par_iter. this is a workaround
pub fn par_iter<I: IntoParallelIterator>(items: I) -> I::Iter {
    items.into_par_iter()
}
