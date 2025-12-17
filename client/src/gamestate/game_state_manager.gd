# OpenMMO Client Game State Manager
# Engine-agnostic game state management - tracks entities, player state, and world data
# This module maintains the client's authoritative view of the game world

class_name GameStateManager
extends RefCounted

# Signals
signal player_spawned(player_id: int)
signal entity_updated(entity_id: int, entity_data: Dictionary)
signal entity_removed(entity_id: int)
signal zone_changed(zone_id: int)

# Game state
var current_zone_id: int = 1
var player_entity_id: int = 0
var entities: Dictionary = {}  # entity_id -> entity_data
var player_stats: Dictionary = {}
var inventory: Array = []
var equipment: Dictionary = {}

func _init():
	_reset_state()

func _reset_state():
	current_zone_id = 1
	player_entity_id = 0
	entities.clear()
	player_stats = {
		"health": 100,
		"max_health": 100,
		"level": 1,
		"experience": 0
	}
	inventory.clear()
	equipment.clear()

func set_player_entity(player_id: int):
	player_entity_id = player_id
	emit_signal("player_spawned", player_id)

func add_entity(entity_id: int, entity_data: Dictionary):
	entities[entity_id] = entity_data
	emit_signal("entity_updated", entity_id, entity_data)

func update_entity(entity_id: int, entity_data: Dictionary):
	if entities.has(entity_id):
		entities[entity_id] = entity_data
		emit_signal("entity_updated", entity_id, entity_data)
	else:
		add_entity(entity_id, entity_data)

func remove_entity(entity_id: int):
	if entities.has(entity_id):
		entities.erase(entity_id)
		emit_signal("entity_removed", entity_id)

func get_entity(entity_id: int) -> Dictionary:
	return entities.get(entity_id, {})

func get_player_entity() -> Dictionary:
	return get_entity(player_entity_id)

func get_all_entities() -> Array:
	return entities.values()

func get_entities_in_range(center: Vector3, range: float) -> Array:
	var result = []
	for entity_data in entities.values():
		if entity_data.has("position"):
			var distance = center.distance_to(Vector3(
				entity_data.position.x,
				entity_data.position.y,
				entity_data.position.z
			))
			if distance <= range:
				result.append(entity_data)
	return result

func update_player_stats(stats: Dictionary):
	player_stats = stats

func get_player_stats() -> Dictionary:
	return player_stats

func set_zone(zone_id: int):
	if current_zone_id != zone_id:
		current_zone_id = zone_id
		emit_signal("zone_changed", zone_id)

func get_current_zone() -> int:
	return current_zone_id

func update_inventory(items: Array):
	inventory = items

func get_inventory() -> Array:
	return inventory

func update_equipment(equip_data: Dictionary):
	equipment = equip_data

func get_equipment() -> Dictionary:
	return equipment

# Utility functions
func is_player_entity(entity_id: int) -> bool:
	return entity_id == player_entity_id

func get_entity_position(entity_id: int) -> Vector3:
	var entity = get_entity(entity_id)
	if entity.has("position"):
		var pos = entity.position
		return Vector3(pos.x, pos.y, pos.z)
	return Vector3.ZERO

func set_entity_position(entity_id: int, position: Vector3):
	var entity = get_entity(entity_id)
	if not entity.is_empty():
		entity.position = {
			"x": position.x,
			"y": position.y,
			"z": position.z
		}
		update_entity(entity_id, entity)