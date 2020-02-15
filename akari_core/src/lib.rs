#![allow(clippy::too_many_arguments)]
#![allow(clippy::match_ref_pats)]
#![warn(clippy::cast_lossless)]

#[cfg(feature = "profiler")]
#[macro_use]
extern crate thread_profiler;

#[macro_use]
extern crate log;

use std::path::Path;

use crow::{glutin::Icon, image, Context, DrawConfig, Texture};

pub mod config;
pub mod data;
pub mod environment;
pub mod input;
pub mod physics;
pub mod ressources;
pub mod spritesheet;
pub mod systems;
pub mod time;

use crate::{data::Components, ressources::Ressources, systems::Systems};

pub const ARENA_WIDTH: usize = 16;
pub const ARENA_HEIGHT: usize = 12;
pub const GAME_SIZE: (u32, u32) = (20 * ARENA_WIDTH as u32, 20 * ARENA_HEIGHT as u32);
pub const WINDOW_SCALE: u32 = 3;
pub const FPS: u32 = 60;

pub struct GlobalState {
    pub s: Systems,
    pub r: Ressources,
    pub c: Components,
}

impl GlobalState {
    pub fn new(fps: u32) -> Self {
        GlobalState {
            s: Systems::new(),
            r: Ressources::new(fps),
            c: Components::new(),
        }
    }

    pub fn run<F>(self, ctx: &mut Context, mut f: F) -> Result<(), crow::Error>
    where
        F: FnMut(
            &mut Context,
            &mut Texture,
            &mut Systems,
            &mut Ressources,
            &mut Components,
        ) -> Result<bool, crow::Error>,
    {
        let (mut s, mut r, mut c) = (self.s, self.r, self.c);

        let mut surface = ctx.window_surface();
        let mut screen_buffer = Texture::new(ctx, GAME_SIZE)?;

        loop {
            #[cfg(feature = "profiler")]
            profile_scope!("frame");
            ctx.clear_color(&mut screen_buffer, (0.3, 0.3, 0.8, 1.0))?;
            screen_buffer.clear_depth(ctx)?;

            if f(ctx, &mut screen_buffer, &mut s, &mut r, &mut c)? {
                break Ok(());
            }

            ctx.draw(
                &mut surface,
                &screen_buffer,
                (0, 0),
                &DrawConfig {
                    scale: (WINDOW_SCALE, WINDOW_SCALE),
                    ..Default::default()
                },
            )?;
            ctx.finalize_frame()?;
            r.time.frame();
        }
    }
}

pub fn load_window_icon<P: AsRef<Path>>(path: P) -> Result<Icon, image::ImageError> {
    let icon = image::open(path)?.to_rgba();
    let icon_dimensions = icon.dimensions();
    Ok(Icon::from_rgba(icon.into_raw(), icon_dimensions.0, icon_dimensions.1).unwrap())
}
