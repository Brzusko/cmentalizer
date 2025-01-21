use godot::builtin::GString;
use godot::classes::Node2D;
use godot::prelude::{DynGd, Gd, Vector2};

use crate::gameframework::game_input::InputData;

trait Player {
    fn get_controlled_entity(&self) -> DynGd<Node2D, dyn ControlledEntity>;
}

trait ControlledEntity {
    fn process_input(&mut self, input: EntityInput);
    fn override_state_basic(&mut self, new_position: Vector2, new_rotation: f32);
    fn getting_controlled(&mut self);
    fn revoking_controll(&mut self);
    fn is_controlled(&self) -> bool;
}

pub(crate) enum EntityInput {
    Player(InputData),
}

trait EntityController {
    fn take_controll(&mut self, entity_ptr: DynGd<Node2D, dyn ControlledEntity>);
    fn revoke_current_controll(&mut self);
}
trait PlayerSpawner {
    fn spawn_player(&mut self, parent: Gd<Node2D>) -> anyhow::Result<DynGd<Node2D, dyn Player>>;
}

trait GameMode {
    fn initialize(&mut self) -> anyhow::Result<(), GString>;
}

pub(crate) mod game_input;
pub(crate) mod player_body_controller;
pub(crate) mod player_controller;
pub(crate) mod player_entity;
pub(crate) mod simple_player_spawner;
pub(crate) mod test_gamemode;
