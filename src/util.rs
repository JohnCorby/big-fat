use rayon::prelude::*;
use std::io::stdin;
use std::path::Path;

#[macro_export]
macro_rules! time {
    ($body:block) => {{
        let time = std::time::Instant::now();
        $body;
        time.elapsed()
    }};
}

#[macro_export]
macro_rules! try_assert {
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            return std::result::Result::Err(format!($($arg)*));
        }
    };
}

pub fn pause() {
    println!("press enter to continue");
    stdin().read_line(&mut String::new()).unwrap();
}

pub fn file_name(path: &Path) -> &str {
    path.file_name().unwrap().to_str().unwrap()
}

/// for some reason, clion doesnt get the return type of into_par_iter. this is a workaround
pub fn par_iter<Items: IntoParallelIterator>(items: Items) -> Items::Iter {
    items.into_par_iter()
}

pub fn par_fold<Items: IntoParallelIterator, Result: Send>(
    items: Items,
    init: impl Fn() -> Result + Send + Sync + Copy,
    fold: impl Fn(Result, Items::Item) -> Result + Send + Sync,
    reduce: impl Fn(Result, Result) -> Result + Send + Sync,
) -> Result {
    par_iter(items).fold(init, fold).reduce(init, reduce)
}
