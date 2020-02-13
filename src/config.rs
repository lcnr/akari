use std::{fs::File, io, iter, path::Path};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crow::{Context, LoadTextureError};

use crow_anim::{Animation, AnimationHandle, AnimationStorage};

use crate::{
    data::PlayerAnimations, environment::Tile, spritesheet::SpriteSheet, ARENA_HEIGHT, ARENA_WIDTH,
};

#[derive(Debug)]
pub enum LoadError {
    IoError(io::Error),
    DeserializeError(ron::de::Error),
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        LoadError::IoError(err)
    }
}

impl From<ron::de::Error> for LoadError {
    fn from(err: ron::de::Error) -> Self {
        LoadError::DeserializeError(err)
    }
}

pub trait Config: Sized {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, LoadError>;
}

impl<'a, T: DeserializeOwned> Config for T {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, LoadError> {
        let f = File::open(path)?;
        Ok(ron::de::from_reader(f)?)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub spritesheet: String,
    pub tiles: [[Option<Tile>; ARENA_WIDTH]; ARENA_HEIGHT],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteConfig {
    pub position: (u32, u32),
    pub size: (u32, u32),
    pub offset: (i32, i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteSheetConfig {
    pub image_path: String,
    pub sprites: Vec<SpriteConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameConfig {
    pub spritesheet: usize,
    pub sprite: usize,
    /// The duration this image is shown in 60th of a second
    pub duration: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerAnimationsConfig {
    pub spritesheets: Vec<String>,
    pub idle: Vec<FrameConfig>,
    pub running: Vec<FrameConfig>,
    pub jumping: Vec<FrameConfig>,
    pub start_falling: Vec<FrameConfig>,
    pub falling: Vec<FrameConfig>,
}

fn add_animation(
    storage: &mut AnimationStorage,
    spritesheets: &[SpriteSheet],
    config: Vec<FrameConfig>,
    next: Option<AnimationHandle>,
) -> AnimationHandle {
    let mut anim = Animation::empty();
    for FrameConfig {
        spritesheet,
        sprite,
        duration,
    } in config
    {
        anim.frames
            .extend(iter::repeat(spritesheets[spritesheet].get(sprite)).take(duration));
    }
    let handle = storage.insert(anim);

    if let Some(next) = next {
        storage.get_mut(handle).next = next;
    } else {
        storage.get_mut(handle).next = handle;
    }

    handle
}

impl PlayerAnimations {
    pub fn from_config(
        ctx: &mut Context,
        storage: &mut AnimationStorage,
        config: PlayerAnimationsConfig,
    ) -> Result<Self, LoadTextureError> {
        let sheets = config
            .spritesheets
            .into_iter()
            .map(|path| SpriteSheet::from_config(ctx, &SpriteSheetConfig::load(path).unwrap()))
            .collect::<Result<Vec<_>, _>>()?;

        let idle = add_animation(storage, &sheets, config.idle, None);
        let running = add_animation(storage, &sheets, config.running, None);
        let falling = add_animation(storage, &sheets, config.falling, None);
        let start_falling = add_animation(storage, &sheets, config.start_falling, Some(falling));
        let jumping = add_animation(storage, &sheets, config.jumping, Some(start_falling));

        Ok(PlayerAnimations {
            idle,
            running,
            jumping,
            start_falling,
            falling,
        })
    }
}
