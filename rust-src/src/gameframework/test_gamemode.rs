use crate::gameframework::{EntityController, GameMode, Player, PlayerSpawner};
use godot::classes::{INode, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct TestGameMode {
    base: Base<Node>,
    #[export]
    spawner_reference: Option<Gd<Node>>,
    #[export]
    world: Option<Gd<Node2D>>,
    #[export]
    player_controller_reference: Option<Gd<Node>>,

    spawner_ptr: Option<DynGd<Node, dyn PlayerSpawner>>,
    player_ptr: Option<DynGd<Node2D, dyn Player>>,
    player_controller_ptr: Option<DynGd<Node, dyn EntityController>>,
}

#[godot_api]
impl INode for TestGameMode {
    fn ready(&mut self) {
        // TODO initialize should be invoked outside GameMode context
        if let Err(error) = self.initialize() {
            godot_error!("{:?}", error);
            return;
        }

        // world ref is checked in self.initialize
        let world_ptr = self.world.as_mut().unwrap();
        // spawner ref is checked in self.initialize
        let spawner_ptr = self.spawner_ptr.as_mut().unwrap();

        let player_spawn_result = spawner_ptr.dyn_bind_mut().spawn_player(world_ptr.clone());

        if let Err(_) = player_spawn_result {
            godot_error!("GameMode - could not spawn player!");
            return;
        }

        let player_ptr = player_spawn_result.unwrap();

        {
            let player_ptr_dyn = player_ptr.dyn_bind();

            let mut player_controller_ptr =
                self.player_controller_ptr.as_mut().unwrap().dyn_bind_mut();
            player_controller_ptr.take_controll(player_ptr_dyn.get_controlled_entity());
        }

        self.player_ptr = Some(player_ptr);
    }
}

#[godot_dyn]
impl GameMode for TestGameMode {
    fn initialize(&mut self) -> anyhow::Result<(), GString> {
        if self.world.is_none() {
            return Err(GString::from("GameMode - world reference is missing!"));
        }

        if self.spawner_reference.is_none() {
            return Err(GString::from(
                "GameMode - player spawner reference is missing!",
            ));
        }

        {
            let spawner_cast = self
                .spawner_reference
                .as_mut()
                .unwrap()
                .to_variant()
                .try_to::<DynGd<Node, dyn PlayerSpawner>>();

            if let Err(_) = spawner_cast {
                return Err(GString::from(
                    "GameMode - could not grab spawner interface pointer!",
                ));
            }

            self.spawner_ptr = Some(spawner_cast.unwrap());
        }

        if self.player_controller_reference.is_none() {
            return Err(GString::from(
                "GameMode - player controller reference is missing!",
            ));
        }

        {
            let player_controller_cast_result = self
                .player_controller_reference
                .as_ref()
                .unwrap()
                .to_variant()
                .try_to::<DynGd<Node, dyn EntityController>>();

            if player_controller_cast_result.is_err() {
                return Err(GString::from("GameMode - could not receive entity controller trait from player controller reference!"));
            }

            let player_controller_ptr = player_controller_cast_result.unwrap();
            self.player_controller_ptr = Some(player_controller_ptr);
        }

        Ok(())
    }
}
