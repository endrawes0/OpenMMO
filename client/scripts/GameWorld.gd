extends Node3D

const MOVE_SPEED := 6.48  # Increased by another 20% from 5.4
const JUMP_SPEED := 8.0
const ACCELERATION := 12.0
const CAMERA_MIN_DISTANCE := 0.5  # First person distance
const CAMERA_MAX_DISTANCE := 12.0
const CAMERA_MIN_PITCH := deg_to_rad(-20.0)  # 20 degrees below player
const CAMERA_MAX_PITCH := deg_to_rad(60.0)   # 60 degrees above player
const CAMERA_SENSITIVITY := 0.005
const CAMERA_ZOOM_STEP := 0.75
const PLAYER_EYE_HEIGHT := 1.6
const MAX_SIGNED_64: int = 9223372036854775807
const MIN_FLOOR_Y := 0.5  # Safety floor to keep the player above terrain

@onready var gravity_value: float = ProjectSettings.get_setting("physics/3d/default_gravity") * 1.2  # Increased by 20%
@onready var camera: Camera3D = $Camera3D
@onready var player: CharacterBody3D = $Player
@onready var spawn_point: Marker3D = $Zone/SpawnPoint

var client_networking = null
var game_state_manager = null
var movement_system = null
var input_manager = null

var character_data: Dictionary = {}
var movement_input: Vector2 = Vector2.ZERO
var camera_distance: float = 6.0
var camera_pitch: float = deg_to_rad(20.0)  # 20 degrees above player
var camera_orbit_offset: float = 0.0
var left_dragging := false
var right_dragging := false
var jump_pressed := false

func _ready() -> void:
	_load_session_modules()
	_load_character_data()
	_initialize_player()
	_configure_camera_defaults()

func _physics_process(delta: float) -> void:
	if client_networking:
		client_networking.poll()
	_update_player_movement(delta)
	_update_camera_position()

func _input(event) -> void:
	if input_manager:
		input_manager.process_input(event)
	_handle_mouse_input(event)

func _load_session_modules() -> void:
	if get_tree().has_meta("session_modules"):
		var modules = get_tree().get_meta("session_modules")
		client_networking = modules.get("client_networking", null)
		game_state_manager = modules.get("game_state_manager", null)
		movement_system = modules.get("movement_system", null)
		input_manager = modules.get("input_manager", null)
	else:
		client_networking = load("res://src/networking/client_networking.gd").new()
		game_state_manager = load("res://src/gamestate/game_state_manager.gd").new()
		movement_system = load("res://src/movement/movement_system.gd").new()
		input_manager = load("res://src/input/input_manager.gd").new()

	if input_manager:
		if not input_manager.is_connected("movement_input_changed", Callable(self, "_on_movement_input_changed")):
			input_manager.connect("movement_input_changed", Callable(self, "_on_movement_input_changed"))
		if not input_manager.is_connected("jump_pressed", Callable(self, "_on_jump_pressed")):
			input_manager.connect("jump_pressed", Callable(self, "_on_jump_pressed"))
		if not input_manager.is_connected("jump_released", Callable(self, "_on_jump_released")):
			input_manager.connect("jump_released", Callable(self, "_on_jump_released"))
		if not input_manager.is_connected("action_pressed", Callable(self, "_on_action_pressed")):
			input_manager.connect("action_pressed", Callable(self, "_on_action_pressed"))
	if client_networking and not client_networking.is_connected("world_snapshot_received", Callable(self, "_on_world_snapshot_received")):
		client_networking.connect("world_snapshot_received", Callable(self, "_on_world_snapshot_received"))

