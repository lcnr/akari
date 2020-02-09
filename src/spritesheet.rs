use std::path::Path;

use crow::{Context, LoadTextureError, Texture};

use crate::data::Sprite;

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub sprites: Vec<Sprite>,
}

impl SpriteSheet {
    pub fn build<P: AsRef<Path>>(
        ctx: &mut Context,
        path: P,
    ) -> Result<SpriteSheetBuilder, LoadTextureError> {
        SpriteSheetBuilder::new(ctx, path)
    }

    pub fn get(&self, idx: usize) -> Option<Sprite> {
        self.sprites.get(idx).cloned()
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
