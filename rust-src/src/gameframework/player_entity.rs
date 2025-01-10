use godot::classes::{CharacterBody2D, INode2D, Node2D};
use godot::prelude::*;

use crate::gameframework::{ControlledEntity, Player};

#[derive(GodotClass)]
#[class(base = Node2D, init)]
struct PlayerEntity {
    #[export]
    body_controller_reference: Option<Gd<CharacterBody2D>>,
    body_controller_ptr: Option<DynGd<Node2D, dyn ControlledEntity>>,
}

#[godot_api]
impl INode2D for PlayerEntity {
    fn ready(&mut self) {
        let body_controller_reference = self.body_controller_reference.as_mut().unwrap();
        let body_controller_ptr = body_controller_reference
            .to_variant()
            .try_to::<DynGd<Node2D, dyn ControlledEntity>>()
            .unwrap();

        self.body_controller_ptr = Some(body_controller_ptr);
    }
}

#[godot_dyn]
impl Player for PlayerEntity {
    fn get_controlled_entity(&self) -> DynGd<Node2D, dyn ControlledEntity> {
        let body_controller_ptr = self.body_controller_ptr.clone().unwrap();
        body_controller_ptr
    }
}
