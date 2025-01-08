use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct TestGameMode {
    base: Base<Node>,
}
