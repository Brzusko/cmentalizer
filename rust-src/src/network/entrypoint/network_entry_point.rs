use godot::prelude::*;
use godot::classes::{IObject, Object};


// Refactor as Engine Singleton later
#[derive(GodotClass)]
#[class(base = Object, init)]
pub(crate) struct NetworkEntryPoint
{
    base: Base<Object>,
}

#[godot_api]
impl IObject for NetworkEntryPoint 
{
}

impl NetworkEntryPoint
{
}