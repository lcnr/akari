use crow_ecs::{Entities, Entity, Joinable, Storage};

use crate::{
    data::{Collider, ColliderType, Collision, Collisions, Grounded, Position, Velocity},
    physics,
    time::Time,
};

#[derive(Debug, Default)]
pub struct PhysicsSystem {
    collisions: Collisions,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        PhysicsSystem::default()
    }

    pub fn run(
        &mut self,
        velocities: &Storage<Velocity>,
        colliders: &Storage<Collider>,
        previous_positions: &mut Storage<Position>,
        mut positions: &mut Storage<Position>,
        grounded: &mut Storage<Grounded>,
        time: &Time,
    ) -> &mut Collisions {
        self.collisions.clear();

        previous_positions.clear();
        grounded.clear();

        for (velocity, position, entity) in (&velocities, &mut positions, Entities).join() {
            if velocity.x != 0.0 || velocity.y != 0.0 {
                previous_positions.insert(entity, *position);
                position.x += velocity.x * time.fixed_seconds();
                position.y += velocity.y * time.fixed_seconds();
            }
        }

        // check for collisions between moving entities
        let mut iter = (&positions, &colliders, Entities, &previous_positions).join();
        while let Some((&a_pos, &a_collider, a_entity, _moved)) = iter.next() {
            for (&b_pos, &b_collider, b_entity, _moved) in iter.clone() {
                if physics::is_collision(a_pos, a_collider, b_pos, b_collider) {
                    self.resolve_collisions((a_entity, a_collider.ty), (b_entity, b_collider.ty))
                }
            }
        }

        // collisions with other entities
        for (&a_pos, &a_collider, a_entity, _moved) in
            (&positions, &colliders, Entities, &previous_positions).join()
        {
            for (&b_pos, &b_collider, b_entity, _not_moved) in
                (&positions, &colliders, Entities, !&previous_positions).join()
            {
                if physics::is_collision(a_pos, a_collider, b_pos, b_collider) {
                    self.resolve_collisions((a_entity, a_collider.ty), (b_entity, b_collider.ty))
                }
            }
        }

        &mut self.collisions
    }

    fn resolve_collisions(&mut self, a: (Entity, ColliderType), b: (Entity, ColliderType)) {
        match (a, b) {
            ((e, ColliderType::Environment), (p, ColliderType::Player))
            | ((p, ColliderType::Player), (e, ColliderType::Environment)) => {
                self.collisions.fixed.push(Collision(e, p))
            }
            ((b, ColliderType::Bridge), (p, ColliderType::Player))
            | ((p, ColliderType::Player), (b, ColliderType::Bridge)) => {
                self.collisions.bridge.push(Collision(b, p))
            }
            ((p, ColliderType::Player), (d, ColliderType::PlayerDamage))
            | ((d, ColliderType::PlayerDamage), (p, ColliderType::Player)) => {
                self.collisions.player_damage.push(Collision(p, d))
            }
            ((_, ColliderType::Environment), (_, ColliderType::Environment))
            | ((_, ColliderType::Environment), (_, ColliderType::Bridge))
            | ((_, ColliderType::Bridge), (_, ColliderType::Environment))
            | ((_, ColliderType::Environment), (_, ColliderType::PlayerDamage))
            | ((_, ColliderType::PlayerDamage), (_, ColliderType::Environment))
            | ((_, ColliderType::Player), (_, ColliderType::Player))
            | ((_, ColliderType::Bridge), (_, ColliderType::Bridge))
            | ((_, ColliderType::Bridge), (_, ColliderType::PlayerDamage))
            | ((_, ColliderType::PlayerDamage), (_, ColliderType::Bridge))
            | ((_, ColliderType::PlayerDamage), (_, ColliderType::PlayerDamage)) => {}
        }
    }
}
