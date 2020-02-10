use std::path::Path;

use serde::{Deserialize, Serialize};

use crow::{Context, LoadTextureError, Texture};

use crow_anim::Sprite;

use crate::LoadError;

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

impl SpriteSheetConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, LoadError> {
        let f = std::fs::File::open(path)?;
        Ok(ron::de::from_reader(f)?)
    }
}

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub sprites: Vec<Sprite>,
}

impl SpriteSheet {
    pub fn from_config(
        ctx: &mut Context,
        config: &SpriteSheetConfig,
    ) -> Result<Self, LoadTextureError> {
        let mut builder = Self::build(ctx, &config.image_path)?;

        for sprite in &config.sprites {
            builder.add_sprite(sprite.position, sprite.size, sprite.offset);
        }

        Ok(builder.finish())
    }

    pub fn build<P: AsRef<Path>>(
        ctx: &mut Context,
        path: P,
    ) -> Result<SpriteSheetBuilder, LoadTextureError> {
        SpriteSheetBuilder::new(ctx, path)
    }

    pub fn count(&self) -> usize {
        self.sprites.len()
    }

    pub fn get(&self, idx: usize) -> Sprite {
        self.sprites[idx].clone()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Sprite> + 'a {
        self.sprites.iter().cloned()
    }
}

pub struct SpriteSheetBuilder {
    pub texture: Texture,
    sprites: Vec<Sprite>,
}

impl SpriteSheetBuilder {
    pub fn new<P: AsRef<Path>>(ctx: &mut Context, path: P) -> Result<Self, LoadTextureError> {
        Texture::load(ctx, path).map(SpriteSheetBuilder::from_texture)
    }

    pub fn from_texture(texture: Texture) -> Self {
        SpriteSheetBuilder {
            texture,
            sprites: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, position: (u32, u32), size: (u32, u32), offset: (i32, i32)) {
        self.sprites.push(Sprite {
            texture: self.texture.get_section(position, size),
            offset,
        })
    }

    pub fn finish(self) -> SpriteSheet {
        SpriteSheet {
            sprites: self.sprites,
        }
    }
}
