#[macro_use]
extern crate log;

use crow::{
    glutin::{EventsLoop, WindowBuilder},
    Context,
};

pub mod data;
pub mod input;
pub mod physics;
pub mod ressources;
pub mod systems;
pub mod time;

use input::InputState;
use systems::*;

const FPS: u32 = 60;

fn main() -> Result<(), crow::Error> {
    let mut ctx = Context::new(WindowBuilder::new(), EventsLoop::new())?;
    let mut surface = ctx.window_surface();

    let mut input_state = InputState::new();
    let mut c = data::Components::new();
    let r = ressources::Ressources::new(FPS);
    let mut systems = Systems::new();

    let a = c.new_entity();
    let b = c.new_entity();

    c.positions.insert(a, data::Position { x: 20.0, y: 40.0 });
    c.positions.insert(b, data::Position { x: 200.0, y: 100.0 });
    c.colliders.insert(
        a,
        data::Collider {
            w: 200.0,
            h: 200.0,
            ty: data::ColliderType::Player,
        },
    );
    c.colliders.insert(
        b,
        data::Collider {
            w: 200.0,
            h: 200.0,
            ty: data::ColliderType::Environment,
        },
    );

    loop {
        if input_state.update(ctx.events_loop()) {
            break;
        }

        ctx.clear_color(&mut surface, (0.3, 0.3, 0.8, 1.0))?;

        // use input / update player
        systems
            .gravity
            .run(&c.gravity, &mut c.velocities, &r.time, &r.gravity);

        let mut collisions = systems.physics.run(
            &c.velocities,
            &c.colliders,
            &mut c.previous_positions,
            &mut c.positions,
            &mut c.grounded,
            &r.time,
        );

        systems.bridge_collision.run(
            &c.positions,
            &c.previous_positions,
            &c.colliders,
            &c.ignore_bridges,
            &mut collisions,
        );

        systems.fixed_collision.run(
            &mut c.positions,
            &mut c.grounded,
            &mut c.wall_collisions,
            &mut c.velocities,
            &c.colliders,
            &r.time,
            &collisions,
        );

        // destruction timer

        // animation system

        draw::debug_colliders(&mut ctx, &mut surface, &c.positions, &c.colliders)?;
        ctx.finalize_frame()?;
    }

    Ok(())
}
