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

    pub fn fixed_seconds(&self) -> f32 {
        1.0 / self.fps as f32
    }

    pub fn frame(&mut self) {
        self.frame_count += 1;
        if let Some(dur) = (Duration::from_micros(1_000_000 / self.fps as u64) * self.frame_count)
            .checked_sub(self.start.elapsed())
        {
            thread::sleep(dur)
        } else {
            println!("LAG at frame {}!", self.frame_count)
        }
    }
}
