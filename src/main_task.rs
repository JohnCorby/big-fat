use crate::*;
use rayon::prelude::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

pub fn make_result(result: &mut AudioResult, readers: Vec<AudioReader>) {
    let info = PollInfo {
        readers_left: AtomicUsize::new(readers.len()),
        ..Default::default()
    };
    rayon::join(|| sum_job(result, readers, &info), || poll_job(&info));
}

#[derive(Debug, Default)]
struct PollInfo {
    readers_left: AtomicUsize,
    chunks_done: AtomicUsize,
}

fn poll_job(info: &PollInfo) {
    while info.readers_left.load(Relaxed) > 0 {
        println!("{:?}", info);
        std::thread::sleep(POLL_DELAY);
    }
}

fn sum_job(result: &mut AudioResult, mut readers: Vec<AudioReader>, info: &PollInfo) {
    // read from 1 reader at a time and add to chunk
    let fold_op = |mut chunk: Vec<f32>, reader: &mut AudioReader| {
        for (index, sample) in reader.take(CHUNK_SIZE).enumerate() {
            chunk[index] += sample;
        }
        if reader.at_eof() {
            info.readers_left.fetch_sub(1, Relaxed);
        }
        chunk
    };
    // create a chunk vec
    let make_chunk = || vec![0.0; CHUNK_SIZE];
    // fold all the readers
    let fold = |readers: &mut [AudioReader]| {
        readers
            .into_par_iter()
            .fold(make_chunk, fold_op)
            .reduce(make_chunk, |l, r| {
                l.iter().zip(r).map(|(ls, rs)| ls + rs).collect()
            })
    };

    while !readers.is_empty() {
        // read and sum
        let chunk = fold(&mut readers);
        info.chunks_done.fetch_add(1, Relaxed);

        // remove done
        readers.drain_filter(|reader| reader.at_eof());

        // write to result
        result.flush(chunk).unwrap();
    }
}
