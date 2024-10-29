extends Node;
class_name StateMachine;

var _current_state: State = null;

func _check_transition() -> bool:
	return false;
