use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};

use crow::{Context, LoadTextureError};

use crow_ecs::Entity;

use crate::{
    data::{Collider, ColliderType, Components, Depth, Position},
    spritesheet::SpriteSheet,
    LoadError, ARENA_HEIGHT, ARENA_WIDTH,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub spritesheet: String,
    pub tiles: [[Option<Tile>; ARENA_WIDTH]; ARENA_HEIGHT],
}

impl EnvironmentConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, LoadError> {
        let f = File::open(path)?;
        Ok(ron::de::from_reader(f)?)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Tile {
    Solid,
    Bridge,
    Grass,
    Spike,
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

#[derive(Debug, Clone)]
pub struct Environment {
    pub sheet: SpriteSheet,
    pub tiles: [[Option<(Tile, Entity)>; ARENA_WIDTH]; ARENA_HEIGHT],
}

pub struct EnvironmentBuilder<'a, 'b> {
    env: &'a mut Environment,
    components: &'b mut Components,
}

impl EnvironmentBuilder<'_, '_> {
    pub fn add_tile(&mut self, (x, y): (usize, usize), tile: Tile) {
        let c = &mut self.components;
        let entity = c.new_entity();

        c.positions.insert(
            entity,
            Position {
                x: x as f32 * 20.0,
                y: y as f32 * 20.0,
            },
        );

        c.depths.insert(entity, tile.depth());

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

        if let Some((_, e)) = self.env.tiles[y][x].replace((tile, entity)) {
            c.delete_entity(e);
        }
    }

    pub fn remove_tile(&mut self, (x, y): (usize, usize)) {
        if let Some((_, e)) = self.env.tiles[y][x].take() {
            self.components.delete_entity(e);
        }
    }
}

impl Drop for EnvironmentBuilder<'_, '_> {
    fn drop(&mut self) {
        for (y, row) in self.env.tiles.iter().enumerate() {
            for (x, opt) in row.iter().copied().enumerate() {
                if let Some((tile, entity)) = opt {
                    self.components.sprites.insert(
                        entity,
                        self.env.sheet.get(match tile {
                            Tile::Bridge => self.env.get_bridge_sprite_number(x, y),
                            Tile::Solid => self.env.get_solid_sprite_number(x, y),
                            Tile::Grass => self.env.get_grass_sprite_number(x, y),
                            Tile::Spike => self.env.get_spike_sprite_number(x, y),
                        }),
                    );
                }
            }
        }
    }
}

impl Environment {
    pub fn new<P: AsRef<Path>>(ctx: &mut Context, path: P) -> Result<Self, LoadTextureError> {
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

        Ok(Environment {
            sheet: builder.finish(),
            tiles: Default::default(),
        })
    }

    pub fn modify<'a, 'b>(
        &'a mut self,
        components: &'b mut Components,
    ) -> EnvironmentBuilder<'a, 'b> {
        EnvironmentBuilder {
            env: self,
            components,
        }
    }

    fn get_spike_sprite_number(&self, x: usize, y: usize) -> usize {
        // TODO: fix grass generation to actually make some kind of sense
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

    fn is_solid(&self, x: usize, y: usize) -> bool {
        if x < ARENA_WIDTH && y < ARENA_HEIGHT {
            if let Some((tile, _)) = self.tiles[y][x] {
                tile == Tile::Solid
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn load(
        ctx: &mut Context,
        components: &mut Components,
        config: &EnvironmentConfig,
    ) -> Result<Self, LoadTextureError> {
        let mut env = Environment::new(ctx, &config.spritesheet)?;
        {
            let mut builder = env.modify(components);

            for (y, line) in config.tiles.iter().enumerate() {
                for (x, tile) in line.iter().copied().enumerate() {
                    if let Some(t) = tile {
                        builder.add_tile((x, y), t);
                    }
                }
            }
        }

        Ok(env)
    }
}
