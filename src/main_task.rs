use crate::audio_reader::AudioReader;
use crate::audio_result::AudioResult;
use crate::{CHUNK_SIZE, POLL_EVERY};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Mutex, RwLock};

type ReadersLock<'a> = RwLock<Vec<ReaderLock<'a>>>;
type ReaderLock<'a> = Mutex<&'a mut AudioReader>;

pub fn main_task(result: &mut AudioResult, readers: &mut [AudioReader]) {
    let info = PollInfo {
        reader_index: AtomicUsize::new(0),
        sample_index: AtomicUsize::new(0),
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
    reader_index: AtomicUsize,
    sample_index: AtomicUsize,
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
            info.reader_index.store(0, Relaxed);
            for reader in readers.iter() {
                let mut reader = reader.lock().unwrap();
                for (chunk_index, sample) in reader.take(CHUNK_SIZE).enumerate() {
                    chunk[chunk_index] += sample;
                }
                info.reader_index.fetch_add(1, Relaxed);
            }
            info.sample_index.fetch_add(CHUNK_SIZE, Relaxed);
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
