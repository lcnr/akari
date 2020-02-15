use crow_ecs::{Joinable, Storage};

use crate::{
    config::GravityConfig,
    data::{Gravity, Velocity},
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
        #[cfg(feature = "profiler")]
        profile_scope!("run");

        for (_gravity, mut velocity) in (&gravity, velocities).join() {
            velocity.y += gravity_config.acceleration * time.fixed_seconds();
            velocity.y = f32::max(velocity.y, gravity_config.terminal_velocity);
        }
    }
}
