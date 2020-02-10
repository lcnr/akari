use crow::Context;

use crow_anim::Animation;

use crate::{
    config::{Config, SpriteSheetConfig},
    data::{
        Collider, ColliderType, Components, Depth, Gravity, PlayerAnimations, PlayerState,
        Position, Velocity,
    },
    ressources::Ressources,
    spritesheet::SpriteSheet,
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

    let jumping_sheet = SpriteSheet::from_config(
        ctx,
        &SpriteSheetConfig::load("ressources/player/jumping_sheet.ron").unwrap(),
    )?;
    let mut jumping_animation = Animation::empty();
    for s in jumping_sheet.iter() {
        jumping_animation.frames.push(s.clone());
    }
    let jumping = r.animation_storage.insert(jumping_animation);
    r.animation_storage.get_mut(jumping).next = jumping;

    let on_jump_sheet = SpriteSheet::from_config(
        ctx,
        &SpriteSheetConfig::load("ressources/player/on_jump_sheet.ron").unwrap(),
    )?;
    let mut on_jump_animation = Animation::empty();
    for s in on_jump_sheet.iter() {
        for _ in 0..4 {
            on_jump_animation.frames.push(s.clone());
        }
    }
    let on_jump = r.animation_storage.insert(on_jump_animation);
    r.animation_storage.get_mut(on_jump).next = jumping;

    let falling_sheet = SpriteSheet::from_config(
        ctx,
        &SpriteSheetConfig::load("ressources/player/falling_sheet.ron").unwrap(),
    )?;
    let mut falling_animation = Animation::empty();
    for s in falling_sheet.iter() {
        falling_animation.frames.push(s.clone());
    }
    let falling = r.animation_storage.insert(falling_animation);
    r.animation_storage.get_mut(falling).next = falling;

    let start_falling_sheet = SpriteSheet::from_config(
        ctx,
        &SpriteSheetConfig::load("ressources/player/start_falling_sheet.ron").unwrap(),
    )?;
    let mut start_falling_animation = Animation::empty();
    for s in start_falling_sheet.iter() {
        for _ in 0..4 {
            start_falling_animation.frames.push(s.clone());
        }
    }
    let start_falling = r.animation_storage.insert(start_falling_animation);
    r.animation_storage.get_mut(start_falling).next = falling;

    c.depths.insert(player, Depth::Player);
    c.player_animations.insert(
        player,
        PlayerAnimations {
            idle,
            on_jump,
            jumping,
            start_falling,
            falling,
        },
    );
    c.animations.insert(player, r.animation_storage.start(idle));

    Ok(())
}
