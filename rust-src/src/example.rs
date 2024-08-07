use std::fmt;
use godot::prelude::*;
use godot::classes::Node;

#[derive(GodotClass)]
#[class(base=Node)]
struct ExampleNode
{
    name: GString,
}

#[godot_api]
impl INode for ExampleNode {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            name: GString::from("Hello World")
        }
    }
}

#[godot_api]
impl ExampleNode {
    #[func]
    fn hello_world(&self) {
        godot_print!("{:?}", self.name);
    }
}