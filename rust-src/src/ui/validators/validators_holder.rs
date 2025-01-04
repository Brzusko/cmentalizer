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

    pub fn validate(&self, value: &GString) -> anyhow::Result<(), Vec<ValidationResponse>>
    {
        todo!()
    }
}