use crow::glutin::{ElementState, Event, EventsLoop, KeyboardInput, WindowEvent};

pub use crow::glutin::{MouseButton, VirtualKeyCode as Key};

use crate::{config::WindowConfig, environment::CHUNK_HEIGHT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    KeyDown(Key),
    KeyUp(Key),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Down,
    Up,
}

#[derive(Debug, Clone, Default)]
pub struct InputState {
    pressed: Vec<Key>,
    mouse_pressed: Vec<MouseButton>,
    events: Vec<InputEvent>,
    cursor_position: (i32, i32),
}

impl InputState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, events_loop: &mut EventsLoop, window_config: &WindowConfig) -> bool {
        self.events.clear();
        let mut fin = false;

        events_loop.poll_events(|e| {
            if let Event::WindowEvent { event, .. } = e {
                match event {
                    WindowEvent::CloseRequested => fin = true,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        if !self.pressed.contains(&key) {
                            self.pressed.push(key);
                            self.events.push(InputEvent::KeyDown(key));
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state: ElementState::Released,
                                ..
                            },
                        ..
                    } => {
                        if let Some(idx) = self.pressed.iter().position(|&i| i == key) {
                            self.pressed.remove(idx);
                            self.events.push(InputEvent::KeyUp(key));
                        }
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button,
                        ..
                    } => {
                        if !self.mouse_pressed.contains(&button) {
                            self.mouse_pressed.push(button);
                            self.events.push(InputEvent::MouseDown(button));
                        }
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Released,
                        button,
                        ..
                    } => {
                        if let Some(idx) = self.mouse_pressed.iter().position(|&i| i == button) {
                            self.mouse_pressed.remove(idx);
                            self.events.push(InputEvent::MouseUp(button));
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let position: (i32, i32) = position.into();
                        let scaled_pos = (
                            position.0 / window_config.scale as i32,
                            position.1 / window_config.scale as i32,
                        );
                        self.cursor_position = if cfg!(feature = "editor") {
                            (scaled_pos.0, CHUNK_HEIGHT as i32 - scaled_pos.1)
                        } else {
                            (scaled_pos.0, window_config.size.1 as i32 - scaled_pos.1)
                        };
                    }
                    _ => (),
                }
            }
        });

        fin
    }

    pub fn key(&self, key: Key) -> KeyState {
        if self.pressed.contains(&key) {
            KeyState::Down
        } else {
            KeyState::Up
        }
    }

    pub fn mouse(&self, button: MouseButton) -> KeyState {
        if self.mouse_pressed.contains(&button) {
            KeyState::Down
        } else {
            KeyState::Up
        }
    }

    pub fn events(&self) -> &[InputEvent] {
        &self.events
    }

    pub fn cursor_position(&self) -> (i32, i32) {
        self.cursor_position
    }
}
