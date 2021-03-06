use std::mem;

use crow::Context;

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

#[cfg(feature = "editor")]
mod editor;

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

#[cfg(feature = "editor")]
pub use editor::EditorSystem;

use crate::{
    data::Components,
    ressources::{DelayedAction, Ressources},
};

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
    #[cfg(feature = "editor")]
    pub editor: EditorSystem,
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
            #[cfg(feature = "editor")]
            editor: EditorSystem::new(),
        }
    }

    pub fn delayed_actions(
        &mut self,
        ctx: &mut Context,
        c: &mut Components,
        r: &mut Ressources,
    ) -> Result<(), crow::Error> {
        let actions = mem::replace(&mut r.delayed_actions, Vec::new()).into_iter();
        r.delayed_actions = actions
            .filter_map(|a| {
                if let Some(frames_left) = a.frames_left.checked_sub(1) {
                    Some(Ok(DelayedAction { frames_left, ..a }))
                } else {
                    match (a.action)(ctx, self, c, r) {
                        Ok(()) => None,
                        Err(err) => Some(Err(err)),
                    }
                }
            })
            .collect::<Result<_, _>>()?;

        Ok(())
    }
}
