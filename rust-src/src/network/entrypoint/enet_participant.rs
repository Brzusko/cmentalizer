use std::rc::Weak;
use godot::prelude::*;
use crate::network::entrypoint::network_entry_point::{NetworkEntryPoint, NetworkConstruct, NetworkListener};
use crate::network::entrypoint::network_construct_data::NetworkConstructData;

pub(crate) struct ENetParticipantConstructData
{
    address: GString,
    port: i32,
    channels: i32,
}

impl ENetParticipantConstructData
{
    pub(crate) fn new(address: GString, port: i32, channels: i32) -> Self
    {
        Self
        {
            address,
            port,
            channels,
        }
    }
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct ENetParticipant
{
    network_entry_point: Option<Gd<NetworkEntryPoint>>
}

impl NetworkConstruct for ENetParticipant
{
    fn construct(&mut self, mut network_entry_point: Gd<NetworkEntryPoint>, meta_data: NetworkConstructData)
    {
        if let NetworkConstructData::ENetParticipant(data) = meta_data {
            self.network_entry_point = Some(network_entry_point);
            // TODO
        }
        else
        {
            network_entry_point.bind_mut().construct_result(false);
        }
    }
}

impl NetworkListener for ENetParticipant {}