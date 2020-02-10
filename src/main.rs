#![allow(clippy::too_many_arguments)]
#![allow(clippy::match_ref_pats)]
#![warn(clippy::cast_lossless)]

#[cfg(feature = "profiler")]
#[macro_use]
extern crate thread_profiler;

#[macro_use]
extern crate log;

use std::io;

use crow::{
    glutin::{EventsLoop, WindowBuilder},
    Context, DrawConfig, Texture,
};

pub mod data;
pub mod environment;
pub mod init;
pub mod input;
pub mod physics;
pub mod ressources;
pub mod spritesheet;
pub mod systems;
pub mod time;

use systems::*;

const ARENA_WIDTH: usize = 16;
const ARENA_HEIGHT: usize = 12;
const GAME_SIZE: (u32, u32) = (20 * ARENA_WIDTH as u32, 20 * ARENA_HEIGHT as u32);
const WINDOW_SCALE: u32 = 3;
const FPS: u32 = 60;

fn main() -> Result<(), crow::Error> {
    #[cfg(feature = "profiler")]
    thread_profiler::register_thread_with_profiler();

    let mut ctx = Context::new(
        WindowBuilder::new().with_dimensions(From::from((
            GAME_SIZE.0 * WINDOW_SCALE,
            GAME_SIZE.1 * WINDOW_SCALE,
        ))),
        EventsLoop::new(),
    )?;
    let mut surface = ctx.window_surface();
    let mut screen_buffer = Texture::new(&mut ctx, GAME_SIZE)?;

    let mut c = data::Components::new();
    let mut r = ressources::Ressources::new(FPS);
    let mut s = Systems::new();

    let config = environment::EnvironmentConfig::load("ressources/environment.ron").unwrap();

    init::player(&mut ctx, &mut c, &mut r)?;

    let _e = environment::Environment::load(&mut ctx, &mut c, &config)?;

    loop {
        #[cfg(feature = "profiler")]
        profile_scope!("frame");

        if r.input_state.update(ctx.events_loop()) {
            break;
        }

        ctx.clear_color(&mut screen_buffer, (0.3, 0.3, 0.8, 1.0))?;
        screen_buffer.clear_depth(&mut ctx)?;

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

        s.animation
            .run(&mut c.sprites, &mut c.animations, &mut r.animation_storage);

        draw::scene(
            &mut ctx,
            &mut screen_buffer,
            &c.positions,
            &c.sprites,
            &c.depths,
        )?;
        draw::debug_colliders(&mut ctx, &mut screen_buffer, &c.positions, &c.colliders)?;

        ctx.draw(
            &mut surface,
            &screen_buffer,
            (0, 0),
            &DrawConfig {
                scale: (WINDOW_SCALE, WINDOW_SCALE),
                ..Default::default()
            },
        )?;
        ctx.finalize_frame()?;
        r.time.frame();
    }

    #[cfg(feature = "profiler")]
    thread_profiler::write_profile("profile.json");
    Ok(())
}

#[derive(Debug)]
pub enum LoadError {
    IoError(io::Error),
    DeserializeError(ron::de::Error),
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        LoadError::IoError(err)
    }
}

impl From<ron::de::Error> for LoadError {
    fn from(err: ron::de::Error) -> Self {
        LoadError::DeserializeError(err)
    }
}
