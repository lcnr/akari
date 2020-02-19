use crow_ecs::{Joinable, SparseStorage, Storage};

use crate::{
    config::CameraConfig,
    data::{Camera, PlayerState, Position, Velocity},
    time::Time,
};

#[derive(Debug)]
pub struct CameraSystem;

impl CameraSystem {
    pub fn run(
        &mut self,
        player_states: &SparseStorage<PlayerState>,
        positions: &Storage<Position>,
        previous_positions: &Storage<Position>,
        velocities: &mut Storage<Velocity>,
        cameras: &SparseStorage<Camera>,
        time: &Time,
        config: &CameraConfig,
    ) {
        match (player_states, previous_positions.maybe(), positions)
            .join()
            .unique()
        {
            Ok((_player, previous_position, position)) => {
                let target = previous_position.copied().unwrap_or(*position);
                for (&Camera, camera_position, velocity) in (cameras, positions, velocities).join()
                {
                    let (diff_x, diff_y) = (
                        target.x - camera_position.x - config.offset.0,
                        target.y - camera_position.y - config.offset.1,
                    );

                    *velocity = Velocity {
                        x: diff_x / time.fixed_seconds(),
                        y: diff_y / time.fixed_seconds(),
                    };
                }
            }
            Err(err) => error!("No unique player: {:?}", err),
        }
    }
}
