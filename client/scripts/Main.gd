extends Control

# Main entry point for OpenMMO client
# This script handles UI binding only - no business logic per AGENTS.md

@onready var connect_button = $VBoxContainer/ConnectButton
@onready var server_address = $VBoxContainer/ServerAddress
@onready var username = $VBoxContainer/Username
@onready var user_password = $VBoxContainer/Password
@onready var exit_button = $VBoxContainer/ExitButton
@onready var network_debug = $NetworkDebug

# Engine-agnostic modules
var client_networking = null
var game_state_manager = null
var movement_system = null
var input_manager = null

# UI state
var ping_timer = null

func _ready():
	# Initialize UI state
	print("OpenMMO Client Started")
	network_debug.set_status("Disconnected")

	# Set default server address
	server_address.text = "ws://localhost:8080/ws"

	# Initialize engine-agnostic modules
	_initialize_modules()

func _initialize_modules():
	# Load engine-agnostic modules
	client_networking = load("res://src/networking/client_networking.gd").new()
	game_state_manager = load("res://src/gamestate/game_state_manager.gd").new()
	movement_system = load("res://src/movement/movement_system.gd").new()
	input_manager = load("res://src/input/input_manager.gd").new()

	# Connect signals
	client_networking.connect("connected", Callable(self, "_on_network_connected"))
	client_networking.connect("disconnected", Callable(self, "_on_network_disconnected"))
	client_networking.connect("message_received", Callable(self, "_on_network_message_received"))
	client_networking.connect("connection_error", Callable(self, "_on_network_error"))

func _on_connect_button_pressed():
	# UI binding only - actual connection logic will be handled by engine-agnostic modules
	var address = server_address.text
	var user = username.text
	var passw = user_password.text

	if address.is_empty() or user.is_empty() or passw.is_empty():
		print("Please fill in all connection fields")
		network_debug.add_message("Error: Please fill in all fields")
		return

	print("Connect button pressed - UI binding only")
	print("Server: ", address)
	print("Username: ", user)

	# Initialize connection using networking module
	_connect_to_server(address)

func _connect_to_server(url):
	if client_networking and client_networking.is_connected():
		client_networking.disconnect()

	network_debug.set_status("Connecting...")
	network_debug.add_message("Connecting to: " + url)

	var error = client_networking.connect_to_server(url)
	if error != OK:
		network_debug.add_message("Failed to connect: " + str(error))
		network_debug.set_status("Connection Failed")
		return

	# Set up ping timer
	if ping_timer != null:
		ping_timer.stop()
	ping_timer = Timer.new()
	ping_timer.wait_time = 5.0  # Ping every 5 seconds
	ping_timer.connect("timeout", Callable(self, "_send_ping"))
	add_child(ping_timer)
	ping_timer.start()

func _send_ping():
	if client_networking and client_networking.is_connection_active():
		var ping_payload = {
			"Ping": {
				"timestamp": Time.get_unix_time_from_system() * 1000
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
	network_debug.set_status("Connected")
	network_debug.add_message("WebSocket connected successfully")

func _on_network_disconnected():
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
	network_debug.add_message("Network error: " + error)
	network_debug.set_status("Error")

func _on_exit_button_pressed():
	print("Exit button pressed")
	if client_networking:
		client_networking.close_connection()
	if ping_timer:
		ping_timer.stop()
	get_tree().quit()