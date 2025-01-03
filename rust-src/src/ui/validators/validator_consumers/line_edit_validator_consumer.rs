use godot::prelude::*;
use godot::classes::{INode, LineEdit, Node};
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
    }
}