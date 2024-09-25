use std::rc::Weak;
use godot::prelude::*;
use crate::network::entrypoint::network_entry_point::{NetworkEntryPoint};

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct ENetListener
{
}
