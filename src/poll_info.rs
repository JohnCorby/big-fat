use crate::cli::POLL_DELAY;
use std::fmt::{Display, Formatter, Result};
use std::sync::atomic::Ordering::*;
use std::sync::atomic::{AtomicBool, AtomicUsize};

pub struct PollInfo {
    readers_left: AtomicUsize,
    iterations_done: AtomicUsize,
    is_done: AtomicBool,
}

impl PollInfo {
    pub fn new(num_readers: usize) -> Self {
        Self {
            readers_left: AtomicUsize::new(num_readers),
            iterations_done: Default::default(),
            is_done: Default::default(),
        }
    }
    pub fn reader_done(&self) {
        self.readers_left.fetch_sub(1, Relaxed);
    }
    pub fn iteration_done(&self) {
        self.iterations_done.fetch_add(1, Relaxed);
    }
    pub fn done(&self) {
        self.is_done.store(true, Relaxed);
    }
}

impl Display for PollInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:?} readers left\t\t{:?} iterations done",
            self.readers_left, self.iterations_done
        )
    }
}

pub fn poll_job(info: &PollInfo) {
    while !info.is_done.load(Relaxed) {
        println!("{}", info);
        std::thread::sleep(*POLL_DELAY);
    }
}
