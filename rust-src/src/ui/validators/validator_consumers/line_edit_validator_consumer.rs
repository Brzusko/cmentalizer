use godot::prelude::*;
use godot::classes::{INode, LineEdit, Node};
use crate::ui::validators::validators_holder::{ValidateResult, ValidatorsHolder};

#[derive(GodotClass)]
#[class(base = Node, init, tool)]
pub(crate) struct LineEditValidatorConsumer
{
    base: Base<Node>,
    #[export]
    validators_holder: Option<Gd<ValidatorsHolder>>,
    #[export]
    line_edit: Option<Gd<LineEdit>>,
    cached_value: GString,
    caret_column: i32,
}

#[godot_api]
impl INode for LineEditValidatorConsumer
{
    fn ready(&mut self)
    {
        // if self.validators_holder.is_none() || self.line_edit.is_none() { return; }
        // { self.validators_holder.as_mut().unwrap().bind_mut().fetch_validators_from_resources(); }
        //
        // {
        //     let base = self.base();
        //     let on_text_changed_delegate = base.callable(&StringName::from("on_text_changed"));
        //     let line_edit_mut = self.line_edit.as_mut().unwrap();
        //
        //     line_edit_mut.connect(&StringName::from("text_changed"), &on_text_changed_delegate);
        // }
    }
}

#[godot_api]
impl LineEditValidatorConsumer
{
    #[func]
    pub fn on_text_changed(&mut self, new_text: GString)
    {
        if self.validators_holder.is_none() || self.line_edit.is_none() { return; }

        let validators_holder = self.validators_holder.as_ref().unwrap().bind();
        let result = validators_holder.validate_value(new_text.clone());
        let mut line_edit = self.line_edit.as_mut().unwrap();

        match result {
            ValidateResult::Revert => { line_edit.set_text(&self.cached_value); line_edit.set_caret_column(self.caret_column) }
            ValidateResult::Failed => { self.caret_column = line_edit.get_caret_column(); }
            ValidateResult::Success => { self.cached_value = new_text; self.caret_column = line_edit.get_caret_column() }
        }
    }
}