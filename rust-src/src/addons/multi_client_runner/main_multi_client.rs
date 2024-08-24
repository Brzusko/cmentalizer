use godot::prelude::*;
use godot::classes::{Control, EditorPlugin, IEditorPlugin};

static CONTROL_SCENE_PATH: &str = "res://addons/multi_client/scenes/MultiClientControl.tscn";

#[derive(GodotClass)]
#[class(base = EditorPlugin, editor_plugin, init, tool)]
struct MultiClientRunnerMain
{
    base: Base<EditorPlugin>,
    main_control: Option<Gd<Control>>
}

#[godot_api]
impl IEditorPlugin for MultiClientRunnerMain
{
    fn enter_tree(&mut self) {
        let packed_scene_result = try_load::<PackedScene>(GString::from(CONTROL_SCENE_PATH));
        let instance: Option<Gd<Control>>;
        
        match packed_scene_result 
        {
            Ok(scene) => { instance = scene.try_instantiate_as::<Control>(); }
            Err(error) => { godot_print!("Could not create multi-client-window"); return; }
        }
        
        let control = instance.unwrap();
        self.base_mut().add_control_to_bottom_panel(&control, GString::from("Multi Client"));
        
        self.main_control = Some(control);
    }

    fn exit_tree(&mut self) {
        if self.main_control.is_none()
        {
            return;
        }
        
        let mut control = self.main_control.take().unwrap();
        self.base_mut().remove_control_from_bottom_panel(&control);
        let nil_variant = [Variant::nil()];
        control.call_deferred(StringName::from("queue_free"), &nil_variant);
    }
}