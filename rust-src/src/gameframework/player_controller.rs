use crate::gameframework::game_input::{InputData, InputProvider};
use crate::gameframework::{ControlledEntity, EntityController};
use godot::classes::{INode, Node};
use godot::prelude::*;

use super::EntityInput;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct PlayerController {
    base: Base<Node>,
    #[export]
    input_provider: Option<Gd<InputProvider>>,
    controlled_entity: Option<DynGd<Node2D, dyn ControlledEntity>>,
}

#[godot_api]
impl INode for PlayerController {
    fn ready(&mut self) {
        {
            let input_callback = self.base().callable("on_input_changed");
            let input_provider = self.input_provider.as_mut().unwrap();
            input_provider.connect("input_changed", &input_callback);
        }
    }
}

#[godot_api]
impl PlayerController {
    #[func]
    fn on_input_changed(&mut self, input_data: InputData) {
        if self.controlled_entity.is_none() {
            return;
        }

        let mut controllerd_entity_ptr = self.controlled_entity.as_mut().unwrap().dyn_bind_mut();
        let entity_data = EntityInput::Player(input_data);
        controllerd_entity_ptr.process_input(entity_data);
    }
}

#[godot_dyn]
impl EntityController for PlayerController {
    fn take_controll(&mut self, mut entity_ptr: DynGd<Node2D, dyn ControlledEntity>) {
        self.revoke_current_controll();

        let mut entity_ptr_mut = entity_ptr.dyn_bind_mut();
        entity_ptr_mut.getting_controlled();
        drop(entity_ptr_mut);

        self.controlled_entity = Some(entity_ptr);
    }
    fn revoke_current_controll(&mut self) {
        if self.controlled_entity.is_none() {
            return;
        }

        let mut controlled_entity_ptr = self.controlled_entity.take().unwrap();
        controlled_entity_ptr.dyn_bind_mut().revoking_controll();
    }
}
