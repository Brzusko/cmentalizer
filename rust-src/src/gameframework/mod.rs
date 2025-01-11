use godot::builtin::GString;
use godot::classes::Node2D;
use godot::prelude::{DynGd, Gd, Vector2};

use game_input::InputData;

trait Player {
    fn get_controlled_entity(&self) -> DynGd<Node2D, dyn ControlledEntity>;
}

trait ControlledEntity {
    fn apply_vertical_input(&mut self, input: Vector2);
    fn apply_aim_offset(&mut self, aim_offset: Vector2);
    fn override_state_basic(&mut self, new_position: Vector2, new_rotation: f32);
}

trait EntityController {
    fn controll_entity(&mut self, entity_to_controll: DynGd<Node2D, dyn ControlledEntity>);
    fn revoke_controll_current_entity(&mut self);
    fn movement_input(&self, input: InputData);
}
trait PlayerSpawner {
    fn spawn_player(&mut self, parent: Gd<Node2D>) -> anyhow::Result<DynGd<Node2D, dyn Player>>;
}

trait GameMode {
    // maybe generic data?
    fn initialize(&mut self) -> anyhow::Result<(), GString>;
}

pub(crate) mod game_input;
pub(crate) mod player_body_controller;
pub(crate) mod player_entity;
pub(crate) mod simple_player_spawner;
pub(crate) mod test_gamemode;
