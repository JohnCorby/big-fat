use crate::audio_reader::AudioReader;
use rayon::prelude::*;
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

pub fn par_fold<NewVec, Fold, Reduce>(
    readers: &mut [AudioReader],
    new_vec: NewVec,
    fold: Fold,
    reduce: Reduce,
) -> Vec<f32>
where
    NewVec: Fn() -> Vec<f32> + Send + Sync,
    Fold: Fn(Vec<f32>, &mut AudioReader) -> Vec<f32> + Send + Sync,
    Reduce: Fn(Vec<f32>, Vec<f32>) -> Vec<f32> + Send + Sync,
{
    IntoParallelIterator::into_par_iter(readers)
        .fold(&new_vec, fold)
        .reduce(&new_vec, reduce)
}
