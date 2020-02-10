use crow::Context;

use crow_anim::Animation;

use crate::{
    data::{
        Collider, ColliderType, Components, Depth, Gravity, PlayerAnimations, PlayerState,
        Position, Velocity,
    },
    ressources::Ressources,
    spritesheet::{SpriteSheet, SpriteSheetConfig},
};

pub fn player(
    ctx: &mut Context,
    c: &mut Components,
    r: &mut Ressources,
) -> Result<(), crow::Error> {
    let player = c.new_entity();

    c.positions.insert(player, Position { x: 50.0, y: 100.0 });
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

    let idle_sheet = SpriteSheet::from_config(
        ctx,
        &SpriteSheetConfig::load("ressources/player/idle_sheet.ron").unwrap(),
    )?;
    let mut idle_animation = Animation::empty();
    for s in idle_sheet.iter() {
        for _ in 0..10 {
            idle_animation.frames.push(s.clone());
        }
    }
    let idle = r.animation_storage.insert(idle_animation);
    r.animation_storage.get_mut(idle).next = idle;

    let jump_sheet = SpriteSheet::from_config(
        ctx,
        &SpriteSheetConfig::load("ressources/player/jump_sheet.ron").unwrap(),
    )?;
    let mut jump_animation = Animation::empty();
    for s in jump_sheet.iter() {
        for _ in 0..5 {
            jump_animation.frames.push(s.clone());
        }
    }
    let jump = r.animation_storage.insert(jump_animation);
    r.animation_storage.get_mut(jump).next = idle;

    c.depths.insert(player, Depth::Player);
    c.player_animations
        .insert(player, PlayerAnimations { idle, jump });
    c.animations.insert(player, r.animation_storage.start(idle));

    Ok(())
}
