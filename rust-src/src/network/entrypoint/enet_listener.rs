use std::rc::Weak;
use godot::prelude::*;
use crate::network::entrypoint::network_entry_point::{ConnectionResponse, Constructor, NetworkEntryPoint};

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct ENetListener
{
}

impl Constructor for ENetListener
{
    fn construct(&mut self, mut entry_point: ()) 
    {
        entry_point.bind_mut().construct_result(ConnectionResponse::Success);
    }
}