extends Control

# Main entry point for OpenMMO client
# This script handles UI binding only - no business logic per AGENTS.md

@onready var login_button = $VBoxContainer/LoginPanel/LoginVBox/ButtonHBox/LoginButton
@onready var register_button = $VBoxContainer/LoginPanel/LoginVBox/ButtonHBox/RegisterButton
@onready var server_address = $VBoxContainer/LoginPanel/LoginVBox/ServerAddress
@onready var username = $VBoxContainer/LoginPanel/LoginVBox/Username
@onready var user_password = $VBoxContainer/LoginPanel/LoginVBox/Password
@onready var exit_button = $VBoxContainer/ExitButton
@onready var network_debug = $NetworkDebug

# Character selection UI
@onready var character_panel = $VBoxContainer/CharacterPanel
@onready var character_list = $VBoxContainer/CharacterPanel/CharacterVBox/CharacterList
@onready var select_character_button = $VBoxContainer/CharacterPanel/CharacterVBox/CharacterButtonHBox/SelectCharacterButton
@onready var create_character_button = $VBoxContainer/CharacterPanel/CharacterVBox/CharacterButtonHBox/CreateCharacterButton
@onready var back_to_login_button = $VBoxContainer/CharacterPanel/CharacterVBox/CharacterButtonHBox/BackToLoginButton

# Character creation UI
@onready var create_panel = $VBoxContainer/CreatePanel
@onready var character_name = $VBoxContainer/CreatePanel/CreateVBox/CharacterName
@onready var character_class = $VBoxContainer/CreatePanel/CreateVBox/CharacterClass
@onready var create_character_btn = $VBoxContainer/CreatePanel/CreateVBox/CreateButtonHBox/CreateCharacterBtn
@onready var cancel_create_button = $VBoxContainer/CreatePanel/CreateVBox/CreateButtonHBox/CancelCreateButton

# Loading and error UI
@onready var loading_panel = $VBoxContainer/LoadingPanel
@onready var error_label = $VBoxContainer/ErrorLabel

# Engine-agnostic modules
var client_networking = null
var game_state_manager = null
var movement_system = null
var input_manager = null
var ui_state_manager = null

# UI state
var ping_timer = null
var connection_timer = null
var auth_timer = null
var ui_fallback_timer = null
var current_screen = null

func _ready():
	print("DEBUG: Main._ready() called")
	# Initialize UI state
	print("OpenMMO Client Started")
	network_debug.set_status("Disconnected")

	# Set default server address
	server_address.text = "ws://localhost:8080/ws"

	# Initialize engine-agnostic modules
	_initialize_modules()

	# Connect UI signals (only if not already connected)
	if not login_button.is_connected("pressed", Callable(self, "_on_login_button_pressed")):
		login_button.connect("pressed", Callable(self, "_on_login_button_pressed"))
	if not register_button.is_connected("pressed", Callable(self, "_on_register_button_pressed")):
		register_button.connect("pressed", Callable(self, "_on_register_button_pressed"))
	if not select_character_button.is_connected("pressed", Callable(self, "_on_select_character_pressed")):
		select_character_button.connect("pressed", Callable(self, "_on_select_character_pressed"))
	if not create_character_button.is_connected("pressed", Callable(self, "_on_create_character_pressed")):
		create_character_button.connect("pressed", Callable(self, "_on_create_character_pressed"))
	if not back_to_login_button.is_connected("pressed", Callable(self, "_on_back_to_login_pressed")):
		back_to_login_button.connect("pressed", Callable(self, "_on_back_to_login_pressed"))
	if not create_character_btn.is_connected("pressed", Callable(self, "_on_create_character_confirm")):
		create_character_btn.connect("pressed", Callable(self, "_on_create_character_confirm"))
	if not cancel_create_button.is_connected("pressed", Callable(self, "_on_cancel_create_pressed")):
		cancel_create_button.connect("pressed", Callable(self, "_on_cancel_create_pressed"))

	# Initialize UI state
	_update_ui_for_state(ui_state_manager.get_current_state())

