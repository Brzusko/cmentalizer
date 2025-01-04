use godot::prelude::*;
use godot::classes::{Resource, RegEx};
use crate::ui::validators::{UIValueValidator, ValidationResponse};

#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
pub(crate) struct RegexValidatorResource
{
    base: Base<Resource>,
    #[export]
    pattern: GString,
    #[export]
    error_key: GString,
    #[export]
    revert: bool,
    regex: Option<Gd<RegEx>>,
}

#[godot_dyn]
impl UIValueValidator for RegexValidatorResource 
{
    fn validate_value(&mut self, value: &GString) -> anyhow::Result<(), ValidationResponse>
    {
        if self.regex.is_none() { self.regex = Some(RegEx::create_from_string(&self.pattern).unwrap()); }

        let regex = self.regex.as_ref().unwrap();
        let string_search = regex.search(value);

        match string_search
        {
            Some(_) => {Ok(())},
            None => {
                let error_key = self.error_key.clone();
                if self.revert { return Err(ValidationResponse::Revert(error_key)) }
                Err(ValidationResponse::Pass(error_key))
            }
        }
    }
}
