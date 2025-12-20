extends Node3D

@export_file("*.tscn", "*.gltf", "*.glb") var model_scene_path := ""
@export var target_height: float = 1.8
@export var model_y_offset: float = -1.0
@export var use_female_model: bool = false
@export var walk_swing_degrees: float = 28.0
@export var idle_sway_degrees: float = 3.0
@export var walk_cycle_frequency: float = 4.2

const MALE_MODEL_PATH := "res://assets/models/Character/Superhero_Male_FullBody.gltf"
const FEMALE_MODEL_PATH := "res://assets/models/Character/Superhero_Female_FullBody.gltf"
const ANIM_LIBRARY_PATH := "res://assets/animations/universal/AnimationLibrary_Godot_Standard.glb"
const ANIM_LIBRARY_NAME := ""
const ANIM_IDLE_NAMES := ["Idle_Loop", "Idle", "A_TPose"]
const ANIM_WALK_NAMES := ["Walk_Loop", "Walk_Formal_Loop", "Walk"]
const ANIM_RUN_NAMES := ["Jog_Fwd", "Jog_Fwd_Loop", "Sprint", "Sprint_Loop", "Run"]
const ANIM_JUMP_NAMES := ["Jump", "Jump_Loop"]
const ANIM_JUMP_START_NAMES := ["Jump_Start"]
const ANIM_JUMP_LAND_NAMES := ["Jump_Land"]
const SKELETON_REL_PATH := "Armature/Skeleton3D"
var _current_clip: String = ""
var _current_state: String = "idle"

var skeleton: Skeleton3D = null
var model_root: Node3D = null
var bone_indices: Dictionary = {}
var rest_poses: Dictionary = {}
var time_accumulator: float = 0.0
var speed_blend: float = 0.0
var animation_player: AnimationPlayer = null
var animation_tree: AnimationTree = null
var _idle_move_blend_path: StringName
var _walk_run_blend_path: StringName
var _ground_air_blend_path: StringName
var _jump_start_clip: String = ""
var _jump_land_clip: String = ""
var _jump_loop_clip: String = ""
var _playing_jump_clip: bool = false
var _jump_started: bool = false
var _was_on_floor: bool = true
var _has_seen_ground: bool = false
var _off_floor_time: float = 0.0
var _on_floor_time: float = 0.0
var _ground_air_amount: float = 0.0
var _retarget_map := {
	"DEF-hips": "pelvis",
	"DEF-spine.001": "spine_01",
	"DEF-spine.002": "spine_02",
	"DEF-spine.003": "spine_03",
	"DEF-neck": "neck_01",
	"DEF-head": "Head",
	"DEF-shoulder.L": "clavicle_l",
	"DEF-upper_arm.L": "upperarm_l",
	"DEF-forearm.L": "lowerarm_l",
	"DEF-hand.L": "hand_l",
	"DEF-shoulder.R": "clavicle_r",
	"DEF-upper_arm.R": "upperarm_r",
	"DEF-forearm.R": "lowerarm_r",
	"DEF-hand.R": "hand_r",
	"DEF-thigh.L": "thigh_l",
	"DEF-shin.L": "calf_l",
	"DEF-foot.L": "foot_l",
	"DEF-toe.L": "ball_l",
	"DEF-thigh.R": "thigh_r",
	"DEF-shin.R": "calf_r",
	"DEF-foot.R": "foot_r",
	"DEF-toe.R": "ball_r"
}
var _in_air := false


func _ready() -> void:
	_spawn_model()


