use crow::glutin::{
    ElementState, Event, EventsLoop, KeyboardInput, VirtualKeyCode as Key, WindowEvent,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    ButtonDown(Button),
    ButtonUp(Button),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Down,
    Up,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Up
    }
}

#[derive(Debug, Clone, Default)]
pub struct InputState {
    pub down: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,
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
                    } => match key {
                        Key::Space => self.events.push(InputEvent::ButtonDown(Button::Space)),
                        Key::S => self.down = ButtonState::Down,
                        Key::A => self.left = ButtonState::Down,
                        Key::D => self.right = ButtonState::Down,
                        _ => (),
                    },
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state: ElementState::Released,
                                ..
                            },
                        ..
                    } => match key {
                        Key::Space => self.events.push(InputEvent::ButtonUp(Button::Space)),
                        Key::S => self.down = ButtonState::Up,
                        Key::A => self.left = ButtonState::Up,
                        Key::D => self.right = ButtonState::Up,
                        _ => (),
                    },
                    _ => (),
                }
            }
        });

        fin
    }

    pub fn events(&self) -> &[InputEvent] {
        &self.events
    }
}
