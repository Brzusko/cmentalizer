use super::GameMode;
use crate::gameframework::{Player, PlayerSpawner};
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
    spawner_ptr: Option<DynGd<Node, dyn PlayerSpawner>>,
    player_ptr: Option<DynGd<Node2D, dyn Player>>,
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

        self.player_ptr = Some(player_spawn_result.unwrap());
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
                    "GameMode - could not grab spawner interface pointer.",
                ));
            }

            self.spawner_ptr = Some(spawner_cast.unwrap());
        }

        Ok(())
    }
}
