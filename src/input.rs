use crow::glutin::{ElementState, Event, EventsLoop, KeyboardInput, WindowEvent};

pub use crow::glutin::VirtualKeyCode as Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    KeyDown(Key),
    KeyUp(Key),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Down,
    Up,
}

#[derive(Debug, Clone, Default)]
pub struct InputState {
    pressed: Vec<Key>,
    events: Vec<InputEvent>,
}

impl InputState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, events_loop: &mut EventsLoop) -> bool {
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

    pub fn events(&self) -> &[InputEvent] {
        &self.events
    }
}
