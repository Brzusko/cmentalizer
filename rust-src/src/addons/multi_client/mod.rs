mod multi_client_window;

use godot::prelude::*;
use godot::classes::{Control, EditorPlugin, IEditorPlugin, PackedScene};
use godot::global::Error;
//tool - run in editor
//editor_plugin ???
//init - generate init method

static NODE_PATH: &str = "res://addons/multi_client/scenes/MultiClientWindow.tscn";

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base = EditorPlugin)]
struct MainMultiClientPlugin
{
    base: Base<EditorPlugin>,
    node_instance: Option<Gd<Control>>,
}

#[godot_api]
impl IEditorPlugin for MainMultiClientPlugin
{
    fn enter_tree(&mut self) {
        let packed_node = try_load::<PackedScene>(GString::from(NODE_PATH));
        let control_instance;
        
        match packed_node {
            Ok(packed_node_result) => {
                let node_candidate = packed_node_result.try_instantiate_as::<Control>();
                match node_candidate {
                    Some(node_instance) => { control_instance = node_instance; }
                    None => { godot_error!("Could not instantiate node as control"); return; }
                }
            }
            Err(error) => { godot_print!("Could not load node at path: {:?}", NODE_PATH); return; }
        }
        
        self.base_mut().add_control_to_bottom_panel(&control_instance, GString::from("MultiClient"));
        self.node_instance = Some(control_instance);
    }

    fn exit_tree(&mut self) {
        if self.node_instance.is_none() {
            return;
        }
        
        let control = self.node_instance.take().unwrap();
        self.base_mut().remove_control_from_bottom_panel(&control);
        control.free();
    }
}