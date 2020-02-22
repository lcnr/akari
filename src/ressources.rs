use crow::Context;

use crow_anim::AnimationStorage;

use crate::{
    config::GameConfig,
    data::Components,
    environment::{World, WorldData},
    input::InputState,
    save::SaveData,
    systems::Systems,
    time::Time,
};

pub struct Ressources {
    pub input_state: InputState,
    pub time: Time,
    pub config: GameConfig,
    pub pressed_space: Option<JumpBuffer>,
    pub animation_storage: AnimationStorage,
    pub world: World,
    pub fadeout: Option<Fadeout>,
    pub delayed_actions: Vec<DelayedAction>,
    pub last_save: SaveData,
    pub debug_draw: bool,
}

impl Ressources {
    pub fn new(config: GameConfig, world_data: WorldData, last_save: SaveData) -> Self {
        Ressources {
            input_state: InputState::new(),
            time: Time::new(config.fps),
            config,
            pressed_space: None,
            animation_storage: AnimationStorage::new(),
            world: World::new(world_data),
            fadeout: None,
            delayed_actions: Vec::new(),
            last_save,
            debug_draw: false,
        }
    }

    pub fn reset(&mut self) {
        self.fadeout = None;
        self.delayed_actions.clear();
        self.world.reset();
    }
}

pub struct JumpBuffer(pub u8);

#[derive(Default, Debug, Clone)]
pub struct Fadeout {
    pub current: f32,
    pub frames_left: usize,
}

pub type Action = dyn FnOnce(
    &mut Context,
    &mut Systems,
    &mut Components,
    &mut Ressources,
) -> Result<(), crow::Error>;

pub struct DelayedAction {
    pub frames_left: usize,
    pub action: Box<Action>,
}
