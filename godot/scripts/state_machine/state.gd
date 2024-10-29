extends Node;
class_name State;

@export var transition_states: Array[State];

func _enter() -> void:
	pass;

func _exit() -> void:
	pass;

func _next_state() -> State:
	return null;
