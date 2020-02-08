mod bridge_collision;
pub mod draw;
mod fixed_collision;
mod gravity;
mod input_buffer;
mod physics;
mod player;

pub use bridge_collision::BridgeCollisionSystem;
pub use fixed_collision::FixedCollisionSystem;
pub use gravity::GravitySystem;
pub use input_buffer::InputBufferSystem;
pub use physics::PhysicsSystem;
pub use player::PlayerStateMachine;

#[derive(Debug)]
pub struct Systems {
    pub input_buffer: InputBufferSystem,
    pub gravity: GravitySystem,
    pub physics: PhysicsSystem,
    pub bridge_collision: BridgeCollisionSystem,
    pub fixed_collision: FixedCollisionSystem,
    pub player: PlayerStateMachine,
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
            gravity: GravitySystem,
            physics: PhysicsSystem::new(),
            bridge_collision: BridgeCollisionSystem,
            fixed_collision: FixedCollisionSystem::new(),
            player: PlayerStateMachine,
        }
    }
}
