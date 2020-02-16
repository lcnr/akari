use crow_anim::AnimationStorage;

use crate::{
    config::GameConfig,
    data::{Components, Position},
    environment::{World, WorldData},
    input::InputState,
    time::Time,
};

pub type LazyUpdate = Vec<Box<dyn FnOnce(&mut Components, &mut Ressources)>>;

pub struct Ressources {
    pub input_state: InputState,
    pub time: Time,
    pub config: GameConfig,
    pub pressed_space: Option<JumpBuffer>,
    pub camera: Camera,
    pub animation_storage: AnimationStorage,
    pub world: World,
    pub lazy_update: LazyUpdate,
}

impl Ressources {
    pub fn new(config: GameConfig, world_data: WorldData) -> Self {
        Ressources {
            input_state: InputState::new(),
            time: Time::new(config.fps),
            config,
            pressed_space: None,
            camera: Camera::default(),
            animation_storage: AnimationStorage::new(),
            world: World::new(world_data),
            lazy_update: Vec::new(),
        }
    }
}

pub struct JumpBuffer(pub u8);

#[derive(Default, Debug, Clone)]
pub struct Camera {
    pub position: Position,
}
