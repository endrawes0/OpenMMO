extends Control

# Main entry point for OpenMMO client
# This script handles UI binding only - no business logic per AGENTS.md

@onready var connect_button = $VBoxContainer/ConnectButton
@onready var server_address = $VBoxContainer/ServerAddress
@onready var username = $VBoxContainer/Username
@onready var password = $VBoxContainer/Password
@onready var exit_button = $VBoxContainer/ExitButton
@onready var network_debug = $NetworkDebug

func _ready():
	# Initialize UI state
	print("OpenMMO Client Started")
	network_debug.set_status("Disconnected")

func _on_connect_button_pressed():
	# UI binding only - actual connection logic will be handled by engine-agnostic modules
	var address = server_address.text
	var user = username.text
	var passw = password.text
	
	if address.is_empty() or user.is_empty() or passw.is_empty():
		print("Please fill in all connection fields")
		return
	
	print("Connect button pressed - UI binding only")
	print("Server: ", address)
	print("Username: ", user)
	
	# Placeholder for actual connection logic
	network_debug.set_status("Connecting...")
	network_debug.add_message("Connection initiated (UI placeholder)")

func _on_exit_button_pressed():
	print("Exit button pressed")
	get_tree().quit()