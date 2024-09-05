use std::sync::Mutex;
use godot::prelude::*;
use godot::classes::{Button, HBoxContainer, IHBoxContainer, LineEdit};
use crate::ui::wrappers::regex_line_edit_wrapper::RegexLineEditWrapper;

static REGEX_PATTERN: &str = r"^-?\d+$";

#[derive(GodotClass)]
#[class(base = HBoxContainer, init, tool)]
struct MainMultiClientUI
{
    base: Base<HBoxContainer>,
    #[export]
    clients_line_edit: Option<Gd<LineEdit>>,
    #[export]
    spawn_btn: Option<Gd<Button>>,
    regex_line_edit_wrapper: Option<Gd<RegexLineEditWrapper>>,
}

#[godot_api]
impl IHBoxContainer for MainMultiClientUI
{
    fn ready(&mut self)
    {
        let line_edit = self.clients_line_edit.take();
        let mut wrapper = RegexLineEditWrapper::construct(line_edit, REGEX_PATTERN);
        wrapper.bind_mut().bind_events();
        self.regex_line_edit_wrapper = Some(wrapper);
        let mut button = self.spawn_btn.take().unwrap();
        let callback = self.base().callable("on_btn_clicked");
        button.connect(StringName::from("pressed"), callback);
        self.spawn_btn = Some(button);
    }
}

#[godot_api]
impl MainMultiClientUI
{
    #[func]
    fn on_btn_clicked(&mut self)
    {
        todo!()
    }
}