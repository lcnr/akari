use crow_ecs::{SparseStorage, Storage};

use crate::data::{Collider, Collision, Collisions, IgnoreBridges, Position};

#[derive(Debug, Default)]
pub struct BridgeCollisionSystem;
impl BridgeCollisionSystem {
    pub fn run(
        &mut self,
        positions: &Storage<Position>,
        previous_positions: &Storage<Position>,
        colliders: &Storage<Collider>,
        ignore_bridges: &SparseStorage<IgnoreBridges>,
        collisions: &mut Collisions,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("run");

        for Collision(bridge, other) in collisions.bridge.drain(..) {
            if ignore_bridges.get(other).is_none() {
                let bridge_pos = positions.get(bridge).copied().unwrap();
                let bridge_pos = previous_positions
                    .get(bridge)
                    .copied()
                    .unwrap_or(bridge_pos);
                let bridge_col = colliders.get(bridge).expect("bridge collider");

                let other_pos = positions.get(other).copied().unwrap();
                let other_pos = previous_positions.get(other).copied().unwrap_or(other_pos);

                if bridge_col.upper_border(bridge_pos) <= other_pos.y {
                    collisions.fixed.push(Collision(bridge, other));
                }
            }
        }
    }
}
