use std::{fs::File, io, path::Path, iter};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crow::{Context, LoadTextureError};

use crow_anim::{Animation, AnimationStorage};

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
    pub jumping: Vec<FrameConfig>,
    pub start_falling: Vec<FrameConfig>,
    pub falling: Vec<FrameConfig>,
}

impl PlayerAnimations {
    pub fn from_config(
        ctx: &mut Context,
        animation_storage: &mut AnimationStorage,
        config: PlayerAnimationsConfig,
    ) -> Result<Self, LoadTextureError> {
        let spritesheets = config.spritesheets.into_iter().map(|path| SpriteSheet::from_config(
            ctx,
            &SpriteSheetConfig::load(path).unwrap(),
        )).collect::<Result<Vec<_>, _>>()?;

        let mut idle_animation = Animation::empty();
        for FrameConfig { spritesheet, sprite, duration } in config.idle {
            idle_animation.frames.extend(iter::repeat(spritesheets[spritesheet].get(sprite)).take(duration));
        }
        let idle = animation_storage.insert(idle_animation);
        animation_storage.get_mut(idle).next = idle;

        let mut falling_animation = Animation::empty();
        for FrameConfig { spritesheet, sprite, duration } in config.falling {
            falling_animation.frames.extend(iter::repeat(spritesheets[spritesheet].get(sprite)).take(duration));
        }
        let falling = animation_storage.insert(falling_animation);
        animation_storage.get_mut(falling).next = falling;

        let mut jumping_animation = Animation::empty();
        for FrameConfig { spritesheet, sprite, duration } in config.jumping {
            jumping_animation.frames.extend(iter::repeat(spritesheets[spritesheet].get(sprite)).take(duration));
        }
        let jumping = animation_storage.insert(jumping_animation);
        animation_storage.get_mut(jumping).next = falling;

        let mut start_falling_animation = Animation::empty();
        for FrameConfig { spritesheet, sprite, duration } in config.start_falling {
            start_falling_animation.frames.extend(iter::repeat(spritesheets[spritesheet].get(sprite)).take(duration));
        }
        let start_falling = animation_storage.insert(start_falling_animation);
        animation_storage.get_mut(start_falling).next = falling;

        Ok(PlayerAnimations {
            idle,
            jumping,
            start_falling,
            falling,
        })
    }
}
