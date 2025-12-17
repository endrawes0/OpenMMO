# OpenMMO Third-Person Camera Controller
# Engine-specific camera controller - implements classic MMO third-person camera
# Follows player with adjustable distance, height, and mouse rotation

extends Camera3D

# Camera configuration
var camera_distance: float = 8.0
var camera_height: float = 5.0
var camera_angle: float = 45.0  # Downward angle in degrees
var mouse_sensitivity: float = 0.002
var invert_y: bool = false

# Camera state
var target_node: Node3D = null
var current_rotation: float = 0.0
var current_pitch: float = 0.0
var is_rotating: bool = false

# Input manager reference
var input_manager = null

# Smoothing
var position_smoothing: float = 5.0
var rotation_smoothing: float = 10.0

func _ready():
	print("ThirdPersonCamera _ready() called")
	
	# Load input manager
	input_manager = load("res://src/input/input_manager.gd").new()
	input_manager.connect("mouse_moved", Callable(self, "_on_mouse_moved"))
	input_manager.connect("action_pressed", Callable(self, "_on_action_pressed"))
	
	# Set initial camera position
	if target_node:
		update_camera_position()

func _process(delta):
	if not target_node:
		return
	
	# Smooth camera follow
	var target_position = calculate_camera_position()
	global_position = global_position.lerp(target_position, position_smoothing * delta)
	
	# Look at target
	look_at(target_node.global_position + Vector3.UP * 2.0)

func _input(event):
	# Pass input to input manager for mouse handling
	if input_manager:
		input_manager.process_input(event)

func set_target(target: Node3D):
	target_node = target
	if target:
		print("Camera following: ", target.name)
		update_camera_position()

func get_target() -> Node3D:
	return target_node

func update_camera_position():
	if not target_node:
		return
	
	var target_position = calculate_camera_position()
	global_position = target_position
	look_at(target_node.global_position + Vector3.UP * 2.0)

func calculate_camera_position() -> Vector3:
	if not target_node:
		return global_position
	
	# Calculate camera position based on spherical coordinates
	var target_pos = target_node.global_position + Vector3.UP * 2.0
	
	# Convert angles to radians
	var yaw_rad = deg_to_rad(current_rotation)
	var pitch_rad = deg_to_rad(current_pitch)
	
	# Calculate offset
	var offset_x = camera_distance * cos(pitch_rad) * sin(yaw_rad)
	var offset_y = -camera_distance * sin(pitch_rad)
	var offset_z = camera_distance * cos(pitch_rad) * cos(yaw_rad)
	
	return target_pos + Vector3(offset_x, offset_y + camera_height, offset_z)

func _on_mouse_moved(relative: Vector2):
	if not is_rotating:
		return
	
	# Update rotation based on mouse movement
	current_rotation += relative.x
	if invert_y:
		current_pitch += relative.y
	else:
		current_pitch -= relative.y
	
	# Clamp pitch to prevent camera flipping
	current_pitch = clamp(current_pitch, -80.0, 80.0)
	
	# Update camera position immediately for responsive feel
	update_camera_position()

func _on_action_pressed(action_name: String):
	match action_name:
		"primary_action":
			# Left mouse button - could be for abilities/interaction
			pass
		"secondary_action":
			# Right mouse button - start camera rotation
			is_rotating = true
			input_manager.capture_mouse()
		"primary_action_release":
			# Left mouse release
			pass
		"secondary_action_release":
			# Right mouse release - stop camera rotation
			is_rotating = false
			input_manager.release_mouse()
		"escape":
			# Stop camera rotation and release mouse
			is_rotating = false
			input_manager.release_mouse()

func set_camera_distance(distance: float):
	camera_distance = clamp(distance, 2.0, 20.0)
	update_camera_position()

func get_camera_distance() -> float:
	return camera_distance

func set_camera_height(height: float):
	camera_height = height
	update_camera_position()

func get_camera_height() -> float:
	return camera_height

func set_camera_angle(angle: float):
	camera_angle = angle
	current_pitch = angle
	update_camera_position()

func get_camera_angle() -> float:
	return camera_angle

func set_mouse_sensitivity(sensitivity: float):
	mouse_sensitivity = sensitivity
	if input_manager:
		input_manager.set_mouse_sensitivity(sensitivity)

func get_mouse_sensitivity() -> float:
	return mouse_sensitivity

func set_invert_y(invert: bool):
	invert_y = invert
	if input_manager:
		input_manager.set_invert_y(invert)

func is_camera_rotating() -> bool:
	return is_rotating