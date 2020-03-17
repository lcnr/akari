#![allow(clippy::too_many_arguments)]
#![warn(clippy::cast_lossless, clippy::match_ref_pats)]

#[cfg(feature = "profiler")]
#[macro_use]
extern crate thread_profiler;

#[macro_use]
extern crate log;

use std::path::Path;

use crow::{
    glutin::{
        dpi::LogicalSize,
        event::Event,
        event_loop::{ControlFlow, EventLoop},
        window::Icon,
        window::WindowBuilder,
    },
    image, Context, DrawConfig, Texture,
};

pub mod config;
pub mod data;
pub mod environment;
pub mod init;
pub mod input;
pub mod physics;
pub mod ressources;
pub mod save;
pub mod spritesheet;
pub mod systems;
pub mod time;

use crate::{
    config::GameConfig, data::Components, environment::WorldData, ressources::Ressources,
    save::SaveData, systems::Systems,
};

#[cfg(feature = "editor")]
use environment::{CHUNK_HEIGHT, CHUNK_WIDTH};

pub struct GlobalState {
    pub s: Systems,
    pub r: Ressources,
    pub c: Components,
    pub ctx: Context,
    event_loop: EventLoop<()>,
}

impl GlobalState {
    pub fn new(
        config: GameConfig,
        world_data: WorldData,
        save_data: SaveData,
    ) -> Result<Self, crow::Error> {
        let icon = load_window_icon(&config.window.icon_path).unwrap();

        #[cfg(not(feature = "editor"))]
        let window_size = (
            config.window.size.0 * config.window.scale,
            config.window.size.1 * config.window.scale,
        );

        #[cfg(feature = "editor")]
        let window_size = (
            CHUNK_WIDTH as u32 * config.window.scale,
            CHUNK_HEIGHT as u32 * config.window.scale,
        );

        let event_loop = EventLoop::new();
        let ctx = Context::new(
            WindowBuilder::new()
                .with_inner_size(LogicalSize::<u32>::from(window_size))
                .with_title(&config.window.title)
                .with_window_icon(Some(icon)),
            &event_loop,
        )?;

        Ok(GlobalState {
            s: Systems::new(),
            r: Ressources::new(config, world_data, save_data),
            c: Components::new(),
            ctx,
            event_loop,
        })
    }

    pub fn run<F>(self, mut frame: F) -> !
    where
        F: 'static
            + FnMut(
                &mut Context,
                &mut Texture,
                &mut Systems,
                &mut Components,
                &mut Ressources,
            ) -> Result<bool, crow::Error>,
    {
        let GlobalState {
            mut s,
            mut r,
            mut c,
            mut ctx,
            event_loop,
        } = self;
        #[cfg(not(feature = "editor"))]
        let mut screen_buffer = Texture::new(&mut ctx, r.config.window.size).unwrap();
        #[cfg(feature = "editor")]
        let mut screen_buffer =
            Texture::new(&mut ctx, (CHUNK_WIDTH as u32, CHUNK_HEIGHT as u32)).unwrap();

        r.time.restart();
        event_loop.run(
            move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
                Event::NewEvents(_) => r.input_state.clear_events(),
                Event::MainEventsCleared => ctx.window().request_redraw(),
                Event::RedrawRequested(_) => {
                    #[cfg(feature = "profiler")]
                    profile_scope!("frame");

                    let mut surface = ctx.surface();
                    ctx.clear_color(&mut screen_buffer, (0.3, 0.3, 0.8, 1.0));
                    ctx.clear_depth(&mut screen_buffer);

                    for event in r.input_state.events() {
                        if &input::InputEvent::KeyDown(r.config.input.debug_toggle) == event {
                            r.debug_draw = !r.debug_draw;
                        }
                    }

                    if frame(&mut ctx, &mut screen_buffer, &mut s, &mut c, &mut r).unwrap() {
                        *control_flow = ControlFlow::Exit;
                    }

                    let fadeout = r.fadeout.as_ref().map_or(0.0, |f| f.current);
                    let color_modulation = [
                        [1.0 - fadeout, 0.0, 0.0, 0.0],
                        [0.0, 1.0 - fadeout, 0.0, 0.0],
                        [0.0, 0.0, 1.0 - fadeout, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ];

                    ctx.draw(
                        &mut surface,
                        &screen_buffer,
                        (0, 0),
                        &DrawConfig {
                            scale: (r.config.window.scale, r.config.window.scale),
                            color_modulation,
                            ..Default::default()
                        },
                    );

                    ctx.present(surface).unwrap();
                    r.time.frame();
                }
                Event::LoopDestroyed => {
                    #[cfg(feature = "profiler")]
                    thread_profiler::write_profile("profile.json");
                }
                e => {
                    if r.input_state.update(e, &r.config.window) {
                        *control_flow = ControlFlow::Exit;
                    }
                }
            },
        )
    }
}