func _load_character_data() -> void:
	if get_tree().has_meta("selected_character"):
		character_data = get_tree().get_meta("selected_character")
	elif get_tree().has_meta("latest_world_snapshot"):
		var snapshot = get_tree().get_meta("latest_world_snapshot")
		var player_id = _u64_to_int(snapshot.get("player_entity_id", 0))
		for entity_data in snapshot.get("entities", []):
			if _u64_to_int(entity_data.get("id", 0)) == player_id:
				character_data = {
					"id": player_id,
					"name": entity_data.get("state", {}).get("display_name", "Adventurer"),
					"class": entity_data.get("entity_type", "Adventurer"),
					"level": 1
				}
				break
	else:
		character_data = {
			"id": 0,
			"name": "Adventurer",
			"class": "Warrior",
			"level": 1
		}

func _initialize_player() -> void:
	if spawn_point:
		player.global_transform.origin = spawn_point.global_transform.origin
	if movement_system:
		movement_system.set_target_position(player.global_position)
	if game_state_manager:
		var player_id = _u64_to_int(character_data.get("id", 0))
		if player_id > 0:
			game_state_manager.set_player_entity(player_id)
			var starting_entity = {
				"id": player_id,
				"name": character_data.get("name", "Adventurer"),
				"class": character_data.get("class", "Adventurer"),
				"level": character_data.get("level", 1),
				"position": {
					"x": player.global_position.x,
					"y": player.global_position.y,
					"z": player.global_position.z
				},
				"movement_state": "Idle"
			}
			game_state_manager.add_entity(player_id, starting_entity)

func _u64_to_int(value) -> int:
	if typeof(value) != TYPE_INT and typeof(value) != TYPE_FLOAT:
		return 0
	var parsed = int(value)
	if parsed < 0 or parsed > MAX_SIGNED_64:
		return 0
	return parsed

func _configure_camera_defaults() -> void:
	camera_distance = clamp(camera_distance, CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE)
	camera_pitch = clamp(camera_pitch, CAMERA_MIN_PITCH, CAMERA_MAX_PITCH)
	if camera:
		camera.current = true

func _update_player_movement(delta: float) -> void:
	var input_vector = Vector2(movement_input.x, -movement_input.y)
	if input_vector.length_squared() > 1.0:
		input_vector = input_vector.normalized()

	var move_direction = Vector3.ZERO
	if input_vector.length_squared() > 0.0:
		var basis = player.global_transform.basis
		var forward = -basis.z
		var right = basis.x
		move_direction = (right * input_vector.x) + (forward * input_vector.y)
		move_direction = move_direction.normalized()

	var target_velocity = move_direction * MOVE_SPEED
	var horizontal_velocity = Vector3(player.velocity.x, 0, player.velocity.z)
	horizontal_velocity = horizontal_velocity.move_toward(target_velocity, ACCELERATION * delta)
	player.velocity.x = horizontal_velocity.x
	player.velocity.z = horizontal_velocity.z
	
	# Handle jumping
	if jump_pressed and player.is_on_floor():
		player.velocity.y = JUMP_SPEED
	else:
		player.velocity.y -= gravity_value * delta

	player.move_and_slide()

	if movement_system:
		movement_system.set_target_position(player.global_position)

func _update_camera_position() -> void:
	if not camera:
		return
	var player_yaw = player.rotation.y
	var yaw = player_yaw + camera_orbit_offset
	var focus_point = player.global_position + Vector3(0, PLAYER_EYE_HEIGHT, 0)
	
	# Handle first person camera (very close to player)
	if camera_distance <= CAMERA_MIN_DISTANCE:
		camera.global_position = focus_point + Vector3(0, 0.1, 0)  # Slightly above eye level
		camera.rotation = player.rotation
		camera.rotation.x = -camera_pitch  # Apply pitch directly
	else:
		# Third person camera
		var horizontal_distance = camera_distance * cos(camera_pitch)
		var height_offset = camera_distance * sin(camera_pitch)
		var offset = Vector3(
			horizontal_distance * sin(yaw),
			height_offset,
			horizontal_distance * cos(yaw)
		)
		camera.global_position = focus_point + offset
		camera.look_at(focus_point, Vector3.UP)

