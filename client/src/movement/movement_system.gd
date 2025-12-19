# OpenMMO Client Movement System
# Engine-agnostic movement logic - handles player movement, prediction, and server reconciliation
# This module manages movement input, prediction, and synchronization with server

class_name MovementSystem
extends RefCounted

# Signals
signal movement_intent_sent(intent: Dictionary)
signal position_corrected(entity_id: int, server_position: Vector3, client_position: Vector3)

# Movement state
var movement_speed: float = 5.0
var rotation_speed: float = 180.0  # degrees per second
var is_moving: bool = false
var target_position: Vector3 = Vector3.ZERO
var current_velocity: Vector3 = Vector3.ZERO
var _was_moving: bool = false
var current_rotation_y: float = 0.0
var last_sent_rotation_y: float = 0.0
var rotation_epsilon: float = 0.01

# Prediction and reconciliation
var pending_movements: Array = []
var last_server_update: float = 0.0
var prediction_error_threshold: float = 0.1

func _init():
	pass

func update(delta: float, input_vector: Vector2, rotation_y: float, jump_pressed: bool = false):
	# Update movement based on input
	var move_vector = Vector3(input_vector.x, 0, input_vector.y).normalized()
	_was_moving = is_moving
	current_rotation_y = rotation_y

	if move_vector.length() > 0:
		is_moving = true
		current_velocity = move_vector * movement_speed

		# Send movement intent to server
		_send_movement_intent(target_position + move_vector * movement_speed * delta, false)
	else:
		is_moving = false
		current_velocity = Vector3.ZERO
		if _was_moving:
			_send_movement_intent(target_position, true)
		elif abs(current_rotation_y - last_sent_rotation_y) > rotation_epsilon:
			_send_movement_intent(target_position, true)

func set_target_position(position: Vector3):
	target_position = position
func set_rotation_y(rotation_y: float):
	current_rotation_y = rotation_y

func get_target_position() -> Vector3:
	return target_position

func get_current_velocity() -> Vector3:
	return current_velocity

func is_entity_moving() -> bool:
	return is_moving

func _send_movement_intent(target_pos: Vector3, stop_movement: bool):
	var intent = {
		"MovementIntent": {
			"target_position": {
				"x": target_pos.x,
				"y": target_pos.y,
				"z": target_pos.z
			},
			"speed_modifier": 1.0,
			"stop_movement": stop_movement,
			"rotation_y": current_rotation_y
		}
	}

	emit_signal("movement_intent_sent", intent)
	last_sent_rotation_y = current_rotation_y

	# Store for prediction
	pending_movements.append({
		"target": target_pos,
		"timestamp": Time.get_unix_time_from_system()
	})

func stop_movement():
	var intent = {
		"MovementIntent": {
			"target_position": {
				"x": target_position.x,
				"y": target_position.y,
				"z": target_position.z
			},
			"speed_modifier": 1.0,
			"stop_movement": true
		}
	}

	emit_signal("movement_intent_sent", intent)
	is_moving = false
	current_velocity = Vector3.ZERO

func reconcile_server_position(entity_id: int, server_position: Vector3, server_velocity: Vector3 = Vector3.ZERO):
	# Compare server position with predicted position
	var predicted_position = target_position
	var error_distance = predicted_position.distance_to(server_position)

	if error_distance > prediction_error_threshold:
		# Significant prediction error - correct position
		emit_signal("position_corrected", entity_id, server_position, predicted_position)
		target_position = server_position
		current_velocity = server_velocity

	# Clear old pending movements
	var current_time = Time.get_unix_time_from_system()
	pending_movements = pending_movements.filter(func(movement):
		return current_time - movement.timestamp < 1.0  # Keep recent movements
	)

func predict_position(delta: float) -> Vector3:
	if is_moving:
		return target_position + current_velocity * delta
	return target_position

func set_movement_speed(speed: float):
	movement_speed = speed

func get_movement_speed() -> float:
	return movement_speed

func set_rotation_speed(speed: float):
	rotation_speed = speed

func get_rotation_speed() -> float:
	return rotation_speed
