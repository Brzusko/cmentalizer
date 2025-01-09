use crate::gameframework::{Player, PlayerSpawner};
use anyhow::anyhow;
use godot::classes::{INode, Node};
use godot::prelude::*;

use super::GameMode;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct TestGameMode {
    base: Base<Node>,
    #[export]
    spawner_reference: Option<Gd<Node>>,
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
    }
}

#[godot_dyn]
impl GameMode for TestGameMode {
    fn initialize(&mut self) -> anyhow::Result<(), GString> {
        if self.spawner_ptr.is_none() {
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
