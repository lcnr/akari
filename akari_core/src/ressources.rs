use crow_anim::AnimationStorage;

use crate::{config::GameConfig, data::Components, input::InputState, time::Time};

pub struct Ressources {
    pub input_state: InputState,
    pub time: Time,
    pub config: GameConfig,
    pub pressed_space: Option<JumpBuffer>,
    pub animation_storage: AnimationStorage,
    pub lazy_update: Vec<Box<dyn FnOnce(&mut Components, &mut Ressources)>>,
}

impl Ressources {
    pub fn new(fps: u32, config: GameConfig) -> Self {
        Ressources {
            input_state: InputState::new(),
            time: Time::new(fps),
            config,
            pressed_space: None,
            animation_storage: AnimationStorage::new(),
            lazy_update: Vec::new(),
        }
    }
}

pub struct JumpBuffer(pub u8);