func update_motion(linear_velocity: Vector3, on_floor: bool, delta: float) -> void:
	if not skeleton or not animation_tree or not animation_player:
		return

	var now := _now_secs()
	var horizontal_speed: float = Vector3(linear_velocity.x, 0, linear_velocity.z).length()
	var speed_ratio: float = clamp(horizontal_speed / 6.5, 0.0, 1.5)
	speed_blend = lerp(speed_blend, speed_ratio, delta * 5.0)

	if not _has_seen_ground:
		if on_floor:
			_has_seen_ground = true
			_on_floor_time = 0.0
			_off_floor_time = 0.0
			_in_air = false
		else:
			# Wait until we touch the ground once before driving air/ground.
			return

	if on_floor:
		_on_floor_time += delta
		_off_floor_time = 0.0
	else:
		_off_floor_time += delta
		_on_floor_time = 0.0

	# Simple debounce: need sustained off-floor to enter air.
	if not _in_air and not on_floor and _off_floor_time > 0.15:
		_in_air = true
	elif _in_air and on_floor:
		_in_air = false

	_playing_jump_clip = false
	_was_on_floor = on_floor

	var target_ground_air: float = 1.0 if _in_air else 0.0
	_ground_air_amount = lerp(_ground_air_amount, target_ground_air, delta * 4.0)

	if speed_blend > 0.01:
		animation_tree.active = true
		animation_player.stop()
		_update_animation_tree()
	else:
		animation_tree.active = true
		animation_player.stop()
		_update_animation_tree()


func set_remote_movement_state(state: Dictionary) -> void:
	# Used for proxies: drive animation based on server-reported movement_state.
	if not animation_tree:
		return
	var movement_state := ""
	if state.has("movement_state"):
		movement_state = str(state.get("movement_state", "")).to_lower()
	var target_blend: float = 0.0
	if movement_state.find("walk") != -1:
		target_blend = 0.4
	elif movement_state.find("run") != -1 or movement_state.find("move") != -1:
		target_blend = 0.8
	speed_blend = target_blend
	_in_air = movement_state.find("jump") != -1
	_update_animation_tree()


func _spawn_model() -> void:
	if model_root:
		model_root.queue_free()
		model_root = null
		skeleton = null

	var path_to_use := model_scene_path
	if use_female_model or path_to_use == "":
		path_to_use = FEMALE_MODEL_PATH if use_female_model else MALE_MODEL_PATH
	var packed: PackedScene = ResourceLoader.load(path_to_use) as PackedScene
	if packed is PackedScene:
		model_root = packed.instantiate() as Node3D
	else:
		push_warning("Failed to load character model at %s" % path_to_use)
		# Attempt the alternate gender path as a fallback if available.
		var alternate := MALE_MODEL_PATH if use_female_model else FEMALE_MODEL_PATH
		if ResourceLoader.exists(alternate):
			push_warning("Falling back to alternate model path %s" % alternate)
			var fallback: PackedScene = ResourceLoader.load(alternate) as PackedScene
			if fallback:
				model_root = fallback.instantiate() as Node3D
			else:
				push_warning("Fallback load also failed at %s" % alternate)
				return
		else:
			push_warning("Alternate model path missing: %s" % alternate)
			return

	add_child(model_root)
	model_root.owner = self

	skeleton = _find_skeleton(model_root)
	if not skeleton:
		push_warning("No Skeleton3D found on the character model.")
		return

	_align_and_scale_model(model_root)
	_cache_bones()
	_cache_rest_poses()
	_setup_animation_players()
	model_root.rotation_degrees.y = 180.0
	if animation_player and not animation_player.is_connected("animation_finished", Callable(self, "_on_animation_finished")):
		animation_player.animation_finished.connect(_on_animation_finished)


func _find_skeleton(node: Node) -> Skeleton3D:
	if node is Skeleton3D:
		return node
	for child in node.get_children():
		if child is Node:
			var found := _find_skeleton(child)
			if found:
				return found
	return null


func _find_mesh_instances(node: Node) -> Array:
	var meshes: Array = []
	if node is MeshInstance3D:
		meshes.append(node)
	for child in node.get_children():
		meshes += _find_mesh_instances(child)
	return meshes


func _compute_model_bounds(root: Node3D) -> AABB:
	var has_bounds := false
	var combined := AABB()
	for mesh: MeshInstance3D in _find_mesh_instances(root):
		var aabb: AABB = mesh.get_aabb()
		if not has_bounds:
			combined = aabb
			has_bounds = true
		else:
			combined = combined.merge(aabb)
	if has_bounds:
		return combined
	return AABB()


