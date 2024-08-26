use godot::prelude::*;
use godot::classes::{Object};

#[derive(GodotClass)]
#[class(base = Object, init)]
struct RegexLineEditWrapper {
    base: Base<Object>,
}

impl RegexLineEditWrapper {
    pub fn construct() -> Gd<RegexLineEditWrapper> {
        todo!()
    }
}

impl Drop for RegexLineEditWrapper {
    fn drop(&mut self) {
        todo!()
    }
}