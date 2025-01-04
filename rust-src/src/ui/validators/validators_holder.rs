use godot::prelude::*;
use godot::obj::{DynGd};

use crate::ui::validators::{UIValueValidator, ValidationResponse};

#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
pub(crate) struct ValidatorsHolder
{
    base: Base<Resource>,
    #[export]
    godot_validators_reference: VariantArray,
    validators: Option<Vec<DynGd<Resource, dyn UIValueValidator>>>,
}

#[godot_api]
impl ValidatorsHolder
{
    pub fn setup_validators(&mut self)
    {
        if self.godot_validators_reference.len() == 0 { return; }
        if self.validators.is_some() { return; }

        let mut validators: Vec<DynGd<Resource, dyn UIValueValidator>> = vec![];
        
        for validator_ref in self.godot_validators_reference.iter_shared() 
        { 
            let cast_result = validator_ref.try_to::<DynGd<Resource, dyn UIValueValidator>>();
            if cast_result.is_err() { continue }
            
            let validator = cast_result.unwrap();
            validators.push(validator);
        }
        
        self.validators = Some(validators);
    }

    pub fn validate(&mut self, value: &GString) -> anyhow::Result<(), Vec<ValidationResponse>>
    {
        if self.validators.is_none() { return Ok(()) }
        let validators = self.validators.as_mut().unwrap();
        if validators.len() == 0 { return Ok(()) }
        
        let mut errors: Vec<ValidationResponse> = vec![];

        for validator in validators.iter_mut() 
        {
            let mut dyn_validator_ref = validator.dyn_bind_mut();
            let validation_result = dyn_validator_ref.validate_value(&value);
            
            if !validation_result.is_err() { continue; }
            let error = validation_result.unwrap_err();
            errors.push(error);
        }
        
        if errors.len() > 0 { return Err(errors); }
        Ok(())
    }
}