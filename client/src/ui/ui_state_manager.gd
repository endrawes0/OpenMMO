# OpenMMO Client UI State Manager
# Manages different UI screens and transitions between them
# Engine-agnostic UI state logic

class_name UIStateManager
extends RefCounted

# UI States
enum UIState {
	LOGIN,
	REGISTER,
	CHARACTER_SELECT,
	CHARACTER_CREATE,
	LOADING,
	CONNECTED
}

# Signals
signal state_changed(from_state: UIState, to_state: UIState)
signal login_successful(username: String)
signal character_selected(character_id: int, character_name: String)
signal character_created(character_data: Dictionary)

# Current state
var current_state: UIState = UIState.LOGIN
var previous_state: UIState = UIState.LOGIN

# UI data
var available_characters: Array = []
var selected_character_id: int = 0
var last_username: String = ""
var last_server_address: String = ""
var last_error_message: String = ""

# File paths for persistence
const CONFIG_FILE = "user://connection_config.json"

func _init():
	_load_connection_config()

func change_state(new_state: UIState):
	if new_state == current_state:
		return

	previous_state = current_state
	current_state = new_state
	emit_signal("state_changed", previous_state, current_state)

func get_current_state() -> UIState:
	return current_state

func get_previous_state() -> UIState:
	return previous_state

func is_in_login_flow() -> bool:
	return current_state in [UIState.LOGIN, UIState.REGISTER]

func is_in_character_flow() -> bool:
	return current_state in [UIState.CHARACTER_SELECT, UIState.CHARACTER_CREATE]

func set_available_characters(characters: Array):
	available_characters = characters

func get_available_characters() -> Array:
	return available_characters

func set_selected_character(character_id: int):
	selected_character_id = character_id

func get_selected_character() -> Dictionary:
	for character in available_characters:
		if character.id == selected_character_id:
			return character
	return {}

func set_last_username(username: String):
	last_username = username
	_save_connection_config()

func get_last_username() -> String:
	return last_username

func set_last_server_address(address: String):
	last_server_address = address
	_save_connection_config()

func get_last_server_address() -> String:
	return last_server_address

func set_error_message(message: String):
	last_error_message = message

func get_error_message() -> String:
	return last_error_message

func clear_error_message():
	last_error_message = ""

# Configuration persistence
func _load_connection_config():
	if FileAccess.file_exists(CONFIG_FILE):
		var file = FileAccess.open(CONFIG_FILE, FileAccess.READ)
		if file:
			var json_text = file.get_as_text()
			file.close()
			
			var json = JSON.new()
			var parse_result = json.parse(json_text)
			if parse_result == OK:
				var data = json.data
				if data.has("username"):
					last_username = data.username
				if data.has("server_address"):
					last_server_address = data.server_address

func _save_connection_config():
	var data = {
		"username": last_username,
		"server_address": last_server_address
	}
	
	var file = FileAccess.open(CONFIG_FILE, FileAccess.WRITE)
	if file:
		file.store_string(JSON.stringify(data))
		file.close()

# State transition helpers
func go_to_login():
	change_state(UIState.LOGIN)

func go_to_register():
	change_state(UIState.REGISTER)

func go_to_character_select():
	change_state(UIState.CHARACTER_SELECT)

func go_to_character_create():
	change_state(UIState.CHARACTER_CREATE)

func go_to_loading():
	change_state(UIState.LOADING)

func go_to_connected():
	change_state(UIState.CONNECTED)

func go_back():
	if previous_state != current_state:
		change_state(previous_state)
