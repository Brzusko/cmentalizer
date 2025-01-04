use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use godot::prelude::*;
use godot::classes::{INode, LineEdit, Node};
use log::error;
use crate::ui::validators::ValidationResponse;
use crate::ui::validators::validators_holder::{ValidatorsHolder};

#[derive(GodotClass)]
#[class(base = Node, init, tool)]
pub(crate) struct LineEditValidatorConsumer
{
    base: Base<Node>,
    #[export]
    validators_holder: Option<Gd<ValidatorsHolder>>,
    #[export]
    line_edit: Option<Gd<LineEdit>>,
    on_text_changed_callable: Option<Callable>,
    cached_value: GString,
    caret_column: i32,
}

#[godot_api]
impl INode for LineEditValidatorConsumer
{
    fn ready(&mut self)
    {
        if self.validators_holder.is_none() || self.line_edit.is_none() { return; }

        {
            let validators_holder = self.validators_holder.as_mut().unwrap();
            validators_holder.bind_mut().setup_validators();
        }

        {
            let callable = self.base().callable("on_text_changed");
            self.line_edit.as_mut().unwrap().connect("text_changed", &callable);
            self.on_text_changed_callable = Some(callable);
        }
    }
}

#[godot_api]
impl LineEditValidatorConsumer
{
    #[func]
    pub fn on_text_changed(&mut self, new_text: GString)
    {
        if self.validators_holder.is_none() { return; }
        let validation_result;
        {
            let mut validators_holder = self.validators_holder.as_mut().unwrap().bind_mut();
            validation_result = validators_holder.validate(&new_text);
        }

        match validation_result
        {
            Ok(()) => {}
            Err(errors) => {
                for error in errors.iter()
                {
                    match error {
                        // For now, ignore pass error
                        ValidationResponse::Pass(_) => {}
                        ValidationResponse::Revert(_) => {
                            self.revert();
                            return;
                        }
                    }
                }
            }
        }

        self.pass(new_text);
    }

    fn pass(&mut self, text: GString)
    {
        self.cached_value = text;
        self.caret_column = self.line_edit.as_ref().unwrap().get_caret_column();
    }

    fn revert(&mut self)
    {
        let line_edit = self.line_edit.as_mut().unwrap();
        line_edit.set_text(&self.cached_value);
        line_edit.set_caret_column(self.caret_column);
    }
}