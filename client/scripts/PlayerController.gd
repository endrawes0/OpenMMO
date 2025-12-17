# OpenMMO Player Controller
# Engine-specific player controller - bridges engine-agnostic systems with Godot CharacterBody3D
# This script handles the CharacterBody3D physics and connects to movement_system

extends CharacterBody3D

# Node references
@onready var mesh_instance = $MeshInstance3D
@onready var collision_shape = $CollisionShape3D

# Engine-agnostic systems
var movement_system = null
var input_manager = null
var game_state_manager = null

# Player state
var player_id: int = 0
var is_initialized: bool = false

# Movement parameters
var base_speed: float = 5.0
var jump_velocity: float = 4.5
var gravity: float = 9.8

# Rotation parameters
var target_rotation: float = 0.0
var rotation_speed: float = 10.0

func _ready():
	print("PlayerController _ready() called")
	
	# Load engine-agnostic modules
	movement_system = load("res://src/movement/movement_system.gd").new()
	input_manager = load("res://src/input/input_manager.gd").new()
	game_state_manager = load("res://src/gamestate/game_state_manager.gd").new()
	
	# Connect signals
	_connect_signals()
	
	# Initialize movement system
	movement_system.set_target_position(global_position)
	movement_system.set_movement_speed(base_speed)
	
	is_initialized = true
	print("PlayerController initialized")

func _connect_signals():
	# Connect to input manager
	input_manager.connect("movement_input_changed", Callable(self, "_on_movement_input_changed"))
	input_manager.connect("jump_pressed", Callable(self, "_on_jump_pressed"))
	input_manager.connect("action_pressed", Callable(self, "_on_action_pressed"))
	input_manager.connect("mouse_moved", Callable(self, "_on_mouse_moved"))

	# Connect to movement system
	movement_system.connect("movement_intent_sent", Callable(self, "_on_movement_intent_sent"))
	movement_system.connect("position_corrected", Callable(self, "_on_position_corrected"))

	# Connect to game state manager
	game_state_manager.connect("player_spawned", Callable(self, "_on_player_spawned"))

func _physics_process(delta):
	if not is_initialized:
		return

	# Handle player rotation
	var current_rotation = rotation.y
	var rotation_diff = target_rotation - current_rotation
	rotation_diff = wrapf(rotation_diff, -PI, PI)  # Keep in -PI to PI range

	if abs(rotation_diff) > 0.01:
		var rotation_step = sign(rotation_diff) * rotation_speed * delta
		if abs(rotation_step) > abs(rotation_diff):
			rotation_step = rotation_diff
		rotation.y += rotation_step

	# Handle gravity
	if not is_on_floor():
		velocity.y -= gravity * delta

	# Get movement input from input manager
	var movement_input = input_manager.get_movement_input()

	# Convert 2D input to 3D movement relative to player facing
	var move_direction = Vector3.ZERO
	if movement_input.length() > 0:
		# Convert input to local space (relative to player rotation)
		var local_direction = Vector3(movement_input.x, 0, movement_input.y).normalized()

		# Transform to world space based on player rotation
		var player_basis = Basis(Vector3.UP, rotation.y)
		move_direction = player_basis * local_direction

		# Apply movement speed
		var horizontal_velocity = move_direction * movement_system.get_movement_speed()
		velocity.x = horizontal_velocity.x
		velocity.z = horizontal_velocity.z
	else:
		# Stop horizontal movement when no input
		velocity.x = 0
		velocity.z = 0

	# Handle jump
	if input_manager.is_jump_pressed() and is_on_floor():
		velocity.y = jump_velocity

	# Move the character
	move_and_slide()

	# Update movement system with new position
	movement_system.set_target_position(global_position)

	# Update movement system for network synchronization
	if move_direction.length() > 0:
		movement_system.update(delta, Vector2(move_direction.x, move_direction.z), false)
	else:
		movement_system.stop_movement()

func _process(delta):
	if not is_initialized:
		return
	
	# Process any buffered input
	input_manager.process_input(null)

func _input(event):
	# Pass input events to input manager
	if input_manager:
		input_manager.process_input(event)

func _on_movement_input_changed(input_vector: Vector2):
	# Movement input changed - handled in _physics_process
	pass

func _on_mouse_moved(relative: Vector2):
	# Update target rotation based on horizontal mouse movement
	# Only rotate player when right mouse button is held (camera is rotating)
	var camera = get_parent().get_node("Camera3D")
	if camera and camera.has_method("is_camera_rotating") and camera.is_camera_rotating():
		target_rotation += relative.x

func _on_jump_pressed():
	# Jump handled in _physics_process
	pass

func _on_action_pressed(action_name: String):
	match action_name:
		"escape":
			print("ESC pressed - return to menu")
			input_manager.release_mouse()
			get_tree().change_scene_to_file("res://scenes/Main.tscn")
		"primary_action":
			print("Primary action (left click)")
			# TODO: Handle combat/interaction
		"secondary_action":
			print("Secondary action (right click)")
			# TODO: Handle camera zoom or alternative actions
		_:
			if action_name.begins_with("ability_"):
				var slot = action_name.substr(8).to_int()
				print("Ability slot ", slot, " activated")
				# TODO: Handle ability activation

func _on_movement_intent_sent(intent: Dictionary):
	# Send movement intent to server via networking
	# TODO: Connect to client networking when available
	print("Movement intent: ", intent)

func _on_position_corrected(entity_id: int, server_position: Vector3, client_position: Vector3):
	if entity_id == player_id:
		print("Position corrected: ", client_position, " -> ", server_position)
		global_position = server_position

func _on_player_spawned(spawned_player_id: int):
	player_id = spawned_player_id
	print("Player spawned with ID: ", player_id)
	show()

func set_player_id(id: int):
	player_id = id
	game_state_manager.set_player_entity(id)

func get_player_id() -> int:
	return player_id

func set_movement_speed(speed: float):
	base_speed = speed
	if movement_system:
		movement_system.set_movement_speed(speed)

func get_movement_speed() -> float:
	return base_speed