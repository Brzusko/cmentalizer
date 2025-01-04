use godot::prelude::*;
use godot::classes::{Resource};
use crate::ui::validators::{UIValueValidator, ValidationResponse};

#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
pub(crate) struct MaxValidatorResource
{
    base: Base<Resource>,
    #[export]
    max_characters: i64,
    #[export]
    error_key: GString,
    #[export]
    revert: bool,
}

#[godot_dyn]
impl UIValueValidator for MaxValidatorResource
{
    fn validate_value(&mut self, _value: &GString) -> anyhow::Result<(), ValidationResponse>
    {
        let value_length: i64 = _value.len().try_into().unwrap();

        if value_length > self.max_characters
        {
            let error_key = self.error_key.clone();

            if self.revert { return Err(ValidationResponse::Revert(error_key)) }
            return Err(ValidationResponse::Pass(error_key))
        }

        Ok(())
    }
}