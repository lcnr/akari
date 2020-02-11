use crow_ecs::{Entities, Entity, Joinable, SparseStorage};

use crow_anim::{AnimationState, AnimationStorage};

use crate::{
    data::{
        Collision, Collisions, Components, Grounded, IgnoreBridges, Mirrored, PlayerAnimations,
        PlayerState, Velocity,
    },
    input::ButtonState,
    ressources::{JumpBuffer, Ressources},
};

#[derive(Debug)]
pub struct PlayerStateMachine;

impl PlayerStateMachine {
    pub fn run(&mut self, c: &mut Components, r: &mut Ressources, collisions: &Collisions) {
        #[cfg(feature = "profiler")]
        profile_scope!("run");

        for (state, animation, player_animations, velocity, grounded, entity) in (
            &mut c.player_state,
            &mut c.animations,
            &c.player_animations,
            &mut c.velocities,
            (&c.grounded).maybe(),
            Entities,
        )
            .join()
        {
            if let Some(new_state) = match *state {
                PlayerState::Grounded | PlayerState::Airborne => {
                    on_player_damage(entity, &collisions.player_damage)
                }
                PlayerState::Dying | PlayerState::Dead => None,
            } {
                initialize_state(
                    new_state,
                    entity,
                    velocity,
                    animation,
                    player_animations,
                    &r.animation_storage,
                    &mut c.ignore_bridges,
                );
                *state = new_state;
            }

            match *state {
                PlayerState::Grounded | PlayerState::Airborne => {
                    let direction = match (r.input_state.left, r.input_state.right) {
                        (ButtonState::Down, ButtonState::Down)
                        | (ButtonState::Up, ButtonState::Up) => 0.0,
                        (ButtonState::Down, ButtonState::Up) => {
                            c.mirrored.insert(entity, Mirrored);
                            -1.0
                        }
                        (ButtonState::Up, ButtonState::Down) => {
                            c.mirrored.remove(entity);
                            1.0
                        }
                    };

                    let acceleration = if state == &mut PlayerState::Grounded {
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
                PlayerState::Grounded => {
                    if grounded == None {
                        Some(PlayerState::Airborne)
                    } else {
                        None
                    }
                }
                PlayerState::Airborne => {
                    if grounded == Some(&Grounded) {
                        Some(PlayerState::Grounded)
                    } else {
                        None
                    }
                }
                PlayerState::Dying | PlayerState::Dead => None,
            } {
                initialize_state(
                    new_state,
                    entity,
                    velocity,
                    animation,
                    player_animations,
                    &r.animation_storage,
                    &mut c.ignore_bridges,
                );
                *state = new_state;
            }

            if match *state {
                PlayerState::Grounded => maybe_jump(&mut r.pressed_space),
                PlayerState::Airborne | PlayerState::Dying | PlayerState::Dead => false,
            } {
                velocity.y = r.config.player.jump_speed;
                *animation = r.animation_storage.start(player_animations.jumping);

                *state = PlayerState::Airborne;
            }

            match state {
                PlayerState::Grounded | PlayerState::Airborne => {
                    if r.input_state.down == ButtonState::Down {
                        c.ignore_bridges.insert(entity, IgnoreBridges);
                    } else {
                        c.ignore_bridges.remove(entity);
                    }
                }
                PlayerState::Dying | PlayerState::Dead => (),
            }

            if *state == PlayerState::Airborne
                && velocity.y.is_sign_negative()
                && animation.current == player_animations.jumping
            {
                *animation = r.animation_storage.start(player_animations.start_falling);
            }
        }
    }
}

fn initialize_state(
    state: PlayerState,
    player: Entity,
    velocity: &mut Velocity,
    animation: &mut AnimationState,
    player_animations: &PlayerAnimations,
    animation_storage: &AnimationStorage,
    ignore_bridges: &mut SparseStorage<IgnoreBridges>,
) {
    match state {
        PlayerState::Grounded => {
            *animation = animation_storage.start(player_animations.idle);
        }
        PlayerState::Airborne => {
            *animation = animation_storage.start(player_animations.start_falling);
            // jumping is handled directly after `maybe_jump`
        }
        PlayerState::Dying => {
            // prevent the player from sliding of falling through bridges
            // while still falling to the ground
            velocity.y = velocity.y.min(0.0);
            velocity.x = 0.0;
            ignore_bridges.remove(player);

            // TODO: dying animation
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

fn maybe_jump(pressed_space: &mut Option<JumpBuffer>) -> bool {
    // use take to prevent double jmp after bonk
    pressed_space.take().is_some()
}
