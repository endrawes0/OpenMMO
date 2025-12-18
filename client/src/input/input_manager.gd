# OpenMMO Client Input Manager
# Engine-agnostic input handling - translates Godot input events into game actions
# This module provides a clean interface for input that can be used by any client implementation

class_name InputManager
extends RefCounted

# Signals
signal movement_input_changed(input_vector: Vector2)
signal jump_pressed
signal jump_released
signal action_pressed(action_name: String)
signal mouse_moved(relative: Vector2)

# Input state
var movement_input: Vector2 = Vector2.ZERO
var jump_input_pressed: bool = false
var mouse_captured: bool = false

# Input configuration
var mouse_sensitivity: float = 0.002
var invert_y: bool = false

func _init():
	pass

func process_input(event):
	if event is InputEventMouseMotion and mouse_captured:
		var relative = event.relative
		if invert_y:
			relative.y = -relative.y
		emit_signal("mouse_moved", relative * mouse_sensitivity)

	elif event is InputEventKey:
		if event.pressed:
			match event.keycode:
				KEY_W, KEY_UP:
					_update_movement_input(Vector2.UP)
				KEY_S, KEY_DOWN:
					_update_movement_input(Vector2.DOWN)
				KEY_A, KEY_LEFT:
					_update_movement_input(Vector2.LEFT)
				KEY_D, KEY_RIGHT:
					_update_movement_input(Vector2.RIGHT)
				KEY_SPACE:
					if not jump_input_pressed:
						jump_input_pressed = true
						emit_signal("jump_pressed")
				KEY_ESCAPE:
					emit_signal("action_pressed", "escape")
				KEY_TAB:
					emit_signal("action_pressed", "tab")
				KEY_1, KEY_2, KEY_3, KEY_4, KEY_5:
					var ability_slot = event.keycode - KEY_1 + 1
					emit_signal("action_pressed", "ability_" + str(ability_slot))
		else:
			match event.keycode:
				KEY_W, KEY_UP:
					_update_movement_input(-Vector2.UP)
				KEY_S, KEY_DOWN:
					_update_movement_input(-Vector2.DOWN)
				KEY_A, KEY_LEFT:
					_update_movement_input(-Vector2.LEFT)
				KEY_D, KEY_RIGHT:
					_update_movement_input(-Vector2.RIGHT)
				KEY_SPACE:
					jump_input_pressed = false
					emit_signal("jump_released")

	elif event is InputEventMouseButton:
		if event.pressed:
			match event.button_index:
				MOUSE_BUTTON_LEFT:
					emit_signal("action_pressed", "primary_action")
				MOUSE_BUTTON_RIGHT:
					emit_signal("action_pressed", "secondary_action")
				MOUSE_BUTTON_WHEEL_UP:
					emit_signal("action_pressed", "scroll_up")
				MOUSE_BUTTON_WHEEL_DOWN:
					emit_signal("action_pressed", "scroll_down")

func _update_movement_input(direction: Vector2):
	var old_input = movement_input
	if direction.x != 0:
		movement_input.x += direction.x
	if direction.y != 0:
		movement_input.y += direction.y

	# Clamp to valid range
	movement_input = movement_input.clamp(Vector2(-1, -1), Vector2(1, 1))

	# Emit signal only if input changed
	if old_input != movement_input:
		emit_signal("movement_input_changed", movement_input)

func get_movement_input() -> Vector2:
	return movement_input

func is_jump_pressed() -> bool:
	return jump_input_pressed

func capture_mouse():
	if not mouse_captured:
		Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
		mouse_captured = true

func release_mouse():
	if mouse_captured:
		Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE)
		mouse_captured = false

func is_mouse_captured() -> bool:
	return mouse_captured

func set_mouse_sensitivity(sensitivity: float):
	mouse_sensitivity = sensitivity

func set_invert_y(invert: bool):
	invert_y = invert

func reset_input():
	movement_input = Vector2.ZERO
	jump_input_pressed = false
	emit_signal("movement_input_changed", movement_input)