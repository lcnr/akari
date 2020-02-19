use crate::ressources::Fadeout;

#[derive(Debug)]
pub struct FadeoutSystem;

impl FadeoutSystem {
    pub fn run(&mut self, fadeout: &mut Option<Fadeout>) {
        if let Some(fadeout) = fadeout {
            if let Some(new_frames_left) = fadeout.frames_left.checked_sub(1) {
                let diff = 1.0 - fadeout.current;
                fadeout.current += diff / fadeout.frames_left as f32;
                fadeout.frames_left = new_frames_left;
            }
        }
    }
}
