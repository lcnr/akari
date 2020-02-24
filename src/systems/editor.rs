use std::convert::TryInto;

use crow::Context;

use crow_ecs::Joinable;

use crate::{
    data::{Components, Position},
    environment::{chunk::Chunk, Tile, CHUNK_HEIGHT, CHUNK_TILES, CHUNK_WIDTH, TILE_SIZE},
    input::{InputEvent, Key, KeyState, MouseButton},
    ressources::Ressources,
};

#[derive(Debug, Default)]
pub struct EditorSystem {
    tile: Tile,
}

impl EditorSystem {
    pub fn new() -> Self {
        EditorSystem { tile: Tile::Solid }
    }
}

fn tile_on_click(camera: Position, r: &mut Ressources) -> ((i32, i32), (usize, usize)) {
    let pos = r.input_state.cursor_position();

    let camera: (i32, i32) = camera.into();

    let ingame_pos = (pos.0 + camera.0, pos.1 + camera.1);

    let chunk = (
        ingame_pos.0 / CHUNK_WIDTH as i32 - (ingame_pos.0 < 0) as i32,
        ingame_pos.1 / CHUNK_HEIGHT as i32 - (ingame_pos.1 < 0) as i32,
    );

    let mut tile = (
        (ingame_pos.0.abs() % CHUNK_WIDTH as i32) / TILE_SIZE as i32,
        (ingame_pos.1.abs() % CHUNK_WIDTH as i32) / TILE_SIZE as i32,
    );

    if ingame_pos.0 < 0 {
        tile.0 = CHUNK_TILES as i32 - 1 - tile.0;
    }

    if ingame_pos.1 < 0 {
        tile.1 = CHUNK_TILES as i32 - 1 - tile.1;
    }

    (
        chunk,
        (tile.0.try_into().unwrap(), tile.1.try_into().unwrap()),
    )
}

impl EditorSystem {
    pub fn run(
        &mut self,
        ctx: &mut Context,
        c: &mut Components,
        r: &mut Ressources,
    ) -> Result<(), crow::Error> {
        let (camera, _) = (&mut c.positions, &c.cameras)
            .join()
            .unique()
            .expect("Camera");
        camera.x += r
            .input_state
            .axis(r.config.input.left, r.config.input.right);
        camera.y += r.input_state.axis(r.config.input.down, r.config.input.up);
        let camera = *camera;

        if r.input_state
            .events()
            .contains(&InputEvent::KeyDown(r.config.input.editor_tile))
        {
            self.tile = match self.tile {
                Tile::Solid => Tile::Spike,
                Tile::Spike => Tile::Bridge,
                Tile::Bridge => Tile::Grass,
                Tile::Grass => Tile::Solid,
            };
            info!("Set editor tile to {:?}", self.tile);
        }

        if r.input_state.key(Key::LControl) == KeyState::Down
            && r.input_state
                .events()
                .contains(&InputEvent::KeyDown(Key::S))
        {
            r.world.save(c).expect("TODO: error type");
        }

        if r.input_state.mouse(MouseButton::Left) == KeyState::Down {
            let (chunk, tile) = tile_on_click(camera, r);

            let chunk_pos = r.world.chunks.iter().position(|c| c.position == chunk);
            let chunk = if let Some(chunk) = chunk_pos.map(|pos| &mut r.world.chunks[pos]) {
                chunk
            } else {
                r.world.chunks.push(Chunk::empty(chunk, c));
                r.world.chunks.last_mut().unwrap()
            };
            chunk.data.tiles[tile.1 as usize][tile.0 as usize] = Some(self.tile);
            chunk.rebuild(ctx, c)?;
        } else if r.input_state.mouse(MouseButton::Right) == KeyState::Down {
            let (chunk, tile) = tile_on_click(camera, r);

            let chunk_pos = r.world.chunks.iter().position(|c| c.position == chunk);
            if let Some(chunk) = chunk_pos.map(|pos| &mut r.world.chunks[pos]) {
                chunk.data.tiles[tile.1 as usize][tile.0 as usize] = None;
                chunk.rebuild(ctx, c)?;
            }
        }

        Ok(())
    }
}
