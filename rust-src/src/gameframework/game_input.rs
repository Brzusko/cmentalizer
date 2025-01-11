use godot::classes::{INode, InputEvent, Node};
use godot::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct InputData {
    pub(crate) movement_input: Vector2,
    pub(crate) mouse_screen_position: Vector2,
    pub(crate) mouse_screen_delta_position: Vector2,
}

// TODO simplify serialization/deserialization
impl GodotConvert for InputData {
    type Via = PackedByteArray;
}

impl ToGodot for InputData {
    type ToVia<'v> = PackedByteArray;

    fn to_godot(&self) -> Self::ToVia<'_> {
        let mut bytes = PackedByteArray::new();
        let bytes_to_reserve = size_of::<InputData>();
        bytes.resize(bytes_to_reserve);
        let float_offset = size_of::<f32>();

        //encoding should not return Error because, PackedByteArray has enough space
        //movement_input
        _ = bytes.encode_float(0, self.movement_input.x); // 0 - 3 # 4
        _ = bytes.encode_float(float_offset, self.movement_input.y); // 4 - 7 #

        //mouse_screen_position
        _ = bytes.encode_float(float_offset * 2, self.mouse_screen_position.x); // 8 - 11
        _ = bytes.encode_float(float_offset * 3, self.mouse_screen_position.y); // 12 - 15

        //mouse_screen_delta_position
        _ = bytes.encode_float(float_offset * 4, self.mouse_screen_delta_position.x); // 16 - 19
        _ = bytes.encode_float(float_offset * 5, self.mouse_screen_delta_position.y); // 20 - 23

        bytes
    }
}

impl FromGodot for InputData {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        let bytes_needed = size_of::<InputData>();

        godot_print!("{:?}", bytes_needed);
        godot_print!("{:?}", via.len());

        if bytes_needed != via.len() {
            return Err(ConvertError::new(
                "Provided variable to convert from has incorrect size",
            ));
        }

        let float_offset = size_of::<f32>();

        let movement_x = via.decode_float(0).unwrap();
        let movement_y = via.decode_float(float_offset).unwrap();

        let mouse_x = via.decode_float(float_offset * 2).unwrap();
        let mouse_y = via.decode_float(float_offset * 3).unwrap();

        let mouse_delta_x = via.decode_float(float_offset * 4).unwrap();
        let mouse_delta_y = via.decode_float(float_offset * 5).unwrap();

        Ok(Self {
            movement_input: Vector2::new(movement_x, movement_y),
            mouse_screen_position: Vector2::new(mouse_x, mouse_y),
            mouse_screen_delta_position: Vector2::new(mouse_delta_x, mouse_delta_y),
        })
    }
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct InputProvider {
    #[export]
    up_input_name: StringName,
    #[export]
    down_input_name: StringName,
    #[export]
    left_input_name: StringName,
    #[export]
    right_input_name: StringName,

    cached_input_data: InputData,
}

#[godot_api]
impl INode for InputProvider {
    fn input(&mut self, event: Gd<InputEvent>) {
        if self.is_movement_action(&event) {
            self.trigger_input_signal();
        }
    }
}

#[godot_api]
impl InputProvider {
    #[signal]
    fn input_changed(input_data: InputData) {}
    // TODO convert to proc macro is_action(event, expression)
    fn is_movement_action(&self, event: &Gd<InputEvent>) -> bool {
        event.is_action(&self.up_input_name)
            || event.is_action(&self.down_input_name)
            || event.is_action(&self.left_input_name)
            || event.is_action(&self.right_input_name)
    }

    fn trigger_input_signal(&self) {
        todo!()
    }
}
