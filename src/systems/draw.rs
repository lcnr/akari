use crow::{Context, DrawTarget};

use crow_ecs::{Joinable, Storage};

use crate::data::{Collider, ColliderType, Position};

pub fn debug_colliders<T: DrawTarget>(
    ctx: &mut Context,
    target: &mut T,
    positions: &Storage<Position>,
    colliders: &Storage<Collider>,
) -> Result<(), crow::Error> {
    for (&Position { x, y }, collider) in (positions, colliders).join() {
        let color = match collider.ty {
            ColliderType::Player => (0.2, 0.9, 0.2, 0.8),
            ColliderType::PlayerDamage => (0.8, 0.2, 0.3, 0.8),
            ColliderType::Environment => (0.3, 0.7, 0.7, 0.8),
            ColliderType::Bridge => (0.7, 0.3, 0.7, 0.8),
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