func _initialize_modules():
	# Load engine-agnostic modules
	client_networking = load("res://src/networking/client_networking.gd").new()
	game_state_manager = load("res://src/gamestate/game_state_manager.gd").new()
	movement_system = load("res://src/movement/movement_system.gd").new()
	input_manager = load("res://src/input/input_manager.gd").new()
	ui_state_manager = load("res://src/ui/ui_state_manager.gd").new()

	# Connect signals
	print("DEBUG: Connecting client networking signals")
	client_networking.connect("connected", Callable(self, "_on_network_connected"))
	client_networking.connect("disconnected", Callable(self, "_on_network_disconnected"))
	client_networking.connect("message_received", Callable(self, "_on_network_message_received"))
	client_networking.connect("connection_error", Callable(self, "_on_network_error"))
	client_networking.connect("auth_successful", Callable(self, "_on_auth_successful"))
	client_networking.connect("auth_failed", Callable(self, "_on_auth_failed"))
	client_networking.connect("character_list_received", Callable(self, "_on_character_list_received"))
	client_networking.connect("character_created", Callable(self, "_on_character_created"))
	client_networking.connect("character_selected", Callable(self, "_on_character_selected"))

	ui_state_manager.connect("state_changed", Callable(self, "_on_ui_state_changed"))
	print("DEBUG: Signal connections completed")

func _process(_delta):
	if client_networking:
		client_networking.poll()
		# Debug: Check connection state occasionally
		if Engine.get_process_frames() % 60 == 0:  # Every second
			var state = client_networking.get_connection_state()
			if state != 0:  # Not DISCONNECTED
				print("DEBUG: Client connection state:", state)

func _on_login_button_pressed():
	_perform_authentication(false)

func _on_register_button_pressed():
	_perform_authentication(true)

func _perform_authentication(is_register: bool):
	var address = server_address.text
	var user = username.text
	var passw = user_password.text

	if address.is_empty() or user.is_empty() or passw.is_empty():
		_show_error("Please fill in all fields")
		return

	print(("Register" if is_register else "Login") + " button pressed")
	print("Server: ", address)
	print("Username: ", user)

	# Clear any previous errors
	_clear_error()

	# Store credentials for later use
	ui_state_manager.set_last_username(user)

	# Initialize connection and attempt authentication
	_connect_and_authenticate(address, user, passw, is_register)

func _on_select_character_pressed():
	var selected_items = character_list.get_selected_items()
	if selected_items.is_empty():
		_show_error("Please select a character")
		return

	var selected_index = selected_items[0]
	var characters = ui_state_manager.get_available_characters()
	if selected_index >= characters.size():
		_show_error("Invalid character selection")
		return

	var selected_character = characters[selected_index]
	var character_id = selected_character.id

	_clear_error()
	ui_state_manager.go_to_loading()
	network_debug.add_message("Selecting character: " + selected_character.name)

	var error = client_networking.select_character(character_id)
	if error != OK:
		_show_error("Failed to select character")
		ui_state_manager.go_to_character_select()

func _on_create_character_pressed():
	ui_state_manager.go_to_character_create()

func _on_back_to_login_pressed():
	ui_state_manager.go_to_login()

func _on_create_character_confirm():
	var name = character_name.text.strip_edges()
	var class_option = character_class.get_selected_id()
	var selected_class_name = character_class.get_item_text(class_option)

	if name.is_empty():
		_show_error("Please enter a character name")
		return

	if name.length() < 3:
		_show_error("Character name must be at least 3 characters")
		return

	_clear_error()
	ui_state_manager.go_to_loading()
	network_debug.add_message("Creating character: " + name + " (" + selected_class_name + ")")

	var error = client_networking.create_character(name, selected_class_name)
	if error != OK:
		_show_error("Failed to create character")
		ui_state_manager.go_to_character_create()

func _on_cancel_create_pressed():
	character_name.text = ""
	character_class.selected = 0
	ui_state_manager.go_to_character_select()

# Timeout handlers
func _on_connection_timeout():
	_cleanup_timers()
	client_networking.close_connection()
	ui_state_manager.go_to_login()
	_show_error("Connection timeout - server may be unreachable")
	network_debug.add_message("Connection attempt timed out")

func _on_auth_timeout():
	_cleanup_timers()
	ui_state_manager.go_to_login()
	_show_error("Authentication timeout - server not responding")
	network_debug.add_message("Authentication request timed out")

func _on_ui_timeout():
	_cleanup_timers()
	ui_state_manager.go_to_login()
	_show_error("Operation timeout - please try again")
	network_debug.add_message("UI operation timed out")

