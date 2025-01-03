use godot::prelude::*;
use godot::classes::{Resource, IResource};
use crate::ui::validators::validators_holder;

#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
pub(crate) struct MinValidatorResource
{
    base: Base<Resource>,
    #[export]
    min_characters: i64,
}

struct MinValidator
{
    min_characters: i64,
}


impl MinValidator
{
    fn new(min_characters: i64) -> Self
    {
        Self
        {
            min_characters,
        }
    }
}
