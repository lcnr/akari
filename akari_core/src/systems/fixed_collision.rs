//! TODO: reduce complexity of FixedCollisionSystem::run
use std::{collections::HashMap, convert::TryFrom};

use ordered_float::OrderedFloat;

use crow_ecs::{Entity, Storage};

use crate::{
    data::{
        Collider, ColliderType, Collision, CollisionDirection, Collisions, Grounded, Position,
        Velocity, WallCollision,
    },
    physics::collision_direction,
};

#[derive(Default, Debug)]
pub struct FixedCollisionSystem {
    moved: HashMap<Entity, Vec<Entity>>,
}

impl FixedCollisionSystem {
    pub fn new() -> Self {
        FixedCollisionSystem {
            moved: HashMap::new(),
        }
    }

    pub fn run(
        &mut self,
        mut positions: &mut Storage<Position>,
        previous_positions: &Storage<Position>,
        mut grounded: &mut Storage<Grounded>,
        mut wall_collisions: &mut Storage<WallCollision>,
        mut velocities: &mut Storage<Velocity>,
        colliders: &Storage<Collider>,
        collisions: &Collisions,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("run");

        wall_collisions.clear();
        for &Collision(e, p) in collisions.fixed.iter() {
            self.moved.entry(p).or_insert_with(Vec::new).push(e);
        }

        for (other, solids) in self.moved.drain() {
            let (other_prev_pos, other_col, other_vel) =
                prev_pos_col_vel(positions, previous_positions, colliders, velocities, other);

            let (unique, shared) =
                solids
                    .iter()
                    .copied()
                    .fold((0b0000, 0b1111), |(unique, shared), solid| {
                        let (solid_prev_pos, solid_col, solid_vel) = prev_pos_col_vel(
                            positions,
                            previous_positions,
                            colliders,
                            velocities,
                            solid,
                        );

                        let dir = collision_direction(
                            (solid_prev_pos, solid_col, solid_vel),
                            (other_prev_pos, other_col, other_vel),
                        ) as u8;
                        (unique | dir, shared & dir)
                    });

            match CollisionDirection::try_from(shared).expect("shared") {
                CollisionDirection::LeftAbove
                | CollisionDirection::Above
                | CollisionDirection::RightAbove => {
                    let solid = solids
                        .into_iter()
                        .max_by_key(|&entity| {
                            let entity_pos = positions.get(entity).copied().expect("entity_pos");
                            let entity_col = *colliders.get(entity).expect("entity_col");
                            OrderedFloat(entity_col.upper_border(entity_pos))
                        })
                        .unwrap();
                    resolve_collision(
                        CollisionDirection::Above,
                        other,
                        solid,
                        &mut positions,
                        &mut wall_collisions,
                        &mut grounded,
                        &mut velocities,
                        &colliders,
                    );
                }
                CollisionDirection::LeftBelow
                | CollisionDirection::Below
                | CollisionDirection::RightBelow => {
                    let solid = solids
                        .into_iter()
                        .min_by_key(|&entity| {
                            let entity_pos = positions.get(entity).copied().expect("entity_pos");
                            let entity_col = *colliders.get(entity).expect("entity_col");
                            OrderedFloat(entity_col.lower_border(entity_pos))
                        })
                        .unwrap();
                    resolve_collision(
                        CollisionDirection::Below,
                        other,
                        solid,
                        &mut positions,
                        &mut wall_collisions,
                        &mut grounded,
                        &mut velocities,
                        &colliders,
                    );
                }
                CollisionDirection::Right => {
                    let solid = solids
                        .into_iter()
                        .max_by_key(|&entity| {
                            let entity_pos = positions.get(entity).copied().expect("entity_pos");
                            let entity_col = *colliders.get(entity).expect("entity_col");
                            OrderedFloat(entity_col.right_border(entity_pos))
                        })
                        .unwrap();
                    resolve_collision(
                        CollisionDirection::Right,
                        other,
                        solid,
                        &mut positions,
                        &mut wall_collisions,
                        &mut grounded,
                        &mut velocities,
                        &colliders,
                    );
                }
                CollisionDirection::Left => {
                    let solid = solids
                        .into_iter()
                        .min_by_key(|&entity| {
                            let entity_pos = positions.get(entity).copied().expect("entity_pos");
                            let entity_col = *colliders.get(entity).expect("entity_col");
                            OrderedFloat(entity_col.left_border(entity_pos))
                        })
                        .unwrap();
                    resolve_collision(
                        CollisionDirection::Left,
                        other,
                        solid,
                        &mut positions,
                        &mut wall_collisions,
                        &mut grounded,
                        &mut velocities,
                        &colliders,
                    );
                }
                CollisionDirection::None => collision_none(
                    unique,
                    other,
                    solids,
                    positions,
                    previous_positions,
                    grounded,
                    wall_collisions,
                    velocities,
                    colliders,
                ),
            };
        }
    }
}

