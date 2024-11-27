use godot::prelude::*;
use godot::classes::{Resource, IResource};
use crate::ui::validators::validators_holder::{UIValueValidator, ValidateResult, ValidatorConstruct};

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

impl ValidatorConstruct for MaxValidatorResource
{
    fn construct_validator(&self) -> Box<dyn UIValueValidator>
    {
        Box::new(MaxValidator::new(self.max_characters.clone(), self.revert.clone()))
    }
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

impl UIValueValidator for MaxValidator
{
    fn validate_value(&self, value: &GString) -> ValidateResult
    {
        godot_print!("anrdzej");
        if value.len() <= self.max_characters as usize { return ValidateResult::Success; }
        if self.revert { ValidateResult::Revert} else { ValidateResult::Failed }
    }
}