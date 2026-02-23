#!/usr/bin/env bash

# tmux-pomodoro plugin entry point.
#
# This plugin provides a #{pomodoro_status} format string that displays
# the current Pomodoro timer status in the tmux status bar.
#
# Usage:
#   Add #{pomodoro_status} to your status-left or status-right option.
#
# Example:
#   set -g status-right "#{pomodoro_status} | %H:%M"

_tmux_root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=scripts/tmux_core.sh
source "$_tmux_root_dir/scripts/tmux_core.sh"

pomodoro_status="#($_tmux_root_dir/scripts/tmux_pomodoro.sh)"
pomodoro_status_pattern="\#{pomodoro_status}"

# Update a tmux option by interpolating the pomodoro status pattern.
#
# Retrieves the current value of the specified tmux option, replaces any
# occurrences of #{pomodoro_status} with the actual Pomodoro status command,
# and sets the option to the new value.
#
# Globals:
#   pomodoro_status - The tmux command string to get Pomodoro status
#   pomodoro_status_pattern - The pattern to replace (#{pomodoro_status})
# Arguments:
#   $1 - The name of the tmux option to update (e.g., "status-right")
# Returns:
#   0 on success
_tmux_update_option() {
	local option="$1"
	local option_value
	local new_option_value

	option_value="$(_tmux_get_option "$option")"
	new_option_value="$(_tmux_interpolate "$option_value" "$pomodoro_status_pattern" "$pomodoro_status")"

	_tmux_set_option "$option" "$new_option_value"
}

# Register tmux keybindings for Pomodoro control.
#
# Binds a configurable chord prefix key (default: p) in the prefix table that
# switches into the "pomodoro" key table, then registers sub-keys f/b/s in
# that table for focus toggle, break, and stop/reset.
#
# Globals:
#   _tmux_root_dir - Absolute path to the plugin root directory
# Arguments:
#   None
# Returns:
#   0 on success
_tmux_setup_keybindings() {
	local pomodoro_key key_focus key_break key_stop
	pomodoro_key="$(_tmux_get_option "@pomodoro-key"       "p")"
	key_focus="$(_tmux_get_option    "@pomodoro-key-focus" "f")"
	key_break="$(_tmux_get_option    "@pomodoro-key-break" "b")"
	key_stop="$(_tmux_get_option     "@pomodoro-key-stop"  "s")"

	# Chord prefix: prefix+<pomodoro_key> enters the pomodoro table
	_tmux_bind_switch "prefix" "$pomodoro_key" "pomodoro"

	# Sub-keys inside the pomodoro table
	_tmux_bind_key "pomodoro" "$key_focus" "$_tmux_root_dir/scripts/tmux_pomodoro_cmd.sh focus"
	_tmux_bind_key "pomodoro" "$key_break" "$_tmux_root_dir/scripts/tmux_pomodoro_cmd.sh break"
	_tmux_bind_key "pomodoro" "$key_stop"  "$_tmux_root_dir/scripts/tmux_pomodoro_cmd.sh stop"
}

# Main entry point for the plugin.
#
# Initializes the Pomodoro plugin by updating the status-right
# and status-left options to interpolate the pomodoro_status pattern,
# then registers the keybindings.
#
# Globals:
#   None
# Arguments:
#   None
# Returns:
#   0 on success
main() {
	_tmux_update_option "status-left"
	_tmux_update_option "status-right"
	_tmux_setup_keybindings
}

main
