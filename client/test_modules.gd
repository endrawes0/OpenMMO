extends SceneTree

func _init():
	print("Testing OpenMMO client modules...")

	# Test loading modules
	var modules = [
		"res://src/networking/client_networking.gd",
		"res://src/gamestate/game_state_manager.gd",
		"res://src/movement/movement_system.gd",
		"res://src/input/input_manager.gd",
		"res://src/ui/ui_state_manager.gd"
	]

	var failures := 0
	for path in modules:
		var script = load(path)
		if script:
			if script.new():
				print("✓ " + path.get_file().get_basename() + " loaded successfully")
			else:
				failures += 1
				push_error("Failed to instantiate " + path)
		else:
			failures += 1
			push_error("Failed to load " + path)

	if failures > 0:
		print("Module loading test failed with " + str(failures) + " errors")
		quit(1)

	print("Module loading test completed")
	quit()
