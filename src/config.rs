use std::{fs::File, io, path::Path};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{environment::Tile, ARENA_HEIGHT, ARENA_WIDTH};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAnimationsConfig {
    spritesheets: Vec<String>,
    idle: Vec<FrameConfig>,
}
