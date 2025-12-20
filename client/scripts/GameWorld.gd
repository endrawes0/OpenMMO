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
@onready var player_avatar: Node3D = $Player/PlayerAvatar
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
var entity_proxies: Dictionary = {}
var _proxies_root: Node3D = null
var _initial_snapshot_applied := false
var _initial_rotation_applied := false
var _initial_position_applied := false
var _trust_server_position := false
var _debug_local_delta := true
var _ignore_server_pose_for_local := true
var _camera_smooth_factor := 0.0
var _prev_camera_focus: Vector3 = Vector3.ZERO
var _player_reconcile_threshold := 0.5
var _player_reconcile_lerp := 0.2
var _debug_local_delta := false
var _player_position_reconcile_threshold := 0.3
const AVATAR_SCRIPT := preload("res://scripts/PlayerAvatar.gd")
const MALE_MODEL_PATH := "res://assets/models/Character/Superhero_Male_FullBody.gltf"
const FEMALE_MODEL_PATH := "res://assets/models/Character/Superhero_Female_FullBody.gltf"

func _ready() -> void:
	_load_session_modules()
	_load_character_data()
	_initialize_player()
	_ensure_proxies_root()
	_configure_camera_defaults()
	_apply_cached_snapshot()

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
	if movement_system and not movement_system.is_connected("movement_intent_sent", Callable(self, "_on_movement_intent_sent")):
		movement_system.connect("movement_intent_sent", Callable(self, "_on_movement_intent_sent"))

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
	if player_avatar and player_avatar.has_method("set"):
		player_avatar.set("use_female_model", _is_female_character(character_data))
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

func _ensure_proxies_root() -> void:
	if _proxies_root and is_instance_valid(_proxies_root):
		return
	_proxies_root = Node3D.new()
	_proxies_root.name = "EntityProxies"
	add_child(_proxies_root)

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
		movement_system.update(delta, input_vector, player.rotation.y)
	if player_avatar and player_avatar.has_method("update_motion"):
		player_avatar.call("update_motion", player.velocity, player.is_on_floor(), delta)

func _update_camera_position() -> void:
	if not camera:
		return
	var player_yaw = player.rotation.y
	var yaw = player_yaw + camera_orbit_offset
	var focus_point = player.global_position + Vector3(0, PLAYER_EYE_HEIGHT, 0)
	# Optional smoothing (disabled by default)
	if _camera_smooth_factor > 0.0:
		if _prev_camera_focus == Vector3.ZERO:
			_prev_camera_focus = focus_point
		else:
			_prev_camera_focus = _prev_camera_focus.lerp(focus_point, _camera_smooth_factor)
			focus_point = _prev_camera_focus
	
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
	if right_dragging:
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
	if client_networking:
		client_networking.close_connection()
	if get_tree().has_meta("session_modules"):
		var modules = get_tree().get_meta("session_modules")
		var ui_manager = modules.get("ui_state_manager", null)
		if ui_manager:
			ui_manager.go_to_character_select()
	get_tree().set_meta("selected_character", null)
	get_tree().change_scene_to_file("res://scenes/Main.tscn")

func _exit_tree() -> void:
	if client_networking:
		client_networking.close_connection()

func _on_world_snapshot_received(snapshot: Dictionary) -> void:
	_initial_snapshot_applied = true
	if game_state_manager:
		game_state_manager.apply_world_snapshot(snapshot)
	_sync_entity_proxies()
	_apply_authoritative_player_rotation()
	_apply_authoritative_player_position()
	_log_player_entity(snapshot)
	_update_hud_name()

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
	var server_pos := Vector3(pos.x, max(pos.y, MIN_FLOOR_Y), pos.z)
	if _ignore_server_pose_for_local:
		return

	if not _initial_position_applied:
		player.global_position = server_pos
		if movement_system:
			movement_system.set_target_position(player.global_position)
		_initial_position_applied = true
		return

	var delta := player.global_position.distance_to(server_pos)
	if delta > _player_reconcile_threshold:
		# Apply a gentle lerp toward server to avoid visible jitter, but still converge.
		player.global_position = player.global_position.lerp(server_pos, _player_reconcile_lerp)
		if movement_system:
			movement_system.set_target_position(player.global_position)
	elif _trust_server_position:
		player.global_position = server_pos
		if movement_system:
			movement_system.set_target_position(player.global_position)

	if _debug_local_delta and delta > 0.01:
		print_debug("LocalPlayer delta_to_server=%.4f client_pos=(%.3f, %.3f, %.3f) server_pos=(%.3f, %.3f, %.3f)" % [
			delta,
			player.global_position.x, player.global_position.y, player.global_position.z,
			server_pos.x, server_pos.y, server_pos.z
		])