fn build_solid_shared<'a>(
    positions: &'a Storage<Position>,
    previous_positions: &'a Storage<Position>,
    colliders: &'a Storage<Collider>,
    velocities: &'a Storage<Velocity>,
    (other_prev_pos, other_col, other_vel): (Position, Collider, Velocity),
) -> impl Fn(&Entity) -> (Entity, Position, Collider, CollisionDirection) + 'a + Copy {
    move |&solid| {
        let solid_pos = positions.get(solid).copied().unwrap();
        let solid_prev_pos = previous_positions.get(solid).copied().unwrap_or(solid_pos);
        let solid_col = colliders.get(solid).copied().unwrap();
        let solid_vel = velocities.get(solid).copied().unwrap_or_default();

        let direction = collision_direction(
            (solid_prev_pos, solid_col, solid_vel),
            (other_prev_pos, other_col, other_vel),
        );

        (solid, solid_pos, solid_col, direction)
    }
}

fn collision_none(
    unique: u8,
    other: Entity,
    solids: Vec<Entity>,
    mut positions: &mut Storage<Position>,
    previous_positions: &Storage<Position>,
    mut grounded: &mut Storage<Grounded>,
    mut wall_collisions: &mut Storage<WallCollision>,
    mut velocities: &mut Storage<Velocity>,
    colliders: &Storage<Collider>,
) {
    let (other_prev_pos, other_col, other_vel) =
        prev_pos_col_vel(positions, previous_positions, colliders, velocities, other);

    let (vertical, horizontal) = if let (Ok(vertical), Ok(horizontal)) = (
        CollisionDirection::try_from(unique & 0b0101),
        CollisionDirection::try_from(unique & 0b1010),
    ) {
        (vertical, horizontal)
    } else {
        warn!("unit is getting squashed!");
        return;
    };

    if vertical == CollisionDirection::None || horizontal == CollisionDirection::None {
        warn!("unit is currently inside of a collider");
        return;
    }

    let solid_shared = build_solid_shared(
        positions,
        previous_positions,
        colliders,
        velocities,
        (other_prev_pos, other_col, other_vel),
    );

    let entity_id = |(e, _, _, _)| e;

    let vertical_solid = if vertical == CollisionDirection::Above {
        solids
            .iter()
            .map(solid_shared)
            .filter(|&(_, _, _, dir)| dir & CollisionDirection::Above != CollisionDirection::None)
            .max_by_key(|&(_, solid_pos, solid_col, _)| {
                OrderedFloat(solid_col.upper_border(solid_pos))
            })
            .map(entity_id)
            .unwrap()
    } else {
        solids
            .iter()
            .map(solid_shared)
            .filter(|&(_, _, _, dir)| dir & CollisionDirection::Below != CollisionDirection::None)
            .min_by_key(|&(_, solid_pos, solid_col, _)| {
                OrderedFloat(solid_col.lower_border(solid_pos))
            })
            .map(entity_id)
            .unwrap()
    };

    resolve_collision(
        vertical,
        other,
        vertical_solid,
        &mut positions,
        &mut wall_collisions,
        &mut grounded,
        &mut velocities,
        &colliders,
    );

    let solid_shared = build_solid_shared(
        positions,
        previous_positions,
        colliders,
        velocities,
        (other_prev_pos, other_col, other_vel),
    );

    let horizontal_solid = if horizontal == CollisionDirection::Right {
        solids
            .iter()
            .map(solid_shared)
            .filter(|&(_, _, _, dir)| dir & CollisionDirection::Right != CollisionDirection::None)
            .max_by_key(|&(_, solid_pos, solid_col, _)| {
                OrderedFloat(solid_col.right_border(solid_pos))
            })
            .map(entity_id)
            .unwrap()
    } else {
        solids
            .iter()
            .map(solid_shared)
            .filter(|&(_, _, _, dir)| dir & CollisionDirection::Left != CollisionDirection::None)
            .min_by_key(|&(_, solid_pos, solid_col, _)| {
                OrderedFloat(solid_col.left_border(solid_pos))
            })
            .map(entity_id)
            .unwrap()
    };
    resolve_collision(
        horizontal,
        other,
        horizontal_solid,
        &mut positions,
        &mut wall_collisions,
        &mut grounded,
        &mut velocities,
        &colliders,
    );
}

