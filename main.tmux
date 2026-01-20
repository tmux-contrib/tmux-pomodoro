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

# Main entry point for the plugin.
#
# Initializes the Pomodoro plugin by updating the status-right
# and status-left options to interpolate the pomodoro_status pattern.
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
}

main
