use std::{collections::HashMap, convert::TryFrom};

use ordered_float::OrderedFloat;

use crow_ecs::{Entity, Storage};

use crate::{
    data::{
        Collider, ColliderType, Collision, CollisionDirection, Collisions, Grounded, Position,
        Velocity, WallCollision,
    },
    physics::collision_direction,
    time::Time,
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
        mut grounded: &mut Storage<Grounded>,
        mut wall_collisions: &mut Storage<WallCollision>,
        mut velocities: &mut Storage<Velocity>,
        colliders: &Storage<Collider>,
        time: &Time,
        collisions: &Collisions,
    ) {
        wall_collisions.clear();
        for &Collision(e, p) in collisions.fixed.iter() {
            self.moved.entry(p).or_insert_with(Vec::new).push(e);
        }

        for (other, solids) in self.moved.drain() {
            let other_pos = positions.get(other).copied().expect("other_pos");
            let other_col = colliders.get(other).copied().expect("other_col");
            let other_vel =
                velocities.get(other).copied().unwrap_or_default() * time.fixed_seconds();

            let (unique, shared) =
                solids
                    .iter()
                    .copied()
                    .fold((0b0000, 0b1111), |(unique, shared), solid| {
                        let solid_pos = positions.get(solid).copied().expect("solid_pos");
                        let solid_col = *colliders.get(solid).expect("solid_col");
                        let solid_vel = velocities.get(solid).copied().unwrap_or_default()
                            * time.fixed_seconds();

                        let dir = collision_direction(
                            (solid_pos, solid_col, solid_vel),
                            (other_pos, other_col, other_vel),
                        ) as u8;
                        (unique | dir, shared & dir)
                    });

            match CollisionDirection::try_from(shared).expect("shared") {
                CollisionDirection::LeftAbove
                | CollisionDirection::Above
                | CollisionDirection::RightAbove => {
                    resolve_collision(
                        CollisionDirection::Above,
                        other,
                        solids
                            .into_iter()
                            .max_by_key(|&entity| {
                                let entity_pos =
                                    positions.get(entity).copied().expect("entity_pos");
                                let entity_col = *colliders.get(entity).expect("entity_col");
                                OrderedFloat(entity_col.upper_border(entity_pos))
                            })
                            .unwrap(),
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
                    resolve_collision(
                        CollisionDirection::Below,
                        other,
                        solids
                            .into_iter()
                            .min_by_key(|&entity| {
                                let entity_pos =
                                    positions.get(entity).copied().expect("entity_pos");
                                let entity_col = *colliders.get(entity).expect("entity_col");
                                OrderedFloat(entity_col.lower_border(entity_pos))
                            })
                            .unwrap(),
                        &mut positions,
                        &mut wall_collisions,
                        &mut grounded,
                        &mut velocities,
                        &colliders,
                    );
                }
                CollisionDirection::Right => {
                    resolve_collision(
                        CollisionDirection::Right,
                        other,
                        solids
                            .into_iter()
                            .max_by_key(|&entity| {
                                let entity_pos =
                                    positions.get(entity).copied().expect("entity_pos");
                                let entity_col = *colliders.get(entity).expect("entity_col");
                                OrderedFloat(entity_col.right_border(entity_pos))
                            })
                            .unwrap(),
                        &mut positions,
                        &mut wall_collisions,
                        &mut grounded,
                        &mut velocities,
                        &colliders,
                    );
                }
                CollisionDirection::Left => {
                    resolve_collision(
                        CollisionDirection::Left,
                        other,
                        solids
                            .into_iter()
                            .min_by_key(|&entity| {
                                let entity_pos =
                                    positions.get(entity).copied().expect("entity_pos");
                                let entity_col = *colliders.get(entity).expect("entity_col");
                                OrderedFloat(entity_col.left_border(entity_pos))
                            })
                            .unwrap(),
                        &mut positions,
                        &mut wall_collisions,
                        &mut grounded,
                        &mut velocities,
                        &colliders,
                    );
                }
                CollisionDirection::None => {
                    if let (Ok(vertical), Ok(horizontal)) = (
                        CollisionDirection::try_from(unique & 0b0101),
                        CollisionDirection::try_from(unique & 0b1010),
                    ) {
                        if vertical == CollisionDirection::None
                            || horizontal == CollisionDirection::None
                        {
                            warn!("unit is currently inside of a collider");
                        } else {
                            resolve_collision(
                                vertical,
                                other,
                                if vertical == CollisionDirection::Above {
                                    solids
                                        .iter()
                                        .copied()
                                        .max_by_key(|&solid| {
                                            let solid_pos =
                                                positions.get(solid).copied().expect("solid_pos");
                                            let solid_col =
                                                *colliders.get(solid).expect("solid_col");
                                            let solid_vel =
                                                velocities.get(solid).copied().unwrap_or_default()
                                                    * time.fixed_seconds();

                                            if collision_direction(
                                                (solid_pos, solid_col, solid_vel),
                                                (other_pos, other_col, other_vel),
                                            ) & CollisionDirection::Above
                                                != CollisionDirection::None
                                            {
                                                let entity_pos = positions
                                                    .get(solid)
                                                    .copied()
                                                    .expect("entity_pos");
                                                let entity_col =
                                                    *colliders.get(solid).expect("entity_col");
                                                OrderedFloat(entity_col.upper_border(entity_pos))
                                            } else {
                                                OrderedFloat(std::f32::MIN)
                                            }
                                        })
                                        .unwrap()
                                } else {
                                    solids
                                        .iter()
                                        .copied()
                                        .min_by_key(|&solid| {
                                            let solid_pos =
                                                positions.get(solid).copied().expect("solid_pos");
                                            let solid_col =
                                                *colliders.get(solid).expect("solid_col");
                                            let solid_vel =
                                                velocities.get(solid).copied().unwrap_or_default()
                                                    * time.fixed_seconds();

                                            if collision_direction(
                                                (solid_pos, solid_col, solid_vel),
                                                (other_pos, other_col, other_vel),
                                            ) & CollisionDirection::Below
                                                != CollisionDirection::None
                                            {
                                                let entity_pos = positions
                                                    .get(solid)
                                                    .copied()
                                                    .expect("entity_pos");
                                                let entity_col =
                                                    *colliders.get(solid).expect("entity_col");
                                                OrderedFloat(entity_col.lower_border(entity_pos))
                                            } else {
                                                OrderedFloat(std::f32::MAX)
                                            }
                                        })
                                        .unwrap()
                                },
                                &mut positions,
                                &mut wall_collisions,
                                &mut grounded,
                                &mut velocities,
                                &colliders,
                            );

                            resolve_collision(
                                horizontal,
                                other,
                                if horizontal == CollisionDirection::Right {
                                    solids
                                        .into_iter()
                                        .max_by_key(|&solid| {
                                            let solid_pos =
                                                positions.get(solid).copied().expect("solid_pos");
                                            let solid_col =
                                                colliders.get(solid).copied().expect("solid_col");
                                            let solid_vel =
                                                velocities.get(solid).copied().unwrap_or_default()
                                                    * time.fixed_seconds();

                                            if collision_direction(
                                                (solid_pos, solid_col, solid_vel),
                                                (other_pos, other_col, other_vel),
                                            ) & CollisionDirection::Right
                                                != CollisionDirection::None
                                            {
                                                let entity_pos = positions
                                                    .get(solid)
                                                    .copied()
                                                    .expect("entity_pos");
                                                let entity_col = colliders
                                                    .get(solid)
                                                    .copied()
                                                    .expect("entity_col");
                                                OrderedFloat(entity_col.right_border(entity_pos))
                                            } else {
                                                OrderedFloat(std::f32::MIN)
                                            }
                                        })
                                        .unwrap()
                                } else {
                                    solids
                                        .into_iter()
                                        .min_by_key(|&solid| {
                                            let solid_pos =
                                                positions.get(solid).copied().expect("solid_pos");
                                            let solid_col =
                                                *colliders.get(solid).expect("solid_col");
                                            let solid_vel =
                                                velocities.get(solid).copied().unwrap_or_default()
                                                    * time.fixed_seconds();

                                            if collision_direction(
                                                (solid_pos, solid_col, solid_vel),
                                                (other_pos, other_col, other_vel),
                                            ) & CollisionDirection::Left
                                                != CollisionDirection::None
                                            {
                                                let entity_pos = positions
                                                    .get(solid)
                                                    .copied()
                                                    .expect("entity_pos");
                                                let entity_col =
                                                    *colliders.get(solid).expect("entity_col");
                                                OrderedFloat(entity_col.left_border(entity_pos))
                                            } else {
                                                OrderedFloat(std::f32::MAX)
                                            }
                                        })
                                        .unwrap()
                                },
                                &mut positions,
                                &mut wall_collisions,
                                &mut grounded,
                                &mut velocities,
                                &colliders,
                            );
                        }
                    } else {
                        warn!("unit is getting squashed!");
                    }
                }
            };
        }
    }
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
                ColliderType::Player | ColliderType::PlayerDamage => (),
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
                ColliderType::Player | ColliderType::PlayerDamage => None,
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
                ColliderType::Player | ColliderType::PlayerDamage => None,
            };
        }
        CollisionDirection::None => (),
        err => panic!("resolve collision requires a simple direction: {:?}", err),
    }
}