#[cfg(feature = "editor")]
pub fn editor_frame(
    ctx: &mut Context,
    screen_buffer: &mut Texture,
    s: &mut Systems,
    c: &mut Components,
    r: &mut Ressources,
) -> Result<bool, crow::Error> {
    s.editor.run(ctx, c, r)?;

    systems::draw::scene(
        ctx,
        screen_buffer,
        &c.positions,
        &c.sprites,
        &c.depths,
        &c.mirrored,
        &c.colliders,
        &c.cameras,
    )?;

    if r.debug_draw {
        systems::draw::debug_colliders(ctx, screen_buffer, &c.positions, &c.colliders, &c.cameras)?;
    }

    Ok(false)
}

pub fn game_frame(
    ctx: &mut Context,
    screen_buffer: &mut Texture,
    s: &mut Systems,
    c: &mut Components,
    r: &mut Ressources,
) -> Result<bool, crow::Error> {
    s.input_buffer.run(
        r.input_state.events(),
        &mut r.pressed_space,
        &r.config.input_buffer,
        &r.config.input,
    );

    s.camera.run(
        &c.player_state,
        &c.positions,
        &c.previous_positions,
        &mut c.velocities,
        &c.cameras,
        &r.time,
        &r.config.camera,
    );

    s.gravity
        .run(&c.gravity, &mut c.velocities, &r.time, &r.config.gravity);

    let mut collisions = s.physics.run(
        &c.velocities,
        &c.colliders,
        &mut c.previous_positions,
        &mut c.positions,
        &mut c.grounded,
        &r.time,
    );

    s.bridge_collision.run(
        &c.positions,
        &c.previous_positions,
        &c.colliders,
        &c.ignore_bridges,
        &mut collisions,
    );

    s.fixed_collision.run(
        &mut c.positions,
        &c.previous_positions,
        &mut c.grounded,
        &mut c.wall_collisions,
        &mut c.velocities,
        &c.colliders,
        &collisions,
    );

    s.player.run(c, r, &collisions);

    s.environment.run(ctx, c, r)?;

    s.fadeout.run(&mut r.fadeout);

    s.animation
        .run(&mut c.sprites, &mut c.animations, &mut r.animation_storage);

    s.delayed_actions(ctx, c, r)?;

    systems::draw::scene(
        ctx,
        screen_buffer,
        &c.positions,
        &c.sprites,
        &c.depths,
        &c.mirrored,
        &c.colliders,
        &c.cameras,
    )?;

    if r.debug_draw {
        systems::draw::debug_colliders(ctx, screen_buffer, &c.positions, &c.colliders, &c.cameras)?;
    }

    Ok(false)
}

pub fn load_window_icon<P: AsRef<Path>>(path: P) -> Result<Icon, image::ImageError> {
    let icon = image::open(path)?.to_rgba();
    let icon_dimensions = icon.dimensions();
    Ok(Icon::from_rgba(icon.into_raw(), icon_dimensions.0, icon_dimensions.1).unwrap())
}