func _align_and_scale_model(root: Node3D) -> void:
	var bounds := _compute_model_bounds(root)
	if bounds.size == Vector3.ZERO:
		return

	if target_height > 0.0 and bounds.size.y > 0.0:
		var scale_factor := target_height / bounds.size.y
		root.scale = Vector3.ONE * scale_factor
		bounds = _compute_model_bounds(root)

	var bottom := bounds.position.y
	var center := bounds.position.y + bounds.size.y * 0.5
	# Lift or lower so feet sit at y = 0 while keeping the model centered vertically.
	root.position.y -= bottom
	root.position.y += model_y_offset


func _cache_bones() -> void:
	var important_bones := [
		"thigh_l", "thigh_r",
		"calf_l", "calf_r",
		"foot_l", "foot_r",
		"upperarm_l", "upperarm_r",
		"lowerarm_l", "lowerarm_r",
		"spine_01", "spine_02", "spine_03",
		"pelvis",
		"clavicle_l", "clavicle_r",
		"ball_l", "ball_r",
		"neck_01", "Head"
	]

	for bone_label in important_bones:
		var idx := skeleton.find_bone(bone_label)
		if idx >= 0:
			bone_indices[bone_label] = idx


func _cache_rest_poses() -> void:
	for bone_name in bone_indices.keys():
		var idx: int = bone_indices[bone_name]
		rest_poses[bone_name] = _bone_rest(idx)


func _bone_rest(idx: int) -> Transform3D:
	if skeleton.has_method("get_bone_global_rest"):
		return skeleton.get_bone_global_rest(idx)
	if skeleton.has_method("get_bone_global_pose_no_override"):
		return skeleton.get_bone_global_pose_no_override(idx)
	return skeleton.get_bone_global_pose(idx)


func _setup_animation_players() -> void:
	if not animation_player:
		animation_player = AnimationPlayer.new()
		add_child(animation_player)
	animation_player.root_node = skeleton.get_path()
	animation_player.playback_active = true
	animation_player.playback_default_blend_time = 0.2
	_load_animation_library()
	_build_animation_tree()


func _load_animation_library() -> void:
	if not animation_player:
		return

	for lib_name in animation_player.get_animation_library_list():
		animation_player.remove_animation_library(lib_name)

	var anim_resource := load(ANIM_LIBRARY_PATH)
	var anim_lib := anim_resource as AnimationLibrary
	if anim_lib:
		var duplicated := anim_lib.duplicate()
		_retarget_library(duplicated)
		_loop_all_animations(duplicated)
		animation_player.add_animation_library(ANIM_LIBRARY_NAME, duplicated)
		return

	var pack: PackedScene = anim_resource as PackedScene
	if not pack:
		push_warning("Animation library not found at %s" % ANIM_LIBRARY_PATH)
		return
	var inst := pack.instantiate()
	var src_player: AnimationPlayer = _find_animation_player(inst)
	if src_player:
		for lib_name in src_player.get_animation_library_list():
			var lib := src_player.get_animation_library(lib_name)
			if lib:
				var duplicated := lib.duplicate()
				_retarget_library(duplicated)
				_loop_all_animations(duplicated)
				animation_player.add_animation_library(ANIM_LIBRARY_NAME, duplicated)
				break
	else:
		push_warning("No AnimationPlayer found in %s" % ANIM_LIBRARY_PATH)
	inst.queue_free()


