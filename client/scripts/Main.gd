extends Control

# Main entry point for OpenMMO client
# This script handles UI binding only - no business logic per AGENTS.md

@onready var connect_button = $VBoxContainer/ConnectButton
@onready var server_address = $VBoxContainer/ServerAddress
@onready var username = $VBoxContainer/Username
@onready var user_password = $VBoxContainer/Password
@onready var exit_button = $VBoxContainer/ExitButton
@onready var network_debug = $NetworkDebug

# WebSocket connection
var websocket = null
var ping_timer = null
var sequence_id = 0

func _ready():
	# Initialize UI state
	print("OpenMMO Client Started")
	network_debug.set_status("Disconnected")

	# Set default server address
	server_address.text = "ws://localhost:8080/ws"

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

	# Initialize WebSocket connection
	_connect_to_server(address)

func _connect_to_server(url):
	if websocket != null:
		websocket.close()

	network_debug.set_status("Connecting...")
	network_debug.add_message("Connecting to: " + url)

	websocket = WebSocketPeer.new()
	var error = websocket.connect_to_url(url)

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
	if websocket.get_ready_state() == WebSocketPeer.STATE_OPEN:
		var ping_data = {
			"sequence_id": sequence_id,
			"timestamp": Time.get_unix_time_from_system() * 1000,
			"payload": {
				"Ping": {
					"timestamp": Time.get_unix_time_from_system() * 1000
				}
			}
		}
		sequence_id += 1

		var json_string = JSON.stringify(ping_data)
		websocket.send_text(json_string)
		network_debug.add_message("Sent ping")

func _process(delta):
	if websocket != null:
		websocket.poll()

		var state = websocket.get_ready_state()
		match state:
			WebSocketPeer.STATE_OPEN:
				if network_debug.get_status() != "Connected":
					network_debug.set_status("Connected")
					network_debug.add_message("WebSocket connected successfully")
			WebSocketPeer.STATE_CLOSED:
				if network_debug.get_status() != "Disconnected":
					network_debug.set_status("Disconnected")
					network_debug.add_message("WebSocket disconnected")
					if ping_timer != null:
						ping_timer.stop()

		# Handle incoming messages
		while websocket.get_available_packet_count() > 0:
			var packet = websocket.get_packet()
			var message = packet.get_string_from_utf8()

			network_debug.add_message("Received: " + message)

			# Parse JSON response
			var json = JSON.new()
			var error = json.parse(message)
			if error == OK:
				var data = json.get_data()
				if data.has("payload") and data.payload.has("Pong"):
					network_debug.add_message("Received pong - connection healthy")

func _on_exit_button_pressed():
	print("Exit button pressed")
	if websocket != null:
		websocket.close()
	if ping_timer != null:
		ping_timer.stop()
	get_tree().quit()