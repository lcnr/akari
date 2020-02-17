use std::mem;

mod animation;
mod bridge_collision;
mod camera;
pub mod draw;
mod fadeout;
mod fixed_collision;
mod gravity;
mod input_buffer;
mod physics;
mod player;

pub use crate::environment::EnvironmentSystem;
pub use animation::AnimationSystem;
pub use bridge_collision::BridgeCollisionSystem;
pub use camera::CameraSystem;
pub use fadeout::FadeoutSystem;
pub use fixed_collision::FixedCollisionSystem;
pub use gravity::GravitySystem;
pub use input_buffer::InputBufferSystem;
pub use physics::PhysicsSystem;
pub use player::PlayerStateMachine;

use crate::{data::Components, ressources::Ressources};

#[derive(Debug)]
pub struct Systems {
    pub input_buffer: InputBufferSystem,
    pub camera: CameraSystem,
    pub gravity: GravitySystem,
    pub physics: PhysicsSystem,
    pub bridge_collision: BridgeCollisionSystem,
    pub fadeout: FadeoutSystem,
    pub fixed_collision: FixedCollisionSystem,
    pub player: PlayerStateMachine,
    pub environment: EnvironmentSystem,
    pub animation: AnimationSystem,
    pub lazy_update: LazyUpdateSystem,
}

impl Default for Systems {
    fn default() -> Self {
        Systems::new()
    }
}

impl Systems {
    pub fn new() -> Self {
        Systems {
            input_buffer: InputBufferSystem,
            camera: CameraSystem,
            gravity: GravitySystem,
            physics: PhysicsSystem::new(),
            bridge_collision: BridgeCollisionSystem,
            fadeout: FadeoutSystem,
            fixed_collision: FixedCollisionSystem::new(),
            player: PlayerStateMachine,
            environment: EnvironmentSystem,
            animation: AnimationSystem,
            lazy_update: LazyUpdateSystem,
        }
    }
}

#[derive(Debug)]
pub struct LazyUpdateSystem;

impl LazyUpdateSystem {
    pub fn run(&mut self, c: &mut Components, r: &mut Ressources) {
        for update in mem::replace(&mut r.lazy_update, Vec::new()) {
            update(c, r);
        }
    }
}