func _build_animation_tree() -> void:
	if not animation_player:
		return
	if not animation_tree:
		animation_tree = AnimationTree.new()
		add_child(animation_tree)
	else:
		animation_tree.active = false
		animation_tree.tree_root = null

	var idle_anim := _find_animation(ANIM_IDLE_NAMES)
	var walk_anim := _find_animation(ANIM_WALK_NAMES)
	var run_anim := _find_animation(ANIM_RUN_NAMES)
	var jump_anim := _find_animation(ANIM_JUMP_NAMES)
	_jump_start_clip = _find_animation(ANIM_JUMP_START_NAMES)
	_jump_land_clip = _find_animation(ANIM_JUMP_LAND_NAMES)
	_jump_loop_clip = jump_anim
	if walk_anim == "":
		walk_anim = idle_anim
	if run_anim == "":
		run_anim = walk_anim
	if jump_anim == "":
		jump_anim = idle_anim

	var blend_tree := AnimationNodeBlendTree.new()

	var idle_node := AnimationNodeAnimation.new()
	idle_node.animation = idle_anim
	var walk_node := AnimationNodeAnimation.new()
	walk_node.animation = walk_anim
	var run_node := AnimationNodeAnimation.new()
	run_node.animation = run_anim
	var jump_node := AnimationNodeAnimation.new()
	jump_node.animation = jump_anim

	var walk_run_blend := AnimationNodeBlend2.new()
	var idle_move_blend := AnimationNodeBlend2.new()
	var ground_air_blend := AnimationNodeBlend2.new()
	blend_tree.add_node("idle", idle_node, Vector2(0, 0))
	blend_tree.add_node("walk", walk_node, Vector2(200, -50))
	blend_tree.add_node("run", run_node, Vector2(200, 50))
	blend_tree.add_node("move_blend", walk_run_blend, Vector2(400, 0))
	blend_tree.add_node("idle_move", idle_move_blend, Vector2(600, 0))
	blend_tree.add_node("jump", jump_node, Vector2(200, -200))
	blend_tree.add_node("ground_air", ground_air_blend, Vector2(600, -200))

	blend_tree.connect_node("move_blend", 0, "walk")
	blend_tree.connect_node("move_blend", 1, "run")
	blend_tree.connect_node("idle_move", 0, "idle")
	blend_tree.connect_node("idle_move", 1, "move_blend")
	blend_tree.connect_node("ground_air", 0, "idle_move")
	blend_tree.connect_node("ground_air", 1, "jump")
	blend_tree.connect_node("output", 0, "ground_air")

	animation_tree.anim_player = animation_tree.get_path_to(animation_player)
	animation_tree.tree_root = blend_tree
	animation_tree.active = true

	_idle_move_blend_path = StringName("parameters/idle_move/blend_amount")
	_walk_run_blend_path = StringName("parameters/move_blend/blend_amount")
	_ground_air_blend_path = StringName("parameters/ground_air/blend_amount")
	animation_tree.set(_idle_move_blend_path, 0.0)
	animation_tree.set(_walk_run_blend_path, 0.0)
	animation_tree.set(_ground_air_blend_path, 0.0)


func _find_animation(names: Array) -> String:
	if not animation_player:
		return ""
	for name in names:
		if animation_player.has_animation(name):
			return name
	var all_anims := animation_player.get_animation_list()
	for candidate in all_anims:
		var lower := String(candidate).to_lower()
		for desired in names:
			if lower.find(desired.to_lower()) != -1:
				return candidate
	if all_anims.size() > 0:
		return all_anims[0]
	return ""


func _update_animation_tree() -> void:
	if not animation_tree:
		return
	var idle_to_move: float = clamp(speed_blend, 0.0, 1.0)
	animation_tree.set(_idle_move_blend_path, idle_to_move)
	# Run clip retarget still unstable; keep fully on walk for now to avoid T-pose.
	animation_tree.set(_walk_run_blend_path, 0.0)
	animation_tree.set(_ground_air_blend_path, _ground_air_amount)


func _animation_play(anim_name: String, blend: float) -> void:
	if anim_name == "":
		return
	animation_tree.active = false
	animation_player.play(anim_name, blend)
	_current_clip = anim_name


func _play_movement_clip(speed_value: float) -> void:
	var walk := _find_animation(ANIM_WALK_NAMES)
	if walk == "":
		walk = _find_animation(ANIM_IDLE_NAMES)
	if walk == "":
		return
	_animation_play(walk, 0.05)


func _apply_rotation(bone_name: String, axis: Vector3, angle: float) -> void:
	if not rest_poses.has(bone_name):
		return
	var idx: int = bone_indices.get(bone_name, -1)
	if idx == -1:
		return

	var rest: Transform3D = rest_poses[bone_name]
	var world_axis: Vector3 = (rest.basis * axis).normalized()
	var basis := rest.basis.rotated(world_axis, angle)
	var pose := Transform3D(basis, rest.origin)
	skeleton.set_bone_global_pose_override(idx, pose, 1.0, true)


