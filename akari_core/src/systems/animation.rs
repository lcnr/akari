use crow_ecs::{Entities, Joinable, Storage};

use crow_anim::{AnimationState, AnimationStorage, Sprite};

#[derive(Debug)]
pub struct AnimationSystem;

impl AnimationSystem {
    pub fn run(
        &mut self,
        sprites: &mut Storage<Sprite>,
        animations: &mut Storage<AnimationState>,
        animation_storage: &mut AnimationStorage,
    ) {
        for (animation, entity) in (animations, Entities).join() {
            sprites.insert(entity, animation_storage.next(animation));
        }
    }
}
