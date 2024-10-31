extends Node
class_name NetworkWindowTabsController;

@export var _tabs_bar: TabBar = null;
@export var _tab_windows: Array[Control];

var _are_tabs_locked: bool = false;
var _current_window_selected_index: int = -1;

func _ready() -> void:
	_tabs_bar.tab_selected.connect(_on_tab_selected);
	
func _on_tab_selected(index: int) -> void:
	_show_window_tab(index, true);

func _lock() -> void:
	if _are_tabs_locked:
		return;
	
	_are_tabs_locked = true;
	_set_tabs_status(_are_tabs_locked);

func _unlock() -> void:
	if !_are_tabs_locked:
		return;
	
	_are_tabs_locked = false;
	_set_tabs_status(_are_tabs_locked);
	
func _set_tabs_status(status: bool) -> void:
	for i: int in _tabs_bar.tab_count:
		_tabs_bar.set_tab_disabled(i, status);

func _show_current_window_tab() -> void:
	_show_window_tab(_current_window_selected_index, true);

func _hide_window_tabs() -> void:
	_current_window_selected_index = -1;
	for window: Control in _tab_windows:
		_show_concrete_window(window, false);
		
func _show_window_tab(index: int, show: bool) -> void:
	var window_count: int = _tab_windows.size();
	
	if index > window_count:
		return;
	
	if _current_window_selected_index == index:
		_show_concrete_window(_tab_windows[_current_window_selected_index], show);
		return;
	
	if _current_window_selected_index != -1:
		_show_concrete_window(_tab_windows[_current_window_selected_index], false);
	
	_current_window_selected_index = index;
	_show_concrete_window(_tab_windows[_current_window_selected_index], show);
	
		
func _show_concrete_window(window: Control, show: bool) -> void:
	if window.visible == show:
		return;
	
	if show:
		window.show();
	else:
		window.hide();
