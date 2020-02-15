use std::{
    thread,
    time::{Duration, Instant},
};

pub struct Time {
    start: Instant,
    frame_count: u32,
    fps: u32,
}

impl Time {
    pub fn new(fps: u32) -> Self {
        Self {
            start: Instant::now(),
            frame_count: 0,
            fps,
        }
    }

    /// restarts this timer, useful after loading
    /// a new scene
    pub fn restart(&mut self) {
        self.frame_count = 0;
        self.start = Instant::now();
    }

    pub fn fixed_seconds(&self) -> f32 {
        1.0 / self.fps as f32
    }

    pub fn frame(&mut self) {
        self.frame_count += 1;
        let finish = Duration::from_micros(1_000_000 / u64::from(self.fps)) * self.frame_count;
        if self.start.elapsed() < finish {
            while self.start.elapsed() < finish {
                thread::yield_now();
            }
        } else {
            println!("LAG at frame {}!", self.frame_count)
        }
    }
}
