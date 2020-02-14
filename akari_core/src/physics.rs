use std::convert::TryFrom;

use crate::data::{Collider, CollisionDirection, Position, Velocity};

pub fn is_collision(a: Position, a_col: Collider, b: Position, b_col: Collider) -> bool {
    let (a_w, a_h) = (a.x + a_col.w, a.y + a_col.h);
    let (b_w, b_h) = (b.x + b_col.w, b.y + b_col.h);

    a.x < b_w && a_w > b.x && a.y < b_h && a_h > b.y
}

/// get the direction of the collision
/// requires velocities to be scaled with `time.fixed_seconds()`
pub fn collision_direction(
    (solid_pos, solid_col, solid_vel): (Position, Collider, Velocity),
    (other_pos, other_col, other_vel): (Position, Collider, Velocity),
) -> CollisionDirection {
    let relative_vel = other_vel - solid_vel;
    let vertical = if relative_vel.y > 0.0
        && other_col.h + other_pos.y - other_vel.y <= solid_pos.y - solid_vel.y
    {
        CollisionDirection::Below
    } else if relative_vel.y < 0.0
        && other_pos.y - other_vel.y >= solid_col.h + solid_pos.y - solid_vel.y
    {
        CollisionDirection::Above
    } else {
        CollisionDirection::None
    };

    let horizontal = if relative_vel.x > 0.0
        && other_col.w + other_pos.x - other_vel.x <= solid_pos.x - solid_vel.x
    {
        CollisionDirection::Left
    } else if relative_vel.x < 0.0
        && other_pos.x - other_vel.x >= solid_col.w + solid_pos.x - solid_vel.x
    {
        CollisionDirection::Right
    } else {
        CollisionDirection::None
    };

    CollisionDirection::try_from(vertical as u8 | horizontal as u8).unwrap()
}