func _log_player_entity(snapshot: Dictionary) -> void:
	if not game_state_manager:
		return
	var player_id = game_state_manager.player_entity_id
	var player_entity = game_state_manager.get_entity(player_id)
	if player_entity.is_empty():
		return
	var pos_str := ""
	if player_entity.has("position"):
		var p = player_entity.position
		pos_str = "pos=(%.3f, %.3f, %.3f)" % [p.x, p.y, p.z]
	var rot_str := ""
	if player_entity.has("rotation"):
		var r = player_entity.rotation
		if typeof(r) == TYPE_DICTIONARY and r.has("y"):
			rot_str = "rot_y=%.3f" % r.y
	var move_state := ""
	if player_entity.has("state"):
		var st = player_entity.state
		if typeof(st) == TYPE_DICTIONARY and st.has("movement_state"):
			move_state = str(st.movement_state)
	print_debug("PlayerEntity id=%s %s %s movement_state=%s" % [str(player_id), pos_str, rot_str, move_state])


func _log_remote_player(entity_id: int, entity_data: Dictionary) -> void:
	var pos_str := ""
	if entity_data.has("position"):
		var p = entity_data.position
		pos_str = "pos=(%.3f, %.3f, %.3f)" % [p.x, p.y, p.z]
	var rot_str := ""
	if entity_data.has("rotation"):
		var r = entity_data.rotation
		if typeof(r) == TYPE_DICTIONARY and r.has("y"):
			rot_str = "rot_y=%.3f" % r.y
	var move_state := ""
	if entity_data.has("state"):
		var st = entity_data.state
		if typeof(st) == TYPE_DICTIONARY and st.has("movement_state"):
			move_state = str(st.movement_state)
	print_debug("RemotePlayer id=%s %s %s movement_state=%s" % [str(entity_id), pos_str, rot_str, move_state])


func _update_hud_name() -> void:
	var hud_label: Label = get_node_or_null("HUD/PlayerName")
	if not hud_label:
		return
	var name := "Adventurer"
	if character_data.has("name"):
		name = str(character_data.get("name"))
	hud_label.text = name

func _apply_authoritative_player_rotation() -> void:
	if _initial_rotation_applied:
		return
	if not game_state_manager:
		return
	var player_id = game_state_manager.player_entity_id
	if player_id == 0:
		return
	var player_entity = game_state_manager.get_entity(player_id)
	if player_entity.is_empty():
		return
	if not player_entity.has("rotation"):
		return
	var rot = player_entity.rotation
	if rot.has("y"):
		player.rotation.y = rot.y
		if movement_system:
			movement_system.set_rotation_y(rot.y)
	_initial_rotation_applied = true

func _on_movement_intent_sent(intent: Dictionary) -> void:
	if client_networking:
		client_networking.send_message(intent)

func _sync_entity_proxies() -> void:
	if not game_state_manager:
		return
	_ensure_proxies_root()

	var current_entities: Dictionary = {}
	for entity_id in game_state_manager.entities.keys():
		current_entities[entity_id] = true
		if entity_id == game_state_manager.player_entity_id:
			continue
		var entity_data: Dictionary = game_state_manager.entities.get(entity_id, {})
		_spawn_or_update_proxy(entity_id, entity_data)

	# Despawn entities that no longer exist in snapshot
	for proxy_id in entity_proxies.keys():
		if not current_entities.has(proxy_id):
			_despawn_proxy(proxy_id)

