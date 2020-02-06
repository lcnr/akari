use crow::glutin::{Event, EventsLoop, WindowEvent};

pub struct InputState {}

impl Default for InputState {
    fn default() -> Self {
        InputState::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        InputState {}
    }

    pub fn update(&mut self, events_loop: &mut EventsLoop) -> bool {
        let mut fin = false;

        events_loop.poll_events(|e| match e {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => fin = true,
                _ => (),
            },
            _ => (),
        });

        fin
    }
}