func _handle_mouse_input(event) -> void:
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT:
			left_dragging = event.pressed
			_update_mouse_mode()
		elif event.button_index == MOUSE_BUTTON_RIGHT:
			right_dragging = event.pressed
			if right_dragging:
				camera_orbit_offset = 0.0
			_update_mouse_mode()
		elif event.button_index == MOUSE_BUTTON_WHEEL_UP:
			_adjust_zoom(-CAMERA_ZOOM_STEP)
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			_adjust_zoom(CAMERA_ZOOM_STEP)
	elif event is InputEventMouseMotion:
		if left_dragging or right_dragging:
			_handle_camera_drag(event.relative)

func _update_mouse_mode() -> void:
	if left_dragging or right_dragging:
		Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
	else:
		Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)

func _handle_camera_drag(relative: Vector2) -> void:
	var horizontal_delta = -relative.x * CAMERA_SENSITIVITY
	var vertical_delta = relative.y * CAMERA_SENSITIVITY  # Invert up/down

	if left_dragging:
		# Left drag: revolve camera around player
		camera_orbit_offset = wrapf(camera_orbit_offset + horizontal_delta, -TAU, TAU)
		camera_pitch = clamp(camera_pitch + vertical_delta, CAMERA_MIN_PITCH, CAMERA_MAX_PITCH)
	elif right_dragging:
		# Right drag left/right: rotate player (and camera relative to player)
		player.rotate_y(horizontal_delta)
		camera_orbit_offset = 0.0
		
		# Right drag up/down: revolve camera above/below player with angle limits
		camera_pitch = clamp(camera_pitch + vertical_delta, CAMERA_MIN_PITCH, CAMERA_MAX_PITCH)

func _adjust_zoom(amount: float) -> void:
	camera_distance = clamp(camera_distance + amount, CAMERA_MIN_DISTANCE, CAMERA_MAX_DISTANCE)

func _on_movement_input_changed(input_vector: Vector2) -> void:
	movement_input = input_vector
	if movement_system:
		movement_system.update(0.0, input_vector)

func _on_jump_pressed() -> void:
	jump_pressed = true

func _on_jump_released() -> void:
	jump_pressed = false

func _on_action_pressed(action_name: String) -> void:
	match action_name:
		"escape":
			_return_to_menu()
		"scroll_up":
			_adjust_zoom(-CAMERA_ZOOM_STEP)
		"scroll_down":
			_adjust_zoom(CAMERA_ZOOM_STEP)

func _return_to_menu() -> void:
	left_dragging = false
	right_dragging = false
	_update_mouse_mode()
	if get_tree().has_meta("session_modules"):
		var modules = get_tree().get_meta("session_modules")
		var ui_manager = modules.get("ui_state_manager", null)
		if ui_manager:
			ui_manager.go_to_character_select()
	get_tree().set_meta("selected_character", null)
	get_tree().change_scene_to_file("res://scenes/Main.tscn")

func _on_world_snapshot_received(snapshot: Dictionary) -> void:
	if game_state_manager:
		game_state_manager.apply_world_snapshot(snapshot)
	_apply_authoritative_player_position()

func _apply_authoritative_player_position() -> void:
	if not game_state_manager:
		return
	var player_id = game_state_manager.player_entity_id
	if player_id == 0:
		return
	var player_entity = game_state_manager.get_entity(player_id)
	if player_entity.is_empty():
		return
	if not player_entity.has("position"):
		return
	var pos = player_entity.position
	var authoritative_position = Vector3(pos.x, max(pos.y, MIN_FLOOR_Y), pos.z)

	# Reconcile only when drift is noticeable to avoid jitter
	var drift_distance = player.global_position.distance_to(authoritative_position)
	if drift_distance > 0.25:
		# Smoothly correct toward the authoritative position to avoid popping
		var blended = player.global_position.lerp(authoritative_position, 0.5)
		player.global_position = blended
