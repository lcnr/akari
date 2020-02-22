use akari::{
    config::{Config, GameConfig},
    environment::WorldData,
    init,
    save::SaveData,
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

    game.run(akari::game_frame)?;

    #[cfg(feature = "profiler")]
    thread_profiler::write_profile("profile.json");

    Ok(())
}
