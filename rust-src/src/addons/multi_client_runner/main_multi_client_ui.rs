use godot::prelude::*;
use godot::classes::{Button, HBoxContainer, IHBoxContainer, LineEdit, Os};
use crate::addons::multi_client_runner::process_runner::{PlatformRunner, ProcessRunner};
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
    #[export]
    close_btn: Option<Gd<Button>>,
    regex_line_edit_wrapper: Option<Gd<RegexLineEditWrapper>>,
    process_runner: PlatformRunner,
}

#[godot_api]
impl IHBoxContainer for MainMultiClientUI
{
    fn ready(&mut self)
    {
        if self.clients_line_edit.is_none()
        {
            return;
        }
        
        let mut wrapper = RegexLineEditWrapper::construct(Some(self.clients_line_edit.as_ref().unwrap().clone()), REGEX_PATTERN);
        wrapper.bind_mut().bind_events();
        self.regex_line_edit_wrapper = Some(wrapper);

        {
            let spawn_btn_callback = self.base().callable("on_spawn_btn_clicked");
            let close_btn_callback = self.base().callable("on_close_btn_clicked");
            let close_button = self.close_btn.as_mut().unwrap();
            let spawn_button = self.spawn_btn.as_mut().unwrap();
            close_button.connect(StringName::from("pressed"), close_btn_callback);
            spawn_button.connect(StringName::from("pressed"), spawn_btn_callback);
        }
        
        self.process_runner = PlatformRunner::get_runner_for_platform();
    }

    fn exit_tree(&mut self) {
        if self.regex_line_edit_wrapper.is_none() { return; }

        let wrapper = self.regex_line_edit_wrapper.take().unwrap();
        wrapper.free();
        
        self.process_runner.kill_processes();
    }
}

#[godot_api]
impl MainMultiClientUI
{
    #[func]
    fn on_spawn_btn_clicked(&mut self)
    {
        if !self.process_runner.can_run() { return; }
        
        let clients_count = self.get_clients_count();
        
        if clients_count == 0 { return; }
        let os = Os::singleton();
        
        godot_print!("Spawning {:?} clients", clients_count);
        for _ in 0..clients_count 
        {
            self.process_runner.create_new_process(os.get_executable_path(), PackedStringArray::new());
        }
    }
    
    #[func]
    fn on_close_btn_clicked(&mut self)
    {
        self.process_runner.kill_processes();
    }
    
    fn get_clients_count(&self) -> i32
    {
        if self.regex_line_edit_wrapper.is_none() { return 0; }
        let text = self.regex_line_edit_wrapper.as_ref().unwrap().bind().get_text().to_string();

        return if let Ok(count) = text.parse::<i32>() { count }
        else { 0 }
    }
}