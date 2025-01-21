use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::gameframework::ControlledEntity;

use super::{EntityInput, InputData};

#[derive(GodotClass)]
#[class(base = CharacterBody2D, init)]
struct PlayerBodyController {
    base: Base<CharacterBody2D>,
    #[export]
    player_camera: Option<Gd<Camera2D>>,
    is_controlled: bool,
}

#[godot_dyn]
impl ControlledEntity for PlayerBodyController {
    fn process_input(&mut self, input: EntityInput) {
        match input {
            EntityInput::Player(input_data) => godot_print!("{:?}", input_data),
        }
    }

    fn override_state_basic(&mut self, new_position: Vector2, new_rotation: f32) {
        let mut base_mut = self.base_mut();

        base_mut.set_global_position(new_position);
        base_mut.set_global_rotation(new_rotation);
    }

    fn getting_controlled(&mut self) {
        self.is_controlled = true;
        let camera = self.player_camera.as_mut().unwrap();
        camera.make_current();
    }

    fn revoking_controll(&mut self) {
        self.is_controlled = false;
    }

    fn is_controlled(&self) -> bool {
        self.is_controlled
    }
}
