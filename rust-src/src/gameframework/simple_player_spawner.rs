use godot::classes::{Marker2D, PackedScene};
use godot::prelude::*;

use crate::gameframework::{Player, PlayerSpawner};

#[derive(GodotClass)]
#[class(base = Node, init)]
struct SimplePlayerSpawner {
    base: Base<Node>,
    spawn_position: Option<Gd<Marker2D>>,
    player_scene: Option<Gd<PackedScene>>,
}

#[godot_dyn]
impl PlayerSpawner for SimplePlayerSpawner {
    fn spawn_player(
        &mut self,
        mut parent: Gd<Node2D>,
    ) -> anyhow::Result<DynGd<Node2D, dyn Player>> {
        if self.player_scene.is_none()
            || self.spawn_position.is_none()
            || !parent.is_instance_valid()
        {
            return Err(anyhow::anyhow!(""));
        }

        let player_scene_ref = self.player_scene.as_mut().unwrap();
        let player_ref = player_scene_ref.try_instantiate_as::<Node2D>();

        if player_ref.is_none() {
            return Err(anyhow::anyhow!(""));
        }

        let mut player_istance = player_ref.unwrap();
        let player_istance_var = player_istance.to_variant();
        let player_ptr_cast = player_istance_var.try_to::<DynGd<Node2D, dyn Player>>();

        if let Err(_) = player_ptr_cast {
            player_istance.queue_free();
            return Err(anyhow::anyhow!(""));
        }

        parent.add_child(&player_istance);
        let mut player_ptr = player_ptr_cast.unwrap();

        {
            let mut controller = player_ptr.dyn_bind_mut().get_controlled_entity();
            let spawn_point = self.spawn_position.as_ref().unwrap();

            let position = spawn_point.get_global_position();

            controller
                .dyn_bind_mut()
                .override_state_basic(position, 0.0);
        }

        Ok(player_ptr)
    }
}
