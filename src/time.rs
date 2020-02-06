pub struct Time {
    fps: u32,
}

impl Time {
    pub fn new(fps: u32) -> Self {
        Self { fps }
    }

    pub fn fixed_seconds(&self) -> f32 {
        1.0 / self.fps as f32
    }
}