func _apply_translation(bone_name: String, offset: Vector3) -> void:
	if not rest_poses.has(bone_name):
		return
	var idx: int = bone_indices.get(bone_name, -1)
	if idx == -1:
		return

	var rest: Transform3D = rest_poses[bone_name]
	var pose := Transform3D(rest.basis, rest.origin + offset)
	skeleton.set_bone_global_pose_override(idx, pose, 1.0, true)


func _retarget_library(lib: AnimationLibrary) -> void:
	for anim_name in lib.get_animation_list():
		var anim: Animation = lib.get_animation(anim_name)
		if not anim:
			continue
		_retarget_animation(anim)


func _loop_all_animations(lib: AnimationLibrary) -> void:
	for anim_name in lib.get_animation_list():
		var anim: Animation = lib.get_animation(anim_name)
		if anim:
			anim.loop_mode = Animation.LOOP_LINEAR


func _retarget_animation(anim: Animation) -> void:
	var prefix := ""
	for track_idx in range(anim.get_track_count() - 1, -1, -1):
		var path := anim.track_get_path(track_idx)
		var path_str := str(path)
		if path_str.find(":") == -1:
			continue
		var parts := path_str.rsplit(":", true, 1)
		if parts.size() != 2:
			continue
		var bone_key := parts[1]
		var mapped: String = _map_bone_name(bone_key)
		if mapped == "":
			anim.remove_track(track_idx)
			continue
		var new_path := NodePath(":%s" % mapped)
		anim.track_set_path(track_idx, new_path)


func _map_bone_name(source: String) -> String:
	if _retarget_map.has(source):
		return _retarget_map[source]

	var lower := source.to_lower()
	lower = lower.replace("def-", "")
	lower = lower.replace(".", "_")
	lower = lower.replace(" ", "_")

	var candidates: Array = [
		lower,
		lower.replace("upper_arm", "upperarm"),
		lower.replace("forearm", "lowerarm"),
		lower.replace("shin", "calf"),
		lower.replace("toe", "ball"),
		lower.replace("shoulder", "clavicle")
	]

	for candidate in candidates:
		if skeleton.find_bone(candidate) >= 0:
			return candidate

	return ""


func _find_animation_player(node: Node) -> AnimationPlayer:
	if node is AnimationPlayer:
		return node
	for child in node.get_children():
		if child is Node:
			var found := _find_animation_player(child)
			if found:
				return found
	return null


func _restart_jump_node() -> void:
	if not animation_tree:
		return
	if not animation_player:
		return
	if _jump_loop_clip == "":
		return
	animation_tree.set("parameters/jump/time", 0.0)
	animation_tree.active = true


func _play_jump_start() -> void:
	if _jump_start_clip != "":
		if animation_tree:
			animation_tree.active = false
		animation_player.stop()
		animation_player.play(_jump_start_clip, 0.15)
		if _jump_loop_clip != "":
			animation_player.queue(_jump_loop_clip)
		_playing_jump_clip = true
		return
	_restart_jump_node()


func _play_jump_land() -> void:
	if _jump_land_clip != "":
		if animation_tree:
			animation_tree.active = false
		animation_player.stop()
		animation_player.play(_jump_land_clip, 0.15)
		_playing_jump_clip = true
		return
	animation_tree.active = true
	_playing_jump_clip = false
	_jump_started = false


func _on_animation_finished(anim_name: StringName) -> void:
	if anim_name == _jump_start_clip and _in_air:
		# Jump loop (if queued) keeps playing on AnimationPlayer while tree is off.
		pass
	elif anim_name == _jump_land_clip:
		animation_tree.active = true
		animation_player.stop()
		_playing_jump_clip = false
		_jump_started = false
		_in_air = false


func _now_secs() -> float:
	return Time.get_ticks_msec() / 1000.0
