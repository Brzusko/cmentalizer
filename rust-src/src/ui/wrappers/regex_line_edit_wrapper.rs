use godot::prelude::*;
use godot::classes::{LineEdit, Object, RegEx};
use godot::private::callbacks;

#[derive(GodotClass)]
#[class(base = Object, no_init, tool)]
pub(crate) struct RegexLineEditWrapper 
{
    base: Base<Object>,
    line_edit: Option<Gd<LineEdit>>,
    reg_ex: Option<Gd<RegEx>>
}


impl RegexLineEditWrapper 
{
    pub fn construct(line_edit: Option<Gd<LineEdit>>, pattern: &str) -> Gd<RegexLineEditWrapper> 
    {
        Gd::from_init_fn(|base| {
            let reg_ex = RegEx::create_from_string(GString::from(pattern)).unwrap();
            let new_instance = Self {
                base,
                line_edit,
                reg_ex: Some(reg_ex),
            };
            
            return new_instance;
        })
    }
    
    pub fn bind_events(&mut self)
    {
        let callback;
        {
            let mut mutable_base = self.base_mut();
            callback = mutable_base.callable("on_text_changed");   
        }
        
        let mut line_edit = self.line_edit.take().unwrap();
        line_edit.connect(StringName::from("text_changed"), callback);
        self.line_edit = Some(line_edit);
    }
    
    pub fn get_text(&mut self) -> GString
    {
        let line_edit = self.line_edit.take().unwrap();
        let text = line_edit.get_text();
        self.line_edit = Some(line_edit);
        return text;
    }
}

#[godot_api]
impl RegexLineEditWrapper {
    #[func]
    fn on_text_changed(&mut self, new_value: GString)
    {
        godot_print!("Test");
    }
}