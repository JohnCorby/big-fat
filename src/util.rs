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

pub fn par_fold<F, I>(mut iterators: Vec<I>, mut vec: Vec<f32>, f: F) -> Vec<f32>
where
    F: Fn(Vec<f32>, &mut I) -> Vec<f32>,
    I: Iterator,
{
    vec = iterators.iter_mut().fold(vec, f);
    vec
}
