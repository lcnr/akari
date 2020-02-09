use crow::{Context, DrawConfig, DrawTarget};

use crow_ecs::{Joinable, Storage};

use crate::data::{Collider, ColliderType, Depth, Position, Sprite};

pub fn scene<T: DrawTarget>(
    ctx: &mut Context,
    target: &mut T,
    positions: &Storage<Position>,
    sprites: &Storage<Sprite>,
    depths: &Storage<Depth>,
) -> Result<(), crow::Error> {
    #[cfg(feature = "profiler")]
    profile_scope!("scene");

    for (&Position { x, y }, sprite, depth) in (positions, sprites, depths.maybe()).join() {
        ctx.draw(
            target,
            &sprite.texture,
            (
                x.round() as i32 - sprite.offset.0,
                y.round() as i32 - sprite.offset.1,
            ),
            &DrawConfig {
                depth: depth.copied().map(From::from),
                ..Default::default()
            },
        )?;
    }

    Ok(())
}

pub fn debug_colliders<T: DrawTarget>(
    ctx: &mut Context,
    target: &mut T,
    positions: &Storage<Position>,
    colliders: &Storage<Collider>,
) -> Result<(), crow::Error> {
    #[cfg(feature = "profiler")]
    profile_scope!("debug_colliders");

    for (&Position { x, y }, collider) in (positions, colliders).join() {
        let color = match collider.ty {
            ColliderType::Player => (0.0, 1.0, 0.0, 0.8),
            ColliderType::PlayerDamage => (1.0, 0.0, 0.0, 0.8),
            ColliderType::Environment => (0.0, 0.7, 0.7, 0.8),
            ColliderType::Bridge => (0.0, 0.0, 1.0, 0.8),
        };

        let xw = (x + collider.w).round() as i32;
        let yh = (y + collider.h).round() as i32;
        let x = x.round() as i32;
        let y = y.round() as i32;

        ctx.draw_line(target, (x, y), (xw, y), color)?;
        ctx.draw_line(target, (xw, y), (xw, yh), color)?;
        ctx.draw_line(target, (xw, yh), (x, yh), color)?;
        ctx.draw_line(target, (x, yh), (x, y), color)?;
    }

    Ok(())
}
