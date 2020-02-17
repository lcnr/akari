#[cfg(feature = "profiler")]
#[macro_use]
extern crate thread_profiler;

use akari_core::{
    config::{Config, GameConfig},
    environment::WorldData,
    systems::draw,
    GlobalState,
};

mod init;

fn main() -> Result<(), crow::Error> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::max())
        .init();

    #[cfg(feature = "profiler")]
    thread_profiler::register_thread_with_profiler();

    let config = GameConfig::load("ressources/game_config.ron").unwrap();
    let world_data = WorldData::load("ressources/environment/world.ron").unwrap();
    let mut game = GlobalState::new(config, world_data)?;

    init::player(&mut game.ctx, &mut game.c, &mut game.r)?;
    init::camera(&mut game.c, &mut game.r);
    game.s
        .environment
        .run(&mut game.ctx, &mut game.c, &mut game.r)?;

    game.run(|ctx, screen_buffer, s, r, c| {
        if r.input_state.update(ctx.events_loop()) {
            return Ok(true);
        }

        s.input_buffer.run(
            r.input_state.events(),
            &mut r.pressed_space,
            &r.config.input_buffer,
        );

        s.camera.run(
            &c.player_state,
            &c.positions,
            &c.previous_positions,
            &mut c.velocities,
            &c.cameras,
            &r.time,
            &r.config.camera,
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
            &c.previous_positions,
            &mut c.grounded,
            &mut c.wall_collisions,
            &mut c.velocities,
            &c.colliders,
            &collisions,
        );

        s.player.run(c, r, &collisions);

        s.environment.run(ctx, c, r)?;

        s.animation
            .run(&mut c.sprites, &mut c.animations, &mut r.animation_storage);

        s.lazy_update.run(c, r);

        draw::scene(
            ctx,
            screen_buffer,
            &c.positions,
            &c.sprites,
            &c.depths,
            &c.mirrored,
            &c.colliders,
            &c.cameras,
        )?;
        //draw::debug_colliders(ctx, screen_buffer, &c.positions, &c.colliders, &c.cameras)?;

        Ok(false)
    })?;

    #[cfg(feature = "profiler")]
    thread_profiler::write_profile("profile.json");

    Ok(())
}
