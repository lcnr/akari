use crow_ecs::{Joinable, Storage};

use crate::{
    data::{Gravity, Velocity},
    ressources::GravityConfig,
    time::Time,
};

#[derive(Debug)]
pub struct GravitySystem;

impl GravitySystem {
    pub fn run(
        &mut self,
        gravity: &Storage<Gravity>,
        velocities: &mut Storage<Velocity>,
        time: &Time,
        gravity_config: &GravityConfig,
    ) {
        for (_gravity, mut velocity) in (&gravity, velocities).join() {
            velocity.y += gravity_config.acceleration * time.fixed_seconds();
            velocity.y = f32::max(velocity.y, gravity_config.terminal_velocity);
        }
    }
}
