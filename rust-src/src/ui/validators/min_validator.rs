use godot::prelude::*;
use godot::classes::{Resource};
use crate::ui::validators::{ValidationResponse, UIValueValidator};

#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
pub(crate) struct MinValidatorResource
{
    base: Base<Resource>,
    #[export]
    min_characters: i64,
    #[export]
    error_key: GString,
    #[export]
    revert: bool,
}


#[godot_dyn]
impl UIValueValidator for MinValidatorResource
{
    fn validate_value(&mut self, value: &GString) -> anyhow::Result<(), ValidationResponse>
    {
        let value_length: i64 = value.len().try_into().unwrap();

        if value_length < self.min_characters
        {
            let error_key = self.error_key.clone();

            if self.revert { return Err(ValidationResponse::Revert(error_key)) }
            return Err(ValidationResponse::Pass(error_key))
        }

        Ok(())
    }
}
