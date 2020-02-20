use akari::{
    config::{Config, GameConfig},
    environment::WorldData,
    init,
    save::SaveData,
    systems::draw,
    GlobalState,
};

fn main() -> Result<(), crow::Error> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::max())
        .init();

    #[cfg(feature = "profiler")]
    thread_profiler::register_thread_with_profiler();

    let config = GameConfig::load("ressources/game_config.ron").unwrap();
    let world_data = WorldData::load("ressources/environment/world.ron").unwrap();
    let save_data = SaveData::load("ressources/save/test_save.ron").unwrap();
    let mut game = GlobalState::new(config, world_data, save_data)?;

    init::player(&mut game.ctx, &mut game.c, &mut game.r)?;
    init::camera(&mut game.c, &mut game.r);
    game.s
        .environment
        .run(&mut game.ctx, &mut game.c, &mut game.r)?;

    let mut debug_draw = false;

    game.run(|ctx, screen_buffer, s, r, c| {
        if r.input_state.update(ctx.events_loop(), &r.config.window) {
            return Ok(true);
        }

        for event in r.input_state.events() {
            if &akari::input::InputEvent::KeyDown(r.config.input.debug_toggle) == event {
                debug_draw = !debug_draw;
            }
        }

        s.input_buffer.run(
            r.input_state.events(),
            &mut r.pressed_space,
            &r.config.input_buffer,
            &r.config.input,
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

        s.fadeout.run(&mut r.fadeout);

        s.animation
            .run(&mut c.sprites, &mut c.animations, &mut r.animation_storage);

        s.delayed_actions(ctx, c, r)?;

        #[cfg(feature = "editor")]
        s.editor.run(ctx, c, r)?;

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

        if debug_draw {
            draw::debug_colliders(ctx, screen_buffer, &c.positions, &c.colliders, &c.cameras)?;
        }

        Ok(false)
    })?;

    #[cfg(feature = "profiler")]
    thread_profiler::write_profile("profile.json");

    Ok(())
}
