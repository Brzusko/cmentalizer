use godot::prelude::*;
use godot::classes::{INode, Node};
use crate::ui::validators::{min_validator, max_validator, regex_validator};
use crate::ui::validators::max_validator::MaxValidatorResource;
use crate::ui::validators::min_validator::MinValidatorResource;
use crate::ui::validators::regex_validator::RegexValidatorResource;

#[derive(GodotConvert, Var, Export, Debug)]
#[godot(via = i64)]
pub enum ValidateResult
{
    Revert,
    Failed,
    Success
}

pub trait UIValueValidator
{
    fn validate_value(&self, value: &GString) -> ValidateResult;
}

pub trait ValidatorConstruct
{
    fn construct_validator(&self) -> Box<dyn UIValueValidator>;
}

#[derive(GodotClass)]
#[class(base = Resource, init)]
pub(crate) struct ValidatorsHolder
{
    base: Base<Resource>,
    validators_collection: Option<Vec<Box<dyn UIValueValidator>>>,

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
    #[func]
    pub fn validate_value(&self, value: GString) -> ValidateResult
    {
        if self.validators_collection.is_none() { return ValidateResult::Success; }

        let mut found_failed = false;

        for validator in self.validators_collection.as_ref().unwrap().iter()
        {
            let result = validator.validate_value(&value);

            match result
            {
                ValidateResult::Revert => {
                    return ValidateResult::Revert;
                }

                ValidateResult::Failed => { found_failed = true;}
                _ => {}
            }
        }

        if found_failed { return ValidateResult::Failed; }
        ValidateResult::Success
    }

    #[func]
    pub fn fetch_validators_from_resources(&mut self)
    {
        let min_validators: Vec<Box<dyn UIValueValidator>> = self.min_validator_constructors.iter_shared()
            .map(|min_construct| min_construct.bind().construct_validator()).collect();

        let max_validators: Vec<Box<dyn UIValueValidator>> = self.max_validator_constructors.iter_shared()
            .map(|max_validator| max_validator.bind().construct_validator()).collect();

        let regex_validators: Vec<Box<dyn UIValueValidator>> = self.regex_validator_constructors.iter_shared()
            .map(|regex_construct| regex_construct.bind().construct_validator()).collect();

        let mut validators_collection: Vec<Box<dyn UIValueValidator>> = vec![];
        validators_collection.extend(min_validators);
        validators_collection.extend(max_validators);
        validators_collection.extend(regex_validators);

        if validators_collection.is_empty() { return; }
        self.validators_collection = Some(validators_collection);
    }
}