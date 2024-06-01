/*! The monitor module is for recording the runtime and momery. */
use std::time::{Duration, Instant};
use memory_stats::memory_stats;
use psutil;

pub struct Monitor {
    clock: Instant,
}

impl Monitor {
    pub fn new()->Self {
        Monitor {clock: Instant::now()}
    }

    pub fn get_runtime_memory_byte(&mut self) -> (Duration, psutil::Bytes) {

        (self.clock.elapsed(), (memory_stats().unwrap().physical_mem).try_into().unwrap())
    }
}


pub mod MemoryTransform {
    pub fn from_Bytes(x: i64) -> psutil::Bytes {
        x as psutil::Bytes
    }
    pub fn from_KB(x: i64) -> psutil::Bytes {
        from_Bytes(x * 1024)
    }
    pub fn from_MB(x: i64) -> psutil::Bytes {
        from_KB(x * 1024)
    }
    pub fn from_GB(x: i64) -> psutil::Bytes {
        from_MB(x * 1024)
    }
    pub fn to_Bytes(x: psutil::Bytes) -> i64 {
        x as i64
    }
    pub fn to_KB(x: psutil::Bytes) -> i64 {
        to_Bytes(x / 1024)
    }
    pub fn to_MB(x: psutil::Bytes) -> i64 {
        to_KB(x / 1024)
    }
    pub fn to_GB(x: psutil::Bytes) -> i64 {
        to_MB(x / 1024)
    }
}