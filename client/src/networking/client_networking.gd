# OpenMMO Client Networking Module
# Engine-agnostic networking logic - handles WebSocket communication and message protocol
# This module is independent of Godot and can be tested/used in any client implementation

class_name ClientNetworking
extends RefCounted

# Signals
signal connected
signal disconnected
signal message_received(message: Dictionary)
signal connection_error(error: String)
signal auth_successful(auth_data: Dictionary)
signal auth_failed(reason: String)
signal character_list_received(characters: Array)
signal character_created(character_data: Dictionary)
signal character_selected(character_data: Dictionary)
signal world_snapshot_received(snapshot: Dictionary)

# Connection state
enum ConnectionState {
	DISCONNECTED,
	CONNECTING,
	CONNECTED,
	ERROR
}

var connection_state: ConnectionState = ConnectionState.DISCONNECTED
var websocket: WebSocketPeer = null
var sequence_id: int = 0
var session_id: String = ""
var player_id: int = 0
const MAX_SIGNED_64: int = 9223372036854775807

# Message queues
var outgoing_queue: Array = []
var incoming_queue: Array = []

func _init():
	pass

func connect_to_server(url: String) -> Error:
	# Always close any existing connection first
	if websocket:
		websocket.close()
		websocket = null
		connection_state = ConnectionState.DISCONNECTED
		session_id = ""
		player_id = 0

	connection_state = ConnectionState.CONNECTING
	websocket = WebSocketPeer.new()

	var error = websocket.connect_to_url(url)
	if error != OK:
		connection_state = ConnectionState.ERROR
		emit_signal("connection_error", "Failed to initiate connection: " + str(error))
		return error

	# Connection initiated successfully - actual connection will be confirmed in poll()
	return OK

func close_connection():
	if websocket:
		websocket.close()
		websocket = null
		connection_state = ConnectionState.DISCONNECTED
		session_id = ""
		player_id = 0
	websocket = null
	connection_state = ConnectionState.DISCONNECTED
	session_id = ""
	player_id = 0
	emit_signal("disconnected")

func poll():
	if not websocket:
		return

	websocket.poll()

	var state = websocket.get_ready_state()
	match state:
		WebSocketPeer.STATE_OPEN:
			if connection_state != ConnectionState.CONNECTED:
				connection_state = ConnectionState.CONNECTED
				print("DEBUG: Emitting connected signal")
				emit_signal("connected")
				_send_handshake()
		WebSocketPeer.STATE_CLOSED:
			if connection_state != ConnectionState.DISCONNECTED:
				connection_state = ConnectionState.DISCONNECTED
				emit_signal("disconnected")

	# Process incoming messages
	while websocket.get_available_packet_count() > 0:
		var packet = websocket.get_packet()
		var message = packet.get_string_from_utf8()

		var json = JSON.new()
		var error = json.parse(message)
		if error == OK:
			var data = json.get_data()
			_process_incoming_message(data)
		else:
			push_error("Failed to parse incoming message: " + message)

func send_message(payload: Dictionary) -> Error:
	if connection_state != ConnectionState.CONNECTED:
		push_error("Not connected to server")
		return ERR_CONNECTION_ERROR

	var message = {
		"sequence_id": sequence_id,
		"timestamp": int(Time.get_unix_time_from_system() * 1000),
		"payload": payload
	}
	sequence_id += 1

	var json_string = JSON.stringify(message)
	var error = websocket.send_text(json_string)

	if error != OK:
		push_error("Failed to send message: " + str(error))
		return error

	return OK

func _send_handshake():
	var handshake = {
		"HandshakeRequest": {
			"client_version": "0.1.0",
			"protocol_version": "1.0",
			"supported_features": 0
		}
	}
	send_message(handshake)

