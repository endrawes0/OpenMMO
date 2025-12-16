extends Node3D

# Game world scene - handles 3D rendering and scene management
# UI binding only - game logic handled by engine-agnostic modules per AGENTS.md

@onready var camera = $Camera3D
@onready var player = $Player

# Engine-agnostic modules
var game_state_manager = null
var movement_system = null
var input_manager = null

func _ready():
	print("GameWorld scene loaded")

	# Initialize engine-agnostic modules
	_initialize_modules()

	# Set up initial scene
	if camera:
		print("Camera position: ", camera.global_position)
	if player:
		print("Player position: ", player.global_position)

func _initialize_modules():
	# Load engine-agnostic modules
	game_state_manager = load("res://src/gamestate/game_state_manager.gd").new()
	movement_system = load("res://src/movement/movement_system.gd").new()
	input_manager = load("res://src/input/input_manager.gd").new()

	# Connect signals
	game_state_manager.connect("player_spawned", Callable(self, "_on_player_spawned"))
	game_state_manager.connect("entity_updated", Callable(self, "_on_entity_updated"))
	game_state_manager.connect("entity_removed", Callable(self, "_on_entity_removed"))

	input_manager.connect("movement_input_changed", Callable(self, "_on_movement_input_changed"))
	input_manager.connect("jump_pressed", Callable(self, "_on_jump_pressed"))
	input_manager.connect("action_pressed", Callable(self, "_on_action_pressed"))

	# Capture mouse for camera control
	input_manager.capture_mouse()

func _process(delta):
	if input_manager:
		input_manager.process_input(null)  # Process any buffered input

	if movement_system and player:
		# Update player position based on movement system
		var predicted_position = movement_system.predict_position(delta)
		player.global_position = predicted_position

func _input(event):
	# Pass input events to input manager
	if input_manager:
		input_manager.process_input(event)

func _on_player_spawned(player_id: int):
	print("Player spawned with ID: ", player_id)
	if player:
		player.show()

func _on_entity_updated(entity_id: int, entity_data: Dictionary):
	print("Entity updated: ", entity_id, " - ", entity_data)
	# TODO: Update 3D representation of entity

func _on_entity_removed(entity_id: int):
	print("Entity removed: ", entity_id)
	# TODO: Remove 3D representation of entity

func _on_movement_input_changed(input_vector: Vector2):
	if movement_system:
		movement_system.update(0.016, input_vector)  # Assume 60fps delta

func _on_jump_pressed():
	print("Jump pressed")
	# TODO: Handle jump action

func _on_action_pressed(action_name: String):
	match action_name:
		"escape":
			print("ESC pressed - return to menu")
			if input_manager:
				input_manager.release_mouse()
			get_tree().change_scene_to_file("res://scenes/Main.tscn")
		"tab":
			print("TAB pressed - toggle UI")
		"primary_action":
			print("Primary action (left click)")
		"secondary_action":
			print("Secondary action (right click)")
		_:
			if action_name.begins_with("ability_"):
				var slot = action_name.substr(8).to_int()
				print("Ability slot ", slot, " activated")