# Timer cleanup utility
func _cleanup_timers():
	if connection_timer:
		connection_timer.stop()
		connection_timer.queue_free()
		connection_timer = null

	if auth_timer:
		auth_timer.stop()
		auth_timer.queue_free()
		auth_timer = null

	if ui_fallback_timer:
		ui_fallback_timer.stop()
		ui_fallback_timer.queue_free()
		ui_fallback_timer = null

	if ping_timer:
		ping_timer.stop()
		# Don't queue_free ping_timer as it might be reused

func _connect_and_authenticate(url: String, username: String, password: String, is_register: bool):
	ui_state_manager.go_to_loading()
	$VBoxContainer/LoadingPanel/LoadingVBox/LoadingLabel.text = "Connecting..."
	network_debug.set_status("Connecting...")
	network_debug.add_message("Connecting to: " + url)

	# Set up connection timeout (10 seconds)
	if connection_timer:
		connection_timer.stop()
	connection_timer = Timer.new()
	connection_timer.wait_time = 10.0
	connection_timer.connect("timeout", Callable(self, "_on_connection_timeout"))
	add_child(connection_timer)
	connection_timer.start()

	var error = client_networking.connect_to_server(url)
	if error != OK:
		_cleanup_timers()
		ui_state_manager.go_to_login()
		_show_error("Failed to connect: " + str(error))
		network_debug.set_status("Connection Failed")
		return

	# Set up ping timer
	if ping_timer:
		ping_timer.stop()
	ping_timer = Timer.new()
	ping_timer.wait_time = 5.0  # Ping every 5 seconds
	ping_timer.connect("timeout", Callable(self, "_send_ping"))
	add_child(ping_timer)
	ping_timer.start()

	# Store credentials for retry if needed
	_last_server_url = url
	_last_username = username
	_last_password = password
	_is_register_attempt = is_register

func _retry_login():
	if _last_server_url and _last_username and _last_password:
		_connect_and_authenticate(_last_server_url, _last_username, _last_password, false)

# Stored credentials for retry
var _last_server_url = ""
var _last_username = ""
var _last_password = ""
var _is_register_attempt = false

func _send_ping():
	if client_networking and client_networking.is_connection_active():
		var ping_payload = {
			"Ping": {
				"timestamp": int(Time.get_unix_time_from_system() * 1000)
			}
		}
		client_networking.send_message(ping_payload)
		network_debug.add_message("Sent ping")
	else:
		# Stop ping timer if not connected
		if ping_timer:
			ping_timer.stop()

# Network signal handlers
func _on_network_connected():
	print("DEBUG: _on_network_connected() called")
	# Stop connection timer since we connected successfully
	if connection_timer:
		connection_timer.stop()

	network_debug.set_status("Connected")
	network_debug.add_message("WebSocket connected successfully")

	# Set up authentication timeout (15 seconds)
	if auth_timer:
		auth_timer.stop()
	auth_timer = Timer.new()
	auth_timer.wait_time = 15.0
	auth_timer.connect("timeout", Callable(self, "_on_auth_timeout"))
	add_child(auth_timer)
	auth_timer.start()

	# Now that we're connected, send authentication request
	print("DEBUG: Sending authentication request, is_register:", _is_register_attempt)
	if _is_register_attempt:
		var error = client_networking.send_register_request(_last_username, _last_password)
		print("DEBUG: Register request sent, error:", error)
		if error != OK:
			_cleanup_timers()
			_show_error("Failed to send registration request")
			ui_state_manager.go_to_login()
	else:
		var error = client_networking.send_login_request(_last_username, _last_password)
		print("DEBUG: Login request sent, error:", error)
		if error != OK:
			_cleanup_timers()
			_show_error("Failed to send login request")
			ui_state_manager.go_to_login()

func _on_network_disconnected():
	_cleanup_timers()
	network_debug.set_status("Disconnected")
	network_debug.add_message("WebSocket disconnected")
	if ping_timer:
		ping_timer.stop()

func _on_network_message_received(message: Dictionary):
	network_debug.add_message("Received: " + JSON.stringify(message))

	# Handle specific message types
	if message.has("payload"):
		var payload = message.payload
		if payload.has("Pong"):
			network_debug.add_message("Received pong - connection healthy")
		elif payload.has("HandshakeResponse"):
			network_debug.add_message("Handshake completed")

func _on_network_error(error: String):
	_cleanup_timers()
	network_debug.add_message("Network error: " + error)
	network_debug.set_status("Error")
	ui_state_manager.set_error_message(error)

	# If we were in a loading state, go back to login
	if ui_state_manager.get_current_state() == ui_state_manager.UIState.LOADING:
		ui_state_manager.go_to_login()

