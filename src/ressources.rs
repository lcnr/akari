use crate::time::Time;

pub struct Ressources {
    pub time: Time,
    pub gravity: GravityConfig,
}

impl Ressources {
    pub fn new(fps: u32) -> Self {
        Ressources {
            time: Time::new(fps),
            gravity: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GravityConfig {
    pub acceleration: f32,
    pub terminal_velocity: f32,
}

impl Default for GravityConfig {
    fn default() -> Self {
        GravityConfig {
            acceleration: -100.0,
            terminal_velocity: -200.0,
        }
    }
}
