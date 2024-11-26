use godot::prelude::*;
use godot::classes::{INode, Node};

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
#[class(base = Node, init)]
struct ValidatorsHolder
{
    base: Base<Node>,
    validators_collection: Option<Box<dyn UIValueValidator>>,

    // arrays of validators
}

#[godot_api]
impl INode for ValidatorsHolder
{
    fn ready(&mut self)
    {
        self.fetch_validators_from_resources();
    }
}

#[godot_api]
impl ValidatorsHolder
{
    #[func]
    pub fn validate_value(&self, value: GString) -> ValidateResult
    {
        todo!()
    }

    fn fetch_validators_from_resources(&mut self)
    {
        todo!()
    }
}