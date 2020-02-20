use std::{fs::File, io, iter, path::Path};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use ron::ser::PrettyConfig;

use crow::{Context, LoadTextureError};

use crow_anim::{Animation, AnimationHandle, AnimationStorage};

use crate::{data::PlayerAnimations, input::Key, spritesheet::SpriteSheet};

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

    fn example() -> String
    where
        Self: Default;
}

impl<'a, T: DeserializeOwned + Serialize> Config for T {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, LoadError> {
        let f = File::open(path)?;
        Ok(ron::de::from_reader(f)?)
    }

    fn example() -> String
    where
        Self: Default,
    {
        let obj = Self::default();
        ron::ser::to_string_pretty(&obj, PrettyConfig::default()).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub fps: u32,
    pub window: WindowConfig,
    pub input: InputConfig,
    pub camera: CameraConfig,
    pub gravity: GravityConfig,
    pub input_buffer: InputBufferConfig,
    pub player: PlayerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowConfig {
    pub size: (u32, u32),
    pub scale: u32,
    pub title: String,
    pub icon_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputConfig {
    pub down: Key,
    pub right: Key,
    pub left: Key,
    pub jump: Key,
    /// TODO: remove me before releasing this game
    pub debug_toggle: Key,
    /// TODO: remove me
    pub editor_tile: Key,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraConfig {
    pub offset: (f32, f32),
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
    pub run_into_obstacle: Vec<FrameConfig>,
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
        let run_into_obstacle =
            add_animation(storage, &sheets, config.run_into_obstacle, Some(idle));
        let falling = add_animation(storage, &sheets, config.falling, None);
        let start_falling = add_animation(storage, &sheets, config.start_falling, Some(falling));
        let jumping = add_animation(storage, &sheets, config.jumping, Some(start_falling));

        Ok(PlayerAnimations {
            idle,
            running,
            run_into_obstacle,
            jumping,
            start_falling,
            falling,
        })
    }
}
