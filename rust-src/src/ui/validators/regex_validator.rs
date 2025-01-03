use godot::prelude::*;
use godot::classes::{Resource, IResource, RegEx, RegExMatch};
use crate::ui::validators::validators_holder::{UIValueValidator};

#[derive(GodotClass)]
#[class(base = Resource, init, tool)]
pub(crate) struct RegexValidatorResource
{
    base: Base<Resource>,
    #[export]
    pattern: GString,
    regex: Option<Gd<RegEx>>,
}

#[godot_dyn]
impl UIValueValidator for RegexValidatorResource 
{
    fn validate_value(&self, value: &GString) -> anyhow::Result<(), GString> 
    {
        if self.regex.is_none() {  }
        todo!()
    }
}
