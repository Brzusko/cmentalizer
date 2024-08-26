use godot::prelude::*;
use godot::classes::{LineEdit, Object, RegEx};

type Form = Gd<LineEdit>;
type Regex = Gd<RegEx>;

#[derive(GodotClass)]
#[class(base = Object, no_init)]
struct RegexLineEditWrapper 
{
    base: Base<Object>,
    line_edit: Form,
    reg_ex: Regex
}


impl RegexLineEditWrapper 
{
    pub fn construct(line_edit: Form, pattern: &str) -> Gd<RegexLineEditWrapper> 
    {
        Gd::from_init_fn(|base| {
            let reg_ex = RegEx::create_from_string(GString::from(pattern)).unwrap();
            let new_instance = Self {
                base,
                line_edit,
                reg_ex,
            };
            
            return new_instance;
        })
    }
    
    fn bind_events(&mut self)
    {
        let mut mutable_base = self.base_mut();
        let callback = mutable_base.callable("on_text_changed");
        mutable_base.connect(StringName::from("text_changed"), callback);
    }
}

#[godot_api]
impl RegexLineEditWrapper {
    fn on_text_changed(&mut self, new_value: GString)
    {
        godot_print!("Test");
    }
}