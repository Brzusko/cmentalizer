use godot::prelude::*;
use godot::classes::{Resource, IResource, RegEx, RegExMatch};
use crate::ui::validators::validators_holder::{UIValueValidator, ValidateResult, ValidatorConstruct};

#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
pub(crate) struct RegexValidatorResource
{
    base: Base<Resource>,
    #[export]
    pattern: GString,
}

impl ValidatorConstruct for RegexValidatorResource
{
    fn construct_validator(&self) -> Box<dyn UIValueValidator>
    {
        Box::new(RegexValidator::new(&self.pattern))
    }
}

struct RegexValidator
{
    regex: Gd<RegEx>,
}

impl RegexValidator
{
    fn new(pattern: &GString) -> Self
    {
        let regex = RegEx::create_from_string(pattern);
        if regex.is_none()
        {
            panic!();
        }

        Self {
            regex: regex.unwrap(),
        }
    }
}

impl UIValueValidator for RegexValidator
{
    fn validate_value(&self, value: &GString) -> ValidateResult
    {
        let search = self.regex.search(value);
        if search.is_none() { return ValidateResult::Revert }
        ValidateResult::Success
    }
}