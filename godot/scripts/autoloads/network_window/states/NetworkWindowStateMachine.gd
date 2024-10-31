extends StateMachine

@export var _starting_state: State;

func _ready() -> void:
	_transition_to_state(_starting_state);
	_connect_transition_signal();
	
	
func _exit_tree() -> void:
	_disconnect_transition_signal();
