use std::sync::Mutex;
use godot::prelude::*;
use godot::obj::{Gd, DynGd};
use anyhow::*;

pub trait UIValueValidator
{
    fn validate_value(&self, value: &GString) -> Result<(), GString>;
}

#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
pub(crate) struct ValidatorsHolder
{
    base: Base<Resource>,
    #[export]
    godot_validators_reference: VariantArray,
    validators: Option<Vec<DynGd<Resource, dyn  UIValueValidator>>>,
}

#[godot_api]
impl ValidatorsHolder
{
    pub fn setup_validators(&mut self)
    {
        if self.godot_validators_reference.len() == 0 { return; }
        let mut validators: Vec<DynGd<Resource, dyn  UIValueValidator>> = vec![];
        
        for validator_ref in self.godot_validators_reference.iter_shared() 
        { 
            let cast_result = validator_ref.try_to::<DynGd<Resource, dyn UIValueValidator>>();
            if cast_result.is_err() { continue }
            
            let validator = cast_result.unwrap();
            validators.push(validator);
        }
        
        self.validators = Some(validators);
    }
}