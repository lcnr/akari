use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crow::Context;

use crow_ecs::Joinable;

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

    pub fn reset(&mut self) {
        self.chunks.clear();
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
        #[cfg(feature = "profiler")]
        profile_scope!("run");

        let (x, y) = match (&c.player_state, &c.positions).join().unique() {
            Ok((_state, position)) => {
                let x = (position.x.round() / CHUNK_WIDTH as f32) as i32;
                let y = (position.y.round() / CHUNK_HEIGHT as f32) as i32;
                (x, y)
            }
            Err(err) => {
                error!("No unique player: {:?}", err);
                return Ok(());
            }
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

        for i in (0..r.world.chunks.len()).rev() {
            let chunk = &mut r.world.chunks[i];
            if !chunks.contains(&chunk.position) {
                r.world.chunks.swap_remove(i).clear(c);
            }
        }

        for &chunk in chunks.iter() {
            if !r.world.chunks.iter().any(|c| c.position == chunk) {
                info!("Loading chunk: {:?}", chunk);
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
        #[cfg(feature = "profiler")]
        profile_scope!("load_chunk");

        if let Some(path) = r.world.data.chunks.get(&position) {
            let config = ChunkData::load(path).unwrap();

            let chunk = Chunk::new(ctx, position, config, c)?;
            r.world.chunks.push(chunk);
        } else {
            let chunk = Chunk::empty(position, c);
            r.world.chunks.push(chunk);
        }

        Ok(())
    }
}
