#![allow(clippy::too_many_arguments)]
#![allow(clippy::match_ref_pats)]
#![warn(clippy::cast_lossless)]

#[cfg(feature = "profiler")]
#[macro_use]
extern crate thread_profiler;

#[macro_use]
extern crate log;

use std::path::Path;

use crow::{
    glutin::{EventsLoop, Icon, WindowBuilder},
    image, Context, DrawConfig, Texture,
};

pub mod config;
pub mod data;
pub mod input;
pub mod physics;
pub mod ressources;
pub mod spritesheet;
pub mod systems;
pub mod time;

use crate::{config::GameConfig, data::Components, ressources::Ressources, systems::Systems};

pub struct GlobalState {
    pub s: Systems,
    pub r: Ressources,
    pub c: Components,
    pub ctx: Context,
}

impl GlobalState {
    pub fn new(config: GameConfig) -> Result<Self, crow::Error> {
        let icon = load_window_icon(&config.window.icon_path).unwrap();

        let ctx = Context::new(
            WindowBuilder::new()
                .with_dimensions(From::from((
                    config.window.size.0 * config.window.scale,
                    config.window.size.1 * config.window.scale,
                )))
                .with_title(&config.window.title)
                .with_window_icon(Some(icon)),
            EventsLoop::new(),
        )?;

        Ok(GlobalState {
            s: Systems::new(),
            r: Ressources::new(config),
            c: Components::new(),
            ctx,
        })
    }

    pub fn run<F>(self, mut frame: F) -> Result<(), crow::Error>
    where
        F: FnMut(
            &mut Context,
            &mut Texture,
            &mut Systems,
            &mut Ressources,
            &mut Components,
        ) -> Result<bool, crow::Error>,
    {
        let GlobalState {
            mut s,
            mut r,
            mut c,
            mut ctx,
        } = self;

        let mut surface = ctx.window_surface();
        let mut screen_buffer = Texture::new(&mut ctx, r.config.window.size)?;

        r.time.restart();
        loop {
            #[cfg(feature = "profiler")]
            profile_scope!("frame");
            ctx.clear_color(&mut screen_buffer, (0.3, 0.3, 0.8, 1.0))?;
            screen_buffer.clear_depth(&mut ctx)?;

            if frame(&mut ctx, &mut screen_buffer, &mut s, &mut r, &mut c)? {
                break Ok(());
            }

            ctx.draw(
                &mut surface,
                &screen_buffer,
                (0, 0),
                &DrawConfig {
                    scale: (r.config.window.scale, r.config.window.scale),
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
