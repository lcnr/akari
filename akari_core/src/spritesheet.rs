use std::path::Path;

use crow::{Context, LoadTextureError, Texture};

use crow_anim::Sprite;

use crate::config::SpriteSheetConfig;

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
