use std;

pub struct Timer {
    start: std::time::Instant
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: std::time::Instant::now()
        }
    }

    pub fn restart(&mut self) -> u64 {
        let time = self.start.elapsed();
        self.start = std::time::Instant::now();
        time.as_secs() * 1_000 + (time.subsec_nanos() / 1_000_000) as u64
    }
}