fn prev_pos_col_vel(
    positions: &Storage<Position>,
    previous_positions: &Storage<Position>,
    colliders: &Storage<Collider>,
    velocities: &Storage<Velocity>,
    entity: Entity,
) -> (Position, Collider, Velocity) {
    let pos = positions.get(entity).copied().unwrap();
    let prev_pos = previous_positions.get(entity).copied().unwrap_or(pos);
    let col = colliders.get(entity).copied().unwrap();
    let vel = velocities.get(entity).copied().unwrap_or_default();
    (prev_pos, col, vel)
}

fn resolve_collision(
    direction: CollisionDirection,
    other: Entity,
    solid: Entity,
    positions: &mut Storage<Position>,
    wall_collisions: &mut Storage<WallCollision>,
    grounded: &mut Storage<Grounded>,
    velocities: &mut Storage<Velocity>,
    colliders: &Storage<Collider>,
) {
    let solid_pos = positions.get(solid).copied().expect("solid_pos");
    let solid_col = colliders.get(solid).copied().expect("solid_col");
    let unscaled_solid_vel = velocities.get(solid).copied().unwrap_or_default();

    let other_pos = positions.get_mut(other).expect("other_pos");
    let other_col = colliders.get(other).copied().expect("other_col");

    match direction {
        CollisionDirection::Below => {
            other_pos.y = solid_col.lower_border(solid_pos) - other_col.h;
            // set speed equal to ground speed
            let mut other_vel = Velocity { x: 0.0, y: 0.0 };
            velocities.get_mut(other).unwrap_or(&mut other_vel).y = unscaled_solid_vel.y;
        }
        CollisionDirection::Above => {
            match solid_col.ty {
                ColliderType::Bridge | ColliderType::Environment => {
                    grounded.insert(other, Grounded);
                }
                ColliderType::Player
                | ColliderType::PlayerDamage
                | ColliderType::Camera
                | ColliderType::CameraRestriction => (),
            }

            other_pos.y = solid_col.upper_border(solid_pos);
            // set speed equal to ground speed
            let mut other_vel = Velocity { x: 0.0, y: 0.0 };
            velocities.get_mut(other).unwrap_or(&mut other_vel).y = unscaled_solid_vel.y;
        }
        CollisionDirection::Left => {
            other_pos.x = solid_col.left_border(solid_pos) - other_col.w;
            let mut other_vel = Velocity { x: 0.0, y: 0.0 };
            velocities.get_mut(other).unwrap_or(&mut other_vel).x = unscaled_solid_vel.x;
            match solid_col.ty {
                ColliderType::Bridge | ColliderType::Environment => {
                    wall_collisions.insert(other, WallCollision::Left)
                }
                ColliderType::Player
                | ColliderType::PlayerDamage
                | ColliderType::Camera
                | ColliderType::CameraRestriction => None,
            };
        }
        CollisionDirection::Right => {
            other_pos.x = solid_col.right_border(solid_pos);
            let mut other_vel = Velocity { x: 0.0, y: 0.0 };
            velocities.get_mut(other).unwrap_or(&mut other_vel).x = unscaled_solid_vel.x;
            match solid_col.ty {
                ColliderType::Bridge | ColliderType::Environment => {
                    wall_collisions.insert(other, WallCollision::Right)
                }
                ColliderType::Player
                | ColliderType::PlayerDamage
                | ColliderType::Camera
                | ColliderType::CameraRestriction => None,
            };
        }
        CollisionDirection::None => (),
        err => panic!("resolve collision requires a simple direction: {:?}", err),
    }
}
