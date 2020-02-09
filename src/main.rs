#![allow(clippy::too_many_arguments)]
#![allow(clippy::match_ref_pats)]
#![warn(clippy::cast_lossless)]

#[macro_use]
extern crate log;

use crow::{
    glutin::{EventsLoop, WindowBuilder},
    Context,
};

pub mod data;
pub mod environment;
pub mod input;
pub mod physics;
pub mod ressources;
pub mod systems;
pub mod time;

use systems::*;

const ARENA_WIDTH: usize = 16;
const ARENA_HEIGHT: usize = 12;
const GAME_SIZE: (u32, u32) = (20 * ARENA_WIDTH as u32, 20 * ARENA_HEIGHT as u32);
const FPS: u32 = 60;

fn main() -> Result<(), crow::Error> {
    let mut ctx = Context::new(
        WindowBuilder::new().with_dimensions(From::from(GAME_SIZE)),
        EventsLoop::new(),
    )?;
    let mut surface = ctx.window_surface();

    let mut c = data::Components::new();
    let mut r = ressources::Ressources::new(FPS);
    let mut s = Systems::new();

    let player = c.new_entity();

    c.positions
        .insert(player, data::Position { x: 50.0, y: 100.0 });
    c.colliders.insert(
        player,
        data::Collider {
            w: 12.0,
            h: 16.0,
            ty: data::ColliderType::Player,
        },
    );
    c.velocities
        .insert(player, data::Velocity { x: 0.0, y: 0.0 });
    c.gravity.insert(player, data::Gravity);
    c.player_state.insert(player, data::PlayerState::Idle);

    let f = std::fs::File::open("ressources/environment.rs").expect("Failed opening file");
    let config: environment::EnvironmentConfig = match ron::de::from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };

    let env = environment::Environment::load(&mut c, &config);

    loop {
        if r.input_state.update(ctx.events_loop()) {
            break;
        }

        ctx.clear_color(&mut surface, (0.3, 0.3, 0.8, 1.0))?;

        s.input_buffer.run(
            r.input_state.events(),
            &mut r.pressed_space,
            &r.config.input_buffer,
        );

        s.gravity
            .run(&c.gravity, &mut c.velocities, &r.time, &r.config.gravity);

        let mut collisions = s.physics.run(
            &c.velocities,
            &c.colliders,
            &mut c.previous_positions,
            &mut c.positions,
            &mut c.grounded,
            &r.time,
        );

        s.bridge_collision.run(
            &c.positions,
            &c.previous_positions,
            &c.colliders,
            &c.ignore_bridges,
            &mut collisions,
        );

        s.fixed_collision.run(
            &mut c.positions,
            &mut c.grounded,
            &mut c.wall_collisions,
            &mut c.velocities,
            &c.colliders,
            &r.time,
            &collisions,
        );

        s.player.run(&mut c, &mut r, &collisions);

        // destruction timer

        // animation system

        draw::debug_colliders(&mut ctx, &mut surface, &c.positions, &c.colliders)?;
        ctx.finalize_frame()?;
        r.time.frame();
    }

    Ok(())
}
