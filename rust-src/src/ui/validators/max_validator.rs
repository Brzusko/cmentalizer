use godot::prelude::*;
use godot::classes::{Resource, IResource};
use crate::ui::validators::validators_holder::{UIValueValidator};

#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
pub(crate) struct MaxValidatorResource
{
    base: Base<Resource>,
    #[export]
    max_characters: i64,
    #[export]
    revert: bool,
}

struct MaxValidator
{
    max_characters: i64,
    revert: bool,
}


impl MaxValidator
{
    fn new(max_characters: i64, revert: bool) -> Self
    {
        Self
        {
            max_characters,
            revert,
        }
    }
}

#[godot_dyn]
impl UIValueValidator for MaxValidatorResource
{
    fn validate_value(&self, value: &GString) -> anyhow::Result<(), GString>
    {
        todo!()
    }
}