func _spawn_or_update_proxy(entity_id: int, entity_data: Dictionary) -> void:
	if not entity_proxies.has(entity_id):
		var proxy = Node3D.new()
		proxy.name = "EntityProxy_%s" % entity_id
		var entity_type := str(entity_data.get("entity_type", "")).to_lower()
		if entity_type == "player":
			var avatar: Node3D = AVATAR_SCRIPT.new()
			avatar.name = "Avatar"
			avatar.set("use_female_model", _is_female_entity(entity_data))
			proxy.add_child(avatar)
		else:
			var mesh_instance = MeshInstance3D.new()
			var capsule = CapsuleMesh.new()
			capsule.radius = 0.4
			capsule.height = 2.0
			mesh_instance.mesh = capsule
			mesh_instance.material_override = _material_for_entity(entity_data)
			mesh_instance.name = "Capsule"
			proxy.add_child(mesh_instance)

		var label = Label3D.new()
		label.name = "NameLabel"
		label.text = _display_name_for_entity(entity_data)
		label.billboard = BaseMaterial3D.BILLBOARD_ENABLED
		label.transform.origin = Vector3(0, 2, 0)
		label.outline_size = 2
		proxy.add_child(label)

		_proxies_root.add_child(proxy)
		entity_proxies[entity_id] = proxy

	var proxy_node: Node3D = entity_proxies[entity_id]
	if not is_instance_valid(proxy_node):
		entity_proxies.erase(entity_id)
		return

	if entity_data.has("position"):
		var pos = entity_data.position
		var target := Vector3(pos.x, max(pos.y, MIN_FLOOR_Y), pos.z)
		proxy_node.global_position = target
	if entity_data.has("rotation"):
		var rot = entity_data.rotation
		if typeof(rot) == TYPE_DICTIONARY and rot.has("y"):
			var target_rot: float = rot.get("y", proxy_node.rotation.y)
			proxy_node.rotation.y = target_rot

	var label_node: Label3D = proxy_node.get_node_or_null("NameLabel")
	if label_node:
		label_node.text = _display_name_for_entity(entity_data)

	var avatar_node: Node3D = proxy_node.get_node_or_null("Avatar")
	if avatar_node and avatar_node.has_method("set_remote_movement_state"):
		var state_dict: Dictionary = entity_data.get("state", {}) if entity_data.has("state") else {}
		avatar_node.call_deferred("set_remote_movement_state", state_dict)
		_log_remote_player(entity_id, entity_data)
		_log_remote_player(entity_id, entity_data)

func _despawn_proxy(entity_id: int) -> void:
	if not entity_proxies.has(entity_id):
		return
	var proxy: Node3D = entity_proxies[entity_id]
	if is_instance_valid(proxy):
		proxy.queue_free()
	entity_proxies.erase(entity_id)

func _material_for_entity(entity_data: Dictionary) -> StandardMaterial3D:
	var mat = StandardMaterial3D.new()
	mat.metallic = 0.0
	mat.roughness = 0.6
	mat.transparency = BaseMaterial3D.TRANSPARENCY_DISABLED

	var entity_type = str(entity_data.get("entity_type", "")).to_lower()
	match entity_type:
		"player":
			mat.albedo_color = Color(0.3, 0.7, 1.0)  # blue
		"mob":
			mat.albedo_color = Color(0.9, 0.3, 0.3)  # red
		"npc":
			mat.albedo_color = Color(0.9, 0.8, 0.4)  # yellow/gold
		_:
			mat.albedo_color = Color(0.7, 0.7, 0.9)  # fallback

	return mat

func _display_name_for_entity(entity_data: Dictionary) -> String:
	var state = entity_data.get("state", {})
	if typeof(state) == TYPE_DICTIONARY and state.has("display_name"):
		return str(state.get("display_name", "Entity"))
	return "Entity"


func _is_female_entity(entity_data: Dictionary) -> bool:
	# Prefer explicit gender provided by the server on the entity or its state.
	if entity_data.has("gender"):
		var g = str(entity_data.get("gender", "")).to_lower()
		if g == "female" or g == "f":
			return true
		if g == "male" or g == "m":
			return false
	var state = entity_data.get("state", {})
	if typeof(state) == TYPE_DICTIONARY:
		if state.has("gender"):
			var gender_val = str(state.get("gender", "")).to_lower()
			return gender_val == "female" or gender_val == "f"
		if state.has("gender"):
			var gender_val = str(state.get("gender", "")).to_lower()
			return gender_val == "female" or gender_val == "f"
		if state.has("class"):
			# Placeholder heuristic; can be replaced with explicit metadata later.
			var class_val = str(state.get("class", "")).to_lower()
			return class_val.find("female") != -1
	return false


func _is_female_character(char_data: Dictionary) -> bool:
	if char_data.has("gender"):
		var g = str(char_data.get("gender", "")).to_lower()
		return g == "female" or g == "f"
	if char_data.has("class"):
		var c = str(char_data.get("class", "")).to_lower()
		return c.find("female") != -1
	return false

func _apply_cached_snapshot() -> void:
	if _initial_snapshot_applied:
		return
	if get_tree().has_meta("latest_world_snapshot"):
		var snapshot = get_tree().get_meta("latest_world_snapshot")
		if typeof(snapshot) == TYPE_DICTIONARY and not snapshot.is_empty():
			_on_world_snapshot_received(snapshot)
