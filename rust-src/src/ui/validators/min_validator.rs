use godot::prelude::*;
use godot::classes::{Resource, IResource};
use crate::ui::validators::validators_holder::{ValidateResult, ValidatorConstruct, UIValueValidator};

#[derive(GodotClass)]
#[class(base = Resource, init)]
pub(crate) struct MinValidatorResource
{
    base: Base<Resource>,
    #[export]
    min_characters: i64,
}

impl ValidatorConstruct for MinValidatorResource
{
    fn construct_validator(&self) -> Box<dyn UIValueValidator>
    {
        Box::new(MinValidator::new(self.min_characters.clone()))
    }
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

impl UIValueValidator for MinValidator
{
    fn validate_value(&self, value: &GString) -> ValidateResult
    {
        if value.len() > self.min_characters as usize { return ValidateResult::Success; }
        ValidateResult::Failed
    }
}
