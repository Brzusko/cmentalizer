extends MarginContainer

var _is_enabled: bool = false: get = _get_is_enabled, set =_set_is_enabled;

func _ready() -> void:
	_is_enabled = false;

func _get_is_enabled() -> bool:
	return _is_enabled;

func _set_is_enabled(new_value: bool) -> void:
	_is_enabled = new_value;
	
	if _is_enabled:
		show();
	else:
		hide();

func _input(event: InputEvent) -> void:
	if event.is_action_pressed("network_debug_ui_enable"):
		_is_enabled = !_is_enabled;
