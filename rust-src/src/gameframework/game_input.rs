use godot::classes::{INode, InputEvent, InputEventMouseMotion, Node};
use godot::prelude::*;

struct InputFlags;

impl InputFlags {
    const PRIMARY_FLAG: u8 = 0b0001; // 1. bit
}

struct InputDecodeErrors;
impl InputDecodeErrors {
    const FLAG_ERROR: &str = "Float decoding failed";
    const BOOL_ERROR: &str = "Boolean decoding failed";
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct InputData {
    pub(crate) movement_input: Vector2,
    pub(crate) mouse_global_position: Vector2,
    pub(crate) mouse_screen_delta_position: Vector2,
    pub(crate) is_primary_pressed: bool,
}

impl InputData {
    fn encode_input_flags(&self) -> u8 {
        let mut flags: u8 = 0;

        if self.is_primary_pressed {
            flags |= InputFlags::PRIMARY_FLAG;
        }

        flags
    }

    pub(crate) fn decode_input_flags(&mut self, flags: u8) {
        self.is_primary_pressed = flags & InputFlags::PRIMARY_FLAG == 1;
    }
}

// Convert signal into Rust events implementataion if performance will be bad
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
        let mut offset: usize = 0;
        _ = bytes.encode_float(offset, self.movement_input.x);
        offset += float_offset;

        _ = bytes.encode_float(offset, self.movement_input.y);
        offset += float_offset;

        //mouse_screen_position
        _ = bytes.encode_float(offset, self.mouse_global_position.x);
        offset += float_offset;

        _ = bytes.encode_float(offset, self.mouse_global_position.y);
        offset += float_offset;

        //mouse_screen_delta_position
        _ = bytes.encode_float(offset, self.mouse_screen_delta_position.x);
        offset += float_offset;

        _ = bytes.encode_float(offset, self.mouse_screen_delta_position.y);
        offset += float_offset;

        //input flags
        let encoded_flags = self.encode_input_flags();
        _ = bytes.encode_u8(offset, encoded_flags);

        bytes
    }

    fn to_variant(&self) -> Variant {
        self.to_godot().to_variant()
    }
}

impl FromGodot for InputData {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        let bytes_needed = size_of::<InputData>();

        if bytes_needed != via.len() {
            return Err(ConvertError::new(
                "Provided variable to convert from has incorrect size",
            ));
        }

        let float_offset = size_of::<f32>();
        let mut offset: usize = 0;

        let movement_x = via
            .decode_float(offset)
            .map_err(|_| ConvertError::new(InputDecodeErrors::FLAG_ERROR))?;
        offset += float_offset;

        let movement_y = via
            .decode_float(offset)
            .map_err(|_| ConvertError::new(InputDecodeErrors::FLAG_ERROR))?;
        offset += float_offset;

        let mouse_x = via
            .decode_float(offset)
            .map_err(|_| ConvertError::new(InputDecodeErrors::FLAG_ERROR))?;
        offset += float_offset;

        let mouse_y = via
            .decode_float(offset)
            .map_err(|_| ConvertError::new(InputDecodeErrors::FLAG_ERROR))?;
        offset += float_offset;

        let mouse_delta_x = via
            .decode_float(offset)
            .map_err(|_| ConvertError::new(InputDecodeErrors::FLAG_ERROR))?;
        offset += float_offset;

        let mouse_delta_y = via
            .decode_float(offset)
            .map_err(|_| ConvertError::new(InputDecodeErrors::FLAG_ERROR))?;
        offset += float_offset;

        let input_flags = via
            .decode_u8(offset)
            .map_err(|_| ConvertError::new(InputDecodeErrors::BOOL_ERROR))?;

        let mut input_data = Self {
            movement_input: Vector2::new(movement_x, movement_y),
            mouse_global_position: Vector2::new(mouse_x, mouse_y),
            mouse_screen_delta_position: Vector2::new(mouse_delta_x, mouse_delta_y),
            is_primary_pressed: false,
        };

        input_data.decode_input_flags(input_flags);

        Ok(input_data)
    }

    fn from_godot(via: Self::Via) -> Self {
        let float_offset = size_of::<f32>();
        let mut offset: usize = 0;

        let movement_x = via.decode_float(offset).unwrap_or_default();
        offset += float_offset;

        let movement_y = via.decode_float(offset).unwrap_or_default();
        offset += float_offset;

        let mouse_x = via.decode_float(offset).unwrap_or_default();
        offset += float_offset;

        let mouse_y = via.decode_float(offset).unwrap_or_default();
        offset += float_offset;

        let mouse_delta_x = via.decode_float(offset).unwrap_or_default();
        offset += float_offset;

        let mouse_delta_y = via.decode_float(offset).unwrap_or_default();
        offset += float_offset;

        let input_flags = via.decode_u8(offset).unwrap_or_default();

        let mut input_data = Self {
            movement_input: Vector2::new(movement_x, movement_y),
            mouse_global_position: Vector2::new(mouse_x, mouse_y),
            mouse_screen_delta_position: Vector2::new(mouse_delta_x, mouse_delta_y),
            is_primary_pressed: false,
        };

        input_data.decode_input_flags(input_flags);
        input_data
    }
    fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError> {
        let byte_array_convert = variant.try_to::<PackedByteArray>()?;
        FromGodot::try_from_godot(byte_array_convert)
    }
    fn from_variant(variant: &Variant) -> Self {
        let byte_array_convert_result = variant.try_to::<PackedByteArray>();

        match byte_array_convert_result {
            Ok(byte_array) => FromGodot::from_godot(byte_array),
            Err(_) => Self::default(),
        }
    }
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct InputProvider {
    base: Base<Node>,

    #[export]
    up_input_name: StringName,
    #[export]
    down_input_name: StringName,
    #[export]
    left_input_name: StringName,
    #[export]
    right_input_name: StringName,
    #[export]
    exit_input_name: StringName,
    #[export]
    primary_input_name: StringName,

    //Maybe later we should improve pointer movement fetching
    mouse_position_space: Option<Gd<Node2D>>,

    cached_input_data: InputData,
}

#[godot_api]
impl INode for InputProvider {
    fn ready(&mut self) {
        let mut mouse_space = Node2D::new_alloc();
        mouse_space.set_name("MouseSpace");

        {
            self.base_mut().add_child(&mouse_space);
        }

        self.mouse_position_space = Some(mouse_space);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.is_movement_action(&event) {
            let movement_vector = Input::singleton().get_vector(
                &self.left_input_name,
                &self.get_right_input_name(),
                &self.up_input_name,
                &self.down_input_name,
            );

            self.cached_input_data.movement_input = movement_vector;

            self.trigger_input_signal();
        }

        let mouse_motion = self.get_mouse_movement_action(&event);

        if mouse_motion.is_some() {
            let mouse_motion_action = mouse_motion.unwrap();
            let mouse_space = self.mouse_position_space.as_ref().unwrap();
            let mouse_global_position = mouse_space.get_global_mouse_position();
            let mouse_delta_position = mouse_motion_action.get_relative();

            self.cached_input_data.mouse_global_position = mouse_global_position;
            self.cached_input_data.mouse_screen_delta_position = mouse_delta_position;

            self.trigger_input_signal();
        }

        if event.is_action(&self.primary_input_name) {
            self.cached_input_data.is_primary_pressed =
                event.is_action_pressed(&self.primary_input_name);

            self.trigger_input_signal();
        }
    }
}

#[godot_api]
impl InputProvider {
    #[signal]
    fn input_changed(input_data: InputData) {}
    #[signal]
    fn exit_pressed() {}
    // TODO convert to proc macro is_action(event, expression)
    fn is_movement_action(&self, event: &Gd<InputEvent>) -> bool {
        event.is_action(&self.up_input_name)
            || event.is_action(&self.down_input_name)
            || event.is_action(&self.left_input_name)
            || event.is_action(&self.right_input_name)
    }

    fn get_mouse_movement_action(
        &self,
        event: &Gd<InputEvent>,
    ) -> Option<Gd<InputEventMouseMotion>> {
        let cast_result = event.clone().try_cast::<InputEventMouseMotion>();

        match cast_result {
            Ok(mouse_motion) => Some(mouse_motion),
            Err(_) => None,
        }
    }

    fn trigger_input_signal(&mut self) {
        let signal_arg = self.cached_input_data.to_variant();
        self.base_mut().emit_signal("input_changed", &[signal_arg]);
    }
}
