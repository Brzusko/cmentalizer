extends Node;
class_name State;

signal TransitionRequest(transition_state: State);

@export var transition_states: Array[State];

func _enter() -> void:
	pass;

func _exit() -> void:
	pass;
