use std::path::Path;

use serde::{Deserialize, Serialize};

use crow::{Context, LoadTextureError};

use crow_ecs::Entity;

use crate::{
    data::{Collider, ColliderType, Components, Position},
    environment::{Tile, CHUNK_HEIGHT, CHUNK_TILES, CHUNK_WIDTH, TILE_SIZE},
    spritesheet::SpriteSheet,
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ChunkData {
    pub spritesheet: String,
    pub tiles: [[Option<Tile>; CHUNK_TILES]; CHUNK_TILES],
}

impl ChunkData {
    const fn width(&self) -> usize {
        CHUNK_TILES
    }

    const fn height(&self) -> usize {
        CHUNK_TILES
    }

    fn tile(&self, (x, y): (usize, usize)) -> Option<Tile> {
        self.tiles
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .flatten()
    }

    fn is_solid(&self, x: usize, y: usize) -> bool {
        if x < self.width() && y < self.height() {
            if let Some(tile) = self.tile((x, y)) {
                tile == Tile::Solid
            } else {
                false
            }
        } else {
            true
        }
    }

    fn get_spike_sprite_number(&self, x: usize, y: usize) -> usize {
        // TODO: fix spike generation to actually make some kind of sense
        match (x * x * 5).wrapping_sub(y % 11 + 3) % 2 {
            0 => 56,
            1 => 57,
            _ => unreachable!(),
        }
    }

    fn get_grass_sprite_number(&self, x: usize, y: usize) -> usize {
        // TODO: fix grass generation to actually make some kind of sense
        match (x * x * 2).wrapping_sub(y % 7 + 3) % 6 {
            0 => 16,
            1 => 17,
            2 => 18,
            3 => 20,
            4 => 21,
            5 => 22,
            _ => unreachable!(),
        }
    }

    fn get_bridge_sprite_number(&self, x: usize, y: usize) -> usize {
        // 0 x 1
        let tiles = (self.is_solid(x.wrapping_sub(1), y), self.is_solid(x + 1, y));

        match tiles {
            (true, _) => 24,
            (false, false) => 25,
            (false, true) => 26,
        }
    }

    fn get_solid_sprite_number(&self, x: usize, y: usize) -> usize {
        // 6 5 4
        // 7 x 3
        // 0 1 2
        let tiles = (
            self.is_solid(x.wrapping_sub(1), y.wrapping_sub(1)),
            self.is_solid(x, y.wrapping_sub(1)),
            self.is_solid(x + 1, y.wrapping_sub(1)),
            self.is_solid(x + 1, y),
            self.is_solid(x + 1, y + 1),
            self.is_solid(x, y + 1),
            self.is_solid(x.wrapping_sub(1), y + 1),
            self.is_solid(x.wrapping_sub(1), y),
        );

        match tiles {
            (_, true, true, true, true, true, _, false) => 0,
            (true, true, true, true, _, false, _, true) => 1,
            (true, true, _, false, _, true, true, true) => 2,
            (_, false, _, true, true, true, true, true) => 3,
            (_, true, true, true, _, false, _, false) => 4,
            (true, true, _, false, _, false, _, true) => 5,
            (_, false, _, false, _, true, true, true) => 6,
            (_, false, _, true, true, true, _, false) => 7,
            (true, true, true, true, true, true, false, true) => 8,
            (true, true, true, true, false, true, true, true) => 9,
            (true, true, false, true, true, true, true, true) => 10,
            (false, true, true, true, true, true, true, true) => 11,
            (_, false, _, true, _, false, _, false) => 12,
            (_, false, _, true, _, false, _, true) => 13,
            (_, false, _, false, _, false, _, true) => 14,
            (_, true, _, false, _, false, _, false) => 15,
            (_, true, _, false, _, true, _, false) => 19,
            (_, false, _, false, _, true, _, false) => 23,
            (_, false, _, false, _, false, _, false) => 27,
            (false, true, true, true, _, false, _, true) => 28,
            (true, true, false, true, _, false, _, true) => 29,
            (false, true, false, true, _, false, _, true) => 30,
            (true, true, _, false, _, true, false, true) => 31,
            (_, true, false, true, false, true, _, false) => 32,
            (_, true, false, true, _, false, _, false) => 33,
            (false, true, _, false, _, false, _, true) => 34,
            (false, true, _, false, _, true, true, true) => 35,
            (_, true, true, true, false, true, _, false) => 36,
            (_, false, _, false, _, true, false, true) => 37,
            (_, false, _, true, false, true, _, false) => 38,
            (false, true, _, false, _, true, false, true) => 39,
            (_, true, false, true, true, true, _, false) => 40,
            (_, false, _, true, false, true, false, true) => 41,
            (_, false, _, true, true, true, false, true) => 42,
            (_, false, _, true, false, true, true, true) => 43,
            (true, true, true, true, false, true, false, true) => 44,
            (true, true, false, true, true, true, false, true) => 45,
            (false, true, true, true, true, true, false, true) => 46,
            (false, true, false, true, false, true, false, true) => 47,
            (true, true, false, true, false, true, true, true) => 48,
            (false, true, true, true, false, true, true, true) => 49,
            (false, true, false, true, true, true, true, true) => 50,
            (true, true, true, true, true, true, true, true) => 51,
            (true, true, false, true, false, true, false, true) => 52,
            (false, true, false, true, false, true, true, true) => 53,
            (false, true, false, true, true, true, false, true) => 54,
            (false, true, true, true, false, true, false, true) => 55,
        }
    }
}

pub struct Chunk {
    pub position: (i32, i32),
    pub tiles: Vec<Entity>,
}

impl Drop for Chunk {
    fn drop(&mut self) {
        if !self.tiles.is_empty() {
            warn!("Dropped chunk without calling `Chunk::clear` first");
        }
    }
}

impl Chunk {
    pub fn new(
        ctx: &mut Context,
        position: (i32, i32),
        config: ChunkData,
        c: &mut Components,
    ) -> Result<Self, crow::Error> {
        let spritesheet = Self::build_spritesheet(ctx, &config.spritesheet).unwrap();

        let mut chunk = Chunk {
            position,
            tiles: Vec::new(),
        };

        for (y, line) in config.tiles.iter().enumerate() {
            for x in 0..line.len() {
                chunk.add_tile((x, y), &config, c, &spritesheet);
            }
        }

        Ok(chunk)
    }

    pub fn build_spritesheet<P: AsRef<Path>>(
        ctx: &mut Context,
        path: P,
    ) -> Result<SpriteSheet, LoadTextureError> {
        let mut builder = SpriteSheet::build(ctx, path)?;

        let mut y = builder.texture.height();
        let mut x = 0;
        for _ in 0..58 {
            if x == 0 {
                y -= 20;
            }

            builder.add_sprite((x, y), (20, 20), (0, 0));

            x += 20;
            if x >= builder.texture.width() {
                x = 0;
            }
        }

        Ok(builder.finish())
    }

    pub fn add_tile(
        &mut self,
        (x, y): (usize, usize),
        config: &ChunkData,
        c: &mut Components,
        sheet: &SpriteSheet,
    ) {
        let (chunk_x, chunk_y) = self.position;

        if let Some(tile) = config.tiles[y][x] {
            let entity = c.new_entity();

            self.tiles.push(entity);

            c.positions.insert(
                entity,
                Position {
                    x: (chunk_x * CHUNK_WIDTH as i32) as f32 + (x * TILE_SIZE) as f32,
                    y: (chunk_y * CHUNK_HEIGHT as i32) as f32 + (y * TILE_SIZE) as f32,
                },
            );

            match tile {
                Tile::Solid => {
                    c.colliders.insert(
                        entity,
                        Collider {
                            w: 20.0,
                            h: 20.0,
                            ty: ColliderType::Environment,
                        },
                    );
                }
                Tile::Bridge => {
                    c.colliders.insert(
                        entity,
                        Collider {
                            w: 20.0,
                            h: 20.0,
                            ty: ColliderType::Bridge,
                        },
                    );
                }
                Tile::Spike => {
                    c.colliders.insert(
                        entity,
                        Collider {
                            w: 20.0,
                            h: 10.0,
                            ty: ColliderType::PlayerDamage,
                        },
                    );
                }
                Tile::Grass => (),
            }

            c.depths.insert(entity, tile.depth());

            c.sprites.insert(
                entity,
                sheet.get(match tile {
                    Tile::Bridge => config.get_bridge_sprite_number(x, y),
                    Tile::Solid => config.get_solid_sprite_number(x, y),
                    Tile::Grass => config.get_grass_sprite_number(x, y),
                    Tile::Spike => config.get_spike_sprite_number(x, y),
                }),
            );
        }
    }
}