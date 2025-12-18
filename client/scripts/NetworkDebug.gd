extends Control

# Network debug overlay for monitoring connection status
# UI binding only - no actual networking logic per AGENTS.md

@onready var status_label = $VBoxContainer/StatusLabel
@onready var ping_label = $VBoxContainer/PingLabel
@onready var messages_label = $VBoxContainer/MessagesLabel

var message_count = 0
var last_ping = 0

func _ready():
	# Initialize debug display
	update_display()

func _process(_delta):
	# Update ping display periodically (placeholder)
	if Engine.get_process_frames() % 600 == 0:  # Every second at 60fps
		update_ping_placeholder()

func set_status(status: String):
	status_label.text = "Status: " + status

func get_status() -> String:
	return status_label.text.trim_prefix("Status: ")

func set_ping(ping_ms: int):
	last_ping = ping_ms
	ping_label.text = "Ping: " + str(ping_ms) + "ms"

func add_message(message: String):
	message_count += 1
	print("Network Debug: ", message)
	update_display()

func update_display():
	messages_label.text = "Messages: " + str(message_count)

func update_ping_placeholder():
	# Placeholder ping simulation - actual ping will come from network layer
	var fake_ping = randi_range(20, 50)
	set_ping(fake_ping)
