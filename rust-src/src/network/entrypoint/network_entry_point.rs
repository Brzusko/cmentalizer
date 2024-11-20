use godot::prelude::*;
use godot::classes::{INode, Node};


#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct NetworkEntryPoint
{
    base: Base<Node>,
}

#[godot_api]
impl INode for NetworkEntryPoint
{
}

#[godot_api]
impl NetworkEntryPoint
{
}