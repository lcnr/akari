use crow::Context;

use crate::{
    config::{Config, PlayerAnimationsConfig},
    data::{
        Camera, Collider, ColliderType, Components, Depth, Gravity, PlayerAnimations, PlayerState,
        Position, Velocity,
    },
    ressources::Ressources,
};

pub fn player(
    ctx: &mut Context,
    c: &mut Components,
    r: &mut Ressources,
) -> Result<(), crow::Error> {
    #[cfg(feature = "profiler")]
    profile_scope!("player");

    let player = c.new_entity();

    c.positions.insert(player, r.last_save.position);
    c.colliders.insert(
        player,
        Collider {
            w: 7.0,
            h: 15.0,
            ty: ColliderType::Player,
        },
    );

    c.velocities.insert(player, Velocity { x: 0.0, y: 0.0 });
    c.gravity.insert(player, Gravity);
    c.player_state.insert(player, PlayerState::Grounded);
    c.depths.insert(player, Depth::Player);

    let player_animations = PlayerAnimations::from_config(
        ctx,
        &mut r.animation_storage,
        PlayerAnimationsConfig::load("ressources/player/animations.ron").unwrap(),
    )?;

    c.animations
        .insert(player, r.animation_storage.start(player_animations.idle));
    c.player_animations.insert(player, player_animations);

    Ok(())
}

pub fn camera(c: &mut Components, r: &mut Ressources) {
    let camera = c.new_entity();

    c.cameras.insert(camera, Camera);

    c.positions.insert(camera, Position { x: 0.0, y: 0.0 });

    c.velocities.insert(camera, Velocity { x: 0.0, y: 0.0 });

    c.colliders.insert(
        camera,
        Collider {
            w: r.config.window.size.0 as f32,
            h: r.config.window.size.1 as f32,
            ty: ColliderType::Camera,
        },
    );
}
