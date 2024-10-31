extends Node;
class_name StateMachine;

@export var _states: Array[State];

var _current_state: State = null;
var _is_transition_signal_connected: bool = false;

func _connect_transition_signal() -> void:
	if _is_transition_signal_connected:
		return;
	
	for state: State in _states:
		state.TransitionRequest.connect(_on_trasition_request);
	
	_is_transition_signal_connected = true;


func _disconnect_transition_signal() -> void:
	if !_is_transition_signal_connected:
		return;
		
	for state: State in _states:
		state.TransitionRequest.disconnect(_on_trasition_request);
		
	_is_transition_signal_connected = false;

func _transition_to_state(next_state: State) -> void:
	if _current_state != null:
		_current_state._exit();
	
	_current_state = next_state;
	_current_state._enter();

func _on_trasition_request(transition_state: State) -> void:
	_transition_to_state(transition_state);
