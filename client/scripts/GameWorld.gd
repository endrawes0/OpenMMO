extends Node3D

# Game world scene placeholder
# UI binding only - no game logic per AGENTS.md

@onready var camera = $Camera3D
@onready var player = $Player

func _ready():
	print("GameWorld scene loaded - placeholder only")
	print("Camera position: ", camera.global_position)
	print("Player position: ", player.global_position)

func _process(_delta):
	# Basic camera follow placeholder (no actual input handling)
	# Real input and movement will be handled by engine-agnostic modules
	pass

func _input(event):
	# UI binding only - placeholder for input events
	if event is InputEventKey and event.pressed:
		match event.keycode:
			KEY_ESCAPE:
				print("ESC pressed - return to menu (placeholder)")
				get_tree().change_scene_to_file("res://scenes/Main.tscn")