mod bridge_collision;
pub mod draw;
mod fixed_collision;
mod gravity;
mod physics;

pub use bridge_collision::BridgeCollisionSystem;
pub use fixed_collision::FixedCollisionSystem;
pub use gravity::GravitySystem;
pub use physics::PhysicsSystem;

#[derive(Debug)]
pub struct Systems {
    pub gravity: GravitySystem,
    pub physics: PhysicsSystem,
    pub bridge_collision: BridgeCollisionSystem,
    pub fixed_collision: FixedCollisionSystem,
}

impl Default for Systems {
    fn default() -> Self {
        Systems::new()
    }
}

impl Systems {
    pub fn new() -> Self {
        Systems {
            gravity: GravitySystem,
            physics: PhysicsSystem::new(),
            bridge_collision: BridgeCollisionSystem,
            fixed_collision: FixedCollisionSystem::new(),
        }
    }
}
