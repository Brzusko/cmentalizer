use std::rc::Weak;
use godot::prelude::*;
use crate::network::entrypoint::network_construct_data::NetworkConstructData;
use crate::network::entrypoint::network_entry_point::{NetworkEntryPoint, NetworkConstruct, NetworkListener};

pub(crate) struct ENetListenerConstructData
{
}

impl Default for ENetListenerConstructData {
    fn default() -> Self {
        Self {}
    }
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct ENetListener
{
    base: Base<Node>,
    op: Option<Gd<NetworkEntryPoint>>
}

impl ENetListener
{
    pub fn test(&mut self) {
        self.op.as_mut().unwrap().bind_mut().construct_result(false);
    }
}

impl NetworkListener for ENetListener {}

impl NetworkConstruct for ENetListener
{
    fn construct(&mut self, mut network_entry_point: Gd<NetworkEntryPoint>, meta_data: NetworkConstructData) {
        if let NetworkConstructData::ENetListener(data) = meta_data {
            // TODO IMPL CONNECTION TO ENET HOST
            self.op = Some(network_entry_point);

        }
        else
        {
            //network_entry_point.bind_mut().construct_result(false);
        }
    }
}
