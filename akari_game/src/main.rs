#[cfg(feature = "profiler")]
#[macro_use]
extern crate thread_profiler;

use crow::{
    glutin::{EventsLoop, WindowBuilder},
    Context,
};

use akari_core::{
    config::{Config, EnvironmentConfig},
    environment::Environment,
    input,
    systems::draw,
    GlobalState, FPS, GAME_SIZE, WINDOW_SCALE,
};

mod init;

fn main() -> Result<(), crow::Error> {
    #[cfg(feature = "profiler")]
    thread_profiler::register_thread_with_profiler();

    let icon = akari_core::load_window_icon("textures/window_icon.png").unwrap();

    let mut ctx = Context::new(
        WindowBuilder::new()
            .with_dimensions(From::from((
                GAME_SIZE.0 * WINDOW_SCALE,
                GAME_SIZE.1 * WINDOW_SCALE,
            )))
            .with_title("Akari")
            .with_window_icon(Some(icon)),
        EventsLoop::new(),
    )?;

    let mut game = GlobalState::new(FPS);

    let config = EnvironmentConfig::load("ressources/environment.ron").unwrap();

    init::player(&mut ctx, &mut game.c, &mut game.r)?;

    let mut e = Some(Environment::load(&mut ctx, &mut game.c, &config)?);

    game.run(&mut ctx, |ctx, screen_buffer, s, r, c| {
        if r.input_state.update(ctx.events_loop()) {
            return Ok(true);
        }

        s.input_buffer.run(
            r.input_state.events(),
            &mut r.pressed_space,
            &r.config.input_buffer,
        );

        if r.input_state.down == input::ButtonState::Down {
            if let Some(e) = e.take() {
                e.delete(c);
            }
        }

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

        s.player.run(c, r, &collisions);

        // destruction timer

        s.animation
            .run(&mut c.sprites, &mut c.animations, &mut r.animation_storage);

        draw::scene(
            ctx,
            screen_buffer,
            &c.positions,
            &c.sprites,
            &c.depths,
            &c.mirrored,
            &c.colliders,
        )?;
        draw::debug_colliders(ctx, screen_buffer, &c.positions, &c.colliders)?;

        Ok(false)
    })?;

    #[cfg(feature = "profiler")]
    thread_profiler::write_profile("profile.json");

    Ok(())
}
