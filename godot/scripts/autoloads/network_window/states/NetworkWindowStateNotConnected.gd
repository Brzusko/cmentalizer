extends State

@export var _windows_tab_controller: NetworkWindowTabsController;

func _enter() -> void:
	_windows_tab_controller._unlock();
	_windows_tab_controller._show_window_tab(0, true);

func _exit_tree() -> void:
	_windows_tab_controller._lock();
	_windows_tab_controller._hide_window_tabs();
