use godot::builtin::GString;
use godot::classes::{Camera2D, Node2D};
use godot::prelude::{Callable, DynGd, Gd, Vector2};

trait Player {
    fn get_controlled_entity(&self) -> DynGd<Node2D, dyn ControlledEntity>;
}

trait ControlledEntity {
    fn apply_vertical_input(&mut self, input: Vector2);
    fn apply_aim_offset(&mut self, aim_offset: Vector2);
    fn override_state_basic(&mut self, new_position: Vector2, new_rotation: f64);
}

trait PlayerSpawner {
    fn spawn_player(&mut self, parent: Gd<Node2D>, event_handler: Callable) -> anyhow::Result<()>;
}

trait GameMode {
    // maybe generic data?
    fn initialize(&mut self) -> anyhow::Result<(), GString>;
}

pub(crate) mod simple_player_spawner;
pub(crate) mod test_gamemode;
