use std::convert::TryInto;

use crow::Context;

use crow_ecs::Joinable;

use crate::{
    data::Components,
    environment::{Tile, CHUNK_HEIGHT, CHUNK_WIDTH, TILE_SIZE},
    input::{InputEvent, KeyState, MouseButton},
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

fn tile_on_click(c: &mut Components, r: &mut Ressources) -> ((i32, i32), (usize, usize)) {
    let pos = r.input_state.cursor_position();

    let (&c_pos, _) = (&c.positions, &c.cameras).join().unique().expect("Camera");
    let c_pos: (i32, i32) = c_pos.into();

    let ingame_pos = (pos.0 + c_pos.0, pos.1 + c_pos.1);

    let chunk = (
        ingame_pos.0 / CHUNK_WIDTH as i32,
        ingame_pos.1 / CHUNK_HEIGHT as i32,
    );
    let tile = (
        (ingame_pos.0 - chunk.0 * CHUNK_WIDTH as i32) / TILE_SIZE as i32,
        (ingame_pos.1 - chunk.1 * CHUNK_HEIGHT as i32) / TILE_SIZE as i32,
    );

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

        if r.input_state.mouse(MouseButton::Left) == KeyState::Down {
            let (chunk, tile) = tile_on_click(c, r);

            let chunk_pos = r.world.chunks.iter().position(|c| c.position == chunk);
            if let Some(chunk) = chunk_pos.map(|pos| &mut r.world.chunks[pos]) {
                chunk.data.tiles[tile.1 as usize][tile.0 as usize] = Some(self.tile);
                chunk.rebuild(ctx, c)?;
            } else {
                error!("Tried to edit an nonexisting chunk");
            }
        }

        Ok(())
    }
}
