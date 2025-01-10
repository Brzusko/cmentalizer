use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::gameframework::ControlledEntity;

#[derive(GodotClass)]
#[class(base = CharacterBody2D, init)]
struct PlayerBodyController {
    base: Base<CharacterBody2D>,
}

#[godot_dyn]
impl ControlledEntity for PlayerBodyController {
    fn apply_vertical_input(&mut self, input: Vector2) {
        todo!()
    }
    fn apply_aim_offset(&mut self, aim_offset: Vector2) {
        todo!()
    }
    fn override_state_basic(&mut self, new_position: Vector2, new_rotation: f32) {
        let mut base_mut = self.base_mut();

        base_mut.set_global_position(new_position);
        base_mut.set_global_rotation(new_rotation);
    }
}