# Authentication signal handlers
func _on_auth_successful(auth_data: Dictionary):
	_cleanup_timers()
	network_debug.add_message("Authentication successful")

	# Request character list
	client_networking.request_character_list()

func _on_auth_failed(reason: String):
	_cleanup_timers()
	network_debug.add_message("Authentication failed: " + reason)
	ui_state_manager.set_error_message(reason)
	ui_state_manager.go_to_login()

# Character management signal handlers
func _on_character_list_received(characters: Array):
	network_debug.add_message("Received " + str(characters.size()) + " characters")
	ui_state_manager.set_available_characters(characters)

	# Always go to character select state first
	ui_state_manager.go_to_character_select()

func _on_character_created(character_data: Dictionary):
	network_debug.add_message("Character created: " + character_data.get("name", "Unknown"))

	# Refresh character list first
	client_networking.request_character_list()
	
	# Then go to character select state (which will populate the list when response arrives)
	ui_state_manager.go_to_character_select()

func _on_character_selected(character_data: Dictionary):
	network_debug.add_message("Character selected: " + character_data.get("name", "Unknown"))
	ui_state_manager.go_to_connected()

	# Transition to game world
	_enter_game_world(character_data)

# UI state management
func _on_ui_state_changed(from_state, to_state):
	print("UI state changed: ", from_state, " -> ", to_state)
	_update_ui_for_state(to_state)

func _update_ui_for_state(state):
	# Hide all panels first
	var login_panel = $VBoxContainer/LoginPanel
	var character_panel = $VBoxContainer/CharacterPanel
	var create_panel = $VBoxContainer/CreatePanel
	var loading_panel = $VBoxContainer/LoadingPanel
	var error_label = $VBoxContainer/ErrorLabel

	login_panel.visible = false
	character_panel.visible = false
	create_panel.visible = false
	loading_panel.visible = false
	error_label.visible = false

	# Show appropriate panel based on state
	match state:
		ui_state_manager.UIState.LOGIN, ui_state_manager.UIState.REGISTER:
			login_panel.visible = true
			_clear_error()
		ui_state_manager.UIState.CHARACTER_SELECT:
			character_panel.visible = true
			_populate_character_list()
			_clear_error()
		ui_state_manager.UIState.CHARACTER_CREATE:
			create_panel.visible = true
			character_name.text = ""
			character_class.selected = 0
			_clear_error()
		ui_state_manager.UIState.LOADING:
			loading_panel.visible = true
			$VBoxContainer/LoadingPanel/LoadingVBox/LoadingLabel.text = "Connecting..."
			# Set up UI fallback timeout (20 seconds)
			if ui_fallback_timer:
				ui_fallback_timer.stop()
			ui_fallback_timer = Timer.new()
			ui_fallback_timer.wait_time = 20.0
			ui_fallback_timer.connect("timeout", Callable(self, "_on_ui_timeout"))
			add_child(ui_fallback_timer)
			ui_fallback_timer.start()
		ui_state_manager.UIState.CONNECTED:
			# Stop UI fallback timer on successful connection
			if ui_fallback_timer:
				ui_fallback_timer.stop()
				ui_fallback_timer.queue_free()
				ui_fallback_timer = null
			loading_panel.visible = true
			$VBoxContainer/LoadingPanel/LoadingVBox/LoadingLabel.text = "Entering game world..."

func _populate_character_list():
	character_list.clear()
	var characters = ui_state_manager.get_available_characters()

	for character in characters:
		var display_text = character.name + " (Lv." + str(character.level) + " " + character.class + ")"
		character_list.add_item(display_text)

	if characters.is_empty():
		character_list.add_item("No characters found - create one first")
		select_character_button.disabled = true
	else:
		select_character_button.disabled = false

func _show_error(message: String):
	error_label.text = "Error: " + message
	error_label.visible = true
	network_debug.add_message("Error: " + message)

func _clear_error():
	error_label.visible = false
	ui_state_manager.clear_error_message()

func _enter_game_world(character_data: Dictionary):
	network_debug.add_message("Entering game world...")
	# TODO: Transition to GameWorld scene
	# get_tree().change_scene_to_file("res://scenes/GameWorld.tscn")

func _on_exit_button_pressed():
	print("Exit button pressed")
	if client_networking:
		client_networking.close_connection()
	if ping_timer:
		ping_timer.stop()
	get_tree().quit()