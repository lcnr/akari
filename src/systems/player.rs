use crow_ecs::{Entities, Entity, Joinable, SparseStorage};

use crate::{
    data::{Collision, Collisions, Components, Grounded, IgnoreBridges, PlayerState, Velocity},
    input::ButtonState,
    ressources::{JumpBuffer, Ressources},
};

#[derive(Debug)]
pub struct PlayerStateMachine;

impl PlayerStateMachine {
    pub fn run(&mut self, c: &mut Components, r: &mut Ressources, collisions: &Collisions) {
        #[cfg(feature = "profiler")]
        profile_scope!("run");

        for (state, velocity, grounded, entity) in (
            &mut c.player_state,
            &mut c.velocities,
            (&c.grounded).maybe(),
            Entities,
        )
            .join()
        {
            if let Some(new_state) = match *state {
                PlayerState::Idle
                | PlayerState::Walking
                | PlayerState::Jumping
                | PlayerState::Falling => on_player_damage(entity, &collisions.player_damage),
                PlayerState::Dying | PlayerState::Dead => None,
            } {
                initialize_state(new_state, entity, velocity, &mut c.ignore_bridges, r);
                *state = new_state;
            }

            match *state {
                PlayerState::Idle
                | PlayerState::Walking
                | PlayerState::Jumping
                | PlayerState::Falling => {
                    let direction = match (r.input_state.left, r.input_state.right) {
                        (ButtonState::Down, ButtonState::Down)
                        | (ButtonState::Up, ButtonState::Up) => 0.0,
                        (ButtonState::Down, ButtonState::Up) => -1.0,
                        (ButtonState::Up, ButtonState::Down) => 1.0,
                    };

                    let acceleration = if state.is_grounded() {
                        r.config.player.grounded_acceleration
                    } else {
                        r.config.player.airborne_acceleration
                    } * r.time.fixed_seconds();

                    let target_speed = r.config.player.movement_speed * direction;
                    let speed_difference = target_speed - velocity.x;

                    velocity.x += if speed_difference.abs() > acceleration {
                        acceleration.copysign(speed_difference)
                    } else {
                        speed_difference
                    };
                }
                PlayerState::Dying | PlayerState::Dead => (),
            }

            if let Some(new_state) = match *state {
                PlayerState::Idle | PlayerState::Walking => {
                    if grounded == None {
                        Some(PlayerState::Falling)
                    } else {
                        None
                    }
                }
                PlayerState::Jumping | PlayerState::Falling => {
                    if grounded == Some(&Grounded) {
                        Some(PlayerState::Idle)
                    } else {
                        None
                    }
                }
                PlayerState::Dying | PlayerState::Dead => None,
            } {
                initialize_state(new_state, entity, velocity, &mut c.ignore_bridges, r);
                *state = new_state;
            }

            if let Some(new_state) = match *state {
                PlayerState::Idle | PlayerState::Walking => maybe_jump(&mut r.pressed_space),
                PlayerState::Jumping
                | PlayerState::Falling
                | PlayerState::Dying
                | PlayerState::Dead => None,
            } {
                initialize_state(new_state, entity, velocity, &mut c.ignore_bridges, r);
                *state = new_state;
            }

            match state {
                PlayerState::Idle
                | PlayerState::Walking
                | PlayerState::Jumping
                | PlayerState::Falling => {
                    if r.input_state.down == ButtonState::Down {
                        c.ignore_bridges.insert(entity, IgnoreBridges);
                    } else {
                        c.ignore_bridges.remove(entity);
                    }
                }
                PlayerState::Dying | PlayerState::Dead => (),
            }
        }
    }
}

fn initialize_state(
    state: PlayerState,
    player: Entity,
    velocity: &mut Velocity,
    ignore_bridges: &mut SparseStorage<IgnoreBridges>,
    r: &mut Ressources,
) {
    println!("Updating state to {:?}", state);
    match state {
        PlayerState::Jumping => {
            velocity.y = r.config.player.jump_speed;
        }
        PlayerState::Dying => {
            // prevent the player from sliding of falling through bridges
            // while still falling to the ground
            velocity.y = velocity.y.min(0.0);
            velocity.x = 0.0;
            ignore_bridges.remove(player);
        }
        _ => (),
    }
}

fn on_player_damage(entity: Entity, player_damage: &[Collision]) -> Option<PlayerState> {
    for &Collision(player, _damage) in player_damage.iter() {
        if player == entity {
            return Some(PlayerState::Dying);
        }
    }

    None
}

fn maybe_jump(pressed_space: &mut Option<JumpBuffer>) -> Option<PlayerState> {
    // use take to prevent double jmp after bonk
    if pressed_space.take().is_some() {
        Some(PlayerState::Jumping)
    } else {
        None
    }
}
