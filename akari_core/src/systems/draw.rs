use crow::{Context, DrawConfig, DrawTarget};

use crow_ecs::{Joinable, SparseStorage, Storage};

use crow_anim::Sprite;

use crate::{
    data::{Collider, ColliderType, Depth, Mirrored, Position},
    ressources::Camera,
};

pub fn scene<T: DrawTarget>(
    ctx: &mut Context,
    target: &mut T,
    positions: &Storage<Position>,
    sprites: &Storage<Sprite>,
    depths: &Storage<Depth>,
    mirrored: &SparseStorage<Mirrored>,
    colliders: &Storage<Collider>,
    camera: &Camera,
) -> Result<(), crow::Error> {
    #[cfg(feature = "profiler")]
    profile_scope!("scene");

    for (&Position { x, y }, sprite, depth, mirrored, collider) in (
        positions,
        sprites,
        depths.maybe(),
        mirrored.maybe(),
        colliders.maybe(),
    )
        .join()
    {
        let x = (x - camera.position.x).round() as i32;
        let y = (y - camera.position.y).round() as i32 - sprite.offset.1;

        let (x, flip_horizontally) = if let Some(Mirrored) = mirrored {
            let offset = sprite.texture.width() as i32
                - sprite.offset.0
                - collider.map_or(0, |c| c.w.round() as i32);

            (x - offset, true)
        } else {
            (x - sprite.offset.0, false)
        };

        ctx.draw(
            target,
            &sprite.texture,
            (x, y),
            &DrawConfig {
                depth: depth.copied().map(From::from),
                flip_horizontally,
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
    camera: &Camera,
) -> Result<(), crow::Error> {
    #[cfg(feature = "profiler")]
    profile_scope!("debug_colliders");

    for (&Position { x, y }, collider) in (positions, colliders).join() {
        let x = x - camera.position.x;
        let y = y - camera.position.y;

        let color = match collider.ty {
            ColliderType::Player => (0.0, 1.0, 0.0, 0.4),
            ColliderType::PlayerDamage => (1.0, 0.0, 0.0, 0.8),
            ColliderType::Environment => (0.0, 0.7, 0.7, 0.8),
            ColliderType::Bridge => (0.0, 0.0, 1.0, 0.8),
        };

        let xw = (x + collider.w).round() as i32;
        let yh = (y + collider.h).round() as i32;
        let x = x.round() as i32;
        let y = y.round() as i32;

        ctx.debug_rectangle(target, (x, y), (xw - 1, yh - 1), color)?;
    }

    Ok(())
}
