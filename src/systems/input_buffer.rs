use crate::{
    config::{InputBufferConfig, InputConfig},
    input::InputEvent,
    ressources::JumpBuffer,
};

#[derive(Debug)]
pub struct InputBufferSystem;

impl InputBufferSystem {
    pub fn run(
        &mut self,
        events: &[InputEvent],
        space: &mut Option<JumpBuffer>,
        buffer_config: &InputBufferConfig,
        input_config: &InputConfig,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("run");

        if let Some(JumpBuffer(c)) = *space {
            *space = c.checked_sub(1).map(JumpBuffer);
        }

        for event in events {
            if &InputEvent::KeyDown(input_config.jump) == *event {
                *space = Some(JumpBuffer(buffer_config.jump_buffer_frames))
            }
        }
    }
}