func _process_incoming_message(message: Dictionary):
	emit_signal("message_received", message)

	# Handle specific message types
	if message.has("payload"):
		var payload = message.payload

		if payload.has("HandshakeResponse"):
			_handle_handshake_response(payload.HandshakeResponse)
		elif payload.has("AuthResponse"):
			_handle_auth_response(payload.AuthResponse)
		elif payload.has("CharacterListResponse"):
			_handle_character_list_response(payload.CharacterListResponse)
		elif payload.has("CharacterCreateResponse"):
			_handle_character_create_response(payload.CharacterCreateResponse)
		elif payload.has("CharacterSelectResponse"):
			_handle_character_select_response(payload.CharacterSelectResponse)
		elif payload.has("WorldSnapshot"):
			_handle_world_snapshot(payload.WorldSnapshot)
		elif payload.has("Pong"):
			_handle_pong(payload.Pong)
		elif payload.has("Error"):
			_handle_error(payload.Error)

func _handle_handshake_response(response: Dictionary):
	if response.accepted:
		print("Handshake accepted - connected to server")
	else:
		push_error("Handshake rejected: " + response.get("message", "Unknown error"))
		connection_state = ConnectionState.ERROR
		emit_signal("connection_error", "Handshake rejected")

func _handle_pong(pong: Dictionary):
	# Calculate ping time if needed
	pass

func _handle_auth_response(response: Dictionary):
	if response.success:
		session_id = response.get("session_token", "")
		player_id = _u64_to_int(response.get("player_id", 0))
		emit_signal("auth_successful", response)
	else:
		emit_signal("auth_failed", response.get("message", "Authentication failed"))

func _handle_character_list_response(response: Dictionary):
	var characters = response.get("characters", [])
	emit_signal("character_list_received", characters)

func _handle_character_create_response(response: Dictionary):
	if response.success and response.has("character"):
		emit_signal("character_created", response.character)
	else:
		var error_msg = response.get("error_message", "Character creation failed")
		emit_signal("connection_error", error_msg)

func _handle_character_select_response(response: Dictionary):
	if response.has("character") and response.character != null:
		emit_signal("character_selected", response.character)
	else:
		emit_signal("connection_error", "Character selection failed")

func _handle_world_snapshot(snapshot: Dictionary):
	emit_signal("world_snapshot_received", snapshot)

func _handle_error(error: Dictionary):
	push_error("Server error: " + error.get("message", "Unknown error"))
	emit_signal("connection_error", error.get("message", "Unknown error"))

func get_connection_state() -> ConnectionState:
	return connection_state

func is_connection_active() -> bool:
	return connection_state == ConnectionState.CONNECTED

func get_session_id() -> String:
	return session_id

func get_player_id() -> int:
	return player_id

func _u64_to_int(value) -> int:
	if typeof(value) != TYPE_INT and typeof(value) != TYPE_FLOAT:
		return 0
	var parsed = int(value)
	if parsed < 0 or parsed > MAX_SIGNED_64:
		return 0
	return parsed

# Authentication methods
func send_login_request(username: String, password: String) -> Error:
	print("DEBUG: send_login_request called for user:", username)
	var auth_request = {
		"AuthRequest": {
			"username": username,
			"password_hash": _hash_password(password),
			"character_name": null
		}
	}
	var result = send_message(auth_request)
	print("DEBUG: send_login_request result:", result)
	return result

func send_register_request(username: String, password: String) -> Error:
	# For MVP, registration is handled the same as login (server creates account if it doesn't exist)
	return send_login_request(username, password)

# Character management methods
func request_character_list() -> Error:
	var request = {
		"CharacterListRequest": {
			"request": true
		}
	}
	return send_message(request)

func create_character(name: String, character_class: String) -> Error:
	var request = {
		"CharacterCreateRequest": {
			"name": name,
			"class": character_class
		}
	}
	return send_message(request)

func select_character(character_id: int) -> Error:
	var request = {
		"CharacterSelectRequest": {
			"character_id": character_id
		}
	}
	return send_message(request)

# Utility methods
func _hash_password(password: String) -> String:
	# For MVP, using simple hash. In production, use proper password hashing
	return password.sha256_text()
