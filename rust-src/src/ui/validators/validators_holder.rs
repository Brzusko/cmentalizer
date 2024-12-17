use godot::prelude::*;
use anyhow::*;
use crate::ui::validators::max_validator::MaxValidatorResource;
use crate::ui::validators::min_validator::MinValidatorResource;
use crate::ui::validators::regex_validator::RegexValidatorResource;


pub struct ValidatorErrorData
{

}

pub enum 

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
    regex_validator_constructors: Array<Gd<RegexValidatorResource>>,
    #[export]
    min_validator_constructors: Array<Gd<MinValidatorResource>>,
    #[export]
    max_validator_constructors: Array<Gd<MaxValidatorResource>>,
}

#[godot_api]
impl ValidatorsHolder
{
}