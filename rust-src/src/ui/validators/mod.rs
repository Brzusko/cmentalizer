use godot::builtin::GString;

pub(crate) enum ValidationResponse
{
    Pass(GString),
    Revert(GString),
}

pub trait UIValueValidator
{
    fn validate_value(&mut self, value: &GString) -> anyhow::Result<(), ValidationResponse>;
}


pub (crate) mod validators_holder;
pub (crate) mod regex_validator;
pub (crate) mod min_validator;
pub (crate) mod max_validator;
pub (crate) mod validator_consumers;