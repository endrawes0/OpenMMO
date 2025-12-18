extends SceneTree

func _init():
	print("Testing OpenMMO client modules...")

	# Test loading modules
	var modules = [
		"res://client/src/networking/client_networking.gd",
		"res://client/src/gamestate/game_state_manager.gd",
		"res://client/src/movement/movement_system.gd",
		"res://client/src/input/input_manager.gd",
		"res://client/src/ui/ui_state_manager.gd"
	]

	for path in modules:
		var script = load(path)
		if script:
			var instance = script.new()
			print("✓ " + path.get_file().get_basename() + " loaded successfully")
		else:
			print("✗ Failed to load " + path)


	print("Module loading test completed")
	quit()