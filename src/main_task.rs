use crate::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Mutex;

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
    current_chunk: AtomicUsize,
    current_reader: Mutex<String>,
    current_chunk_index: AtomicUsize,
}

fn poll_job(info: &PollInfo) {
    while info.readers_left.load(Relaxed) > 0 {
        println!("{:?}", info);
        std::thread::sleep(POLL_DELAY);
    }
}

fn sum_job(result: &mut AudioResult, mut readers: Vec<AudioReader>, info: &PollInfo) {
    let mut chunk = vec![0.0; CHUNK_SIZE];
    while !readers.is_empty() {
        // read and sum
        chunk = readers.iter_mut().fold(chunk, |mut chunk, reader| {
            for (chunk_index, reader_sample) in reader.take(CHUNK_SIZE).enumerate() {
                chunk[chunk_index] += reader_sample;
                info.current_chunk_index.store(chunk_index, Relaxed);
            }
            *info.current_reader.lock().unwrap() = format!("{}", reader);
            if reader.at_eof() {
                info.readers_left.fetch_sub(1, Relaxed);
            }
            chunk
        });
        info.current_chunk.fetch_add(1, Relaxed);

        // remove done
        readers.drain_filter(|reader| reader.at_eof());

        // write to result, resetting the chunk
        result.flush(&mut chunk).unwrap();
    }
}
