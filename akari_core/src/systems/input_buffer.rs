use crate::{
    config::InputBufferConfig,
    input::{InputEvent, Key},
    ressources::JumpBuffer,
};

#[derive(Debug)]
pub struct InputBufferSystem;

impl InputBufferSystem {
    pub fn run(
        &mut self,
        events: &[InputEvent],
        space: &mut Option<JumpBuffer>,
        config: &InputBufferConfig,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("run");

        if let &mut Some(JumpBuffer(c)) = space {
            *space = c.checked_sub(1).map(JumpBuffer);
        }

        for event in events {
            if let InputEvent::KeyDown(Key::Space) = event {
                *space = Some(JumpBuffer(config.jump_buffer_frames))
            }
        }
    }
}
