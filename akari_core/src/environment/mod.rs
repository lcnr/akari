use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crow::Context;

use crate::{
    config::Config,
    data::{Components, Depth},
    ressources::Ressources,
};

pub mod chunk;

use chunk::{Chunk, ChunkData};

pub const CHUNK_TILES: usize = 16;

pub const CHUNK_WIDTH: usize = CHUNK_TILES * TILE_SIZE;
pub const CHUNK_HEIGHT: usize = CHUNK_TILES * TILE_SIZE;

pub const TILE_SIZE: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Tile {
    Solid,
    Grass,
    Spike,
    Bridge,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Solid
    }
}

impl Tile {
    fn depth(self) -> Depth {
        match self {
            Tile::Solid => Depth::Tiles,
            Tile::Bridge => Depth::Bridges,
            Tile::Grass => Depth::Grass,
            Tile::Spike => Depth::Grass,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WorldData {
    pub chunks: HashMap<(i32, i32), String>,
}

pub struct World {
    pub data: WorldData,
    pub chunks: Vec<Chunk>,
}

impl World {
    pub fn new(data: WorldData) -> Self {
        World {
            data,
            chunks: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct EnvironmentSystem;

impl EnvironmentSystem {
    pub fn run(
        &mut self,
        ctx: &mut Context,
        c: &mut Components,
        r: &mut Ressources,
    ) -> Result<(), crow::Error> {
        let (x, y) = {
            let x = (r.camera.position.x.round() / CHUNK_WIDTH as f32) as i32;
            let y = (r.camera.position.y.round() / CHUNK_HEIGHT as f32) as i32;
            (x, y)
        };

        let chunks = [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ];

        r.world.chunks.retain(|c| chunks.contains(&c.position));

        for &chunk in chunks.iter() {
            if !r.world.chunks.iter().any(|c| c.position == (x, y)) {
                self.load_chunk(ctx, chunk, c, r)?;
            }
        }

        Ok(())
    }

    pub fn load_chunk(
        &mut self,
        ctx: &mut Context,
        position: (i32, i32),
        c: &mut Components,
        r: &mut Ressources,
    ) -> Result<(), crow::Error> {
        if let Some(path) = r.world.data.chunks.get(&position) {
            let config = ChunkData::load(path).unwrap();

            let chunk = Chunk::new(ctx, position, config, c)?;
            r.world.chunks.push(chunk);
        }

        Ok(())
    }
}