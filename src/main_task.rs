use crate::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Mutex, RwLock};

pub fn main_task(result: &mut AudioResult, readers: &mut [AudioReader]) {
    let info = PollInfo {
        samples_done: AtomicUsize::new(0),
        readers_left: AtomicUsize::new(readers.len()),
    };
    let readers: ReadersLock = ReadersLock::new(readers.iter_mut().map(ReaderLock::new).collect());
    rayon::scope(|s| {
        s.spawn(|_| sum_job(result, &readers, &info));
        s.spawn(|_| poll_job(&info));
    });
}

#[derive(Debug)]
struct PollInfo {
    samples_done: AtomicUsize,
    readers_left: AtomicUsize,
}

fn poll_job(info: &PollInfo) {
    loop {
        // break if done
        if info.readers_left.load(Relaxed) == 0 {
            break;
        }

        println!("{:?}", info);
        std::thread::sleep(POLL_EVERY);
    }
}

type ReadersLock<'a> = RwLock<Vec<ReaderLock<'a>>>;
type ReaderLock<'a> = Mutex<&'a mut AudioReader>;

fn sum_job(result: &mut AudioResult, readers: &ReadersLock, info: &PollInfo) {
    let mut chunk = vec![0.0; CHUNK_SIZE];
    loop {
        // read, sum, and push
        {
            let readers = readers.read().unwrap();
            if readers.is_empty() {
                break;
            }

            chunk.fill(0.0);
            for reader in readers.iter() {
                let mut reader = reader.lock().unwrap();
                for (chunk_index, sample) in reader.take(CHUNK_SIZE).enumerate() {
                    chunk[chunk_index] += sample;
                }
            }
            info.samples_done.fetch_add(CHUNK_SIZE, Relaxed);
        }

        // write to result
        result.push(&chunk).unwrap();

        // remove empty
        {
            let mut readers = readers.write().unwrap();
            readers.drain_filter(|reader| {
                let reader = reader.get_mut().unwrap();
                reader.reached_none()
            });
            info.readers_left.store(readers.len(), Relaxed);
        }
    }
}
