use serde::{Deserialize, Serialize};

use crow_anim::AnimationStorage;

use crate::{data::Components, input::InputState, time::Time};

pub struct Ressources {
    pub input_state: InputState,
    pub time: Time,
    pub config: Config,
    pub pressed_space: Option<JumpBuffer>,
    pub animation_storage: AnimationStorage,
    pub lazy_update: Vec<Box<dyn FnOnce(&mut Components, &mut Ressources)>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub gravity: GravityConfig,
    pub input_buffer: InputBufferConfig,
    pub player: PlayerConfig,
}

impl Ressources {
    pub fn new(fps: u32) -> Self {
        Ressources {
            input_state: InputState::new(),
            time: Time::new(fps),
            config: Default::default(),
            pressed_space: None,
            animation_storage: AnimationStorage::new(),
            lazy_update: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub jump_speed: f32,
    pub movement_speed: f32,
    pub grounded_acceleration: f32,
    pub airborne_acceleration: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig {
            jump_speed: 280.0,
            movement_speed: 100.0,
            grounded_acceleration: 850.0,
            airborne_acceleration: 250.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravityConfig {
    pub acceleration: f32,
    pub terminal_velocity: f32,
}

impl Default for GravityConfig {
    fn default() -> Self {
        GravityConfig {
            acceleration: -480.0,
            terminal_velocity: -180.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputBufferConfig {
    pub jump_buffer_frames: u8,
}

impl Default for InputBufferConfig {
    fn default() -> Self {
        InputBufferConfig {
            jump_buffer_frames: 3,
        }
    }
}

pub struct JumpBuffer(pub u8);
