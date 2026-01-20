#!/usr/bin/env bash

[ -z "$DEBUG" ] || set -x

# Get the current Pomodoro status from openpomodoro-cli.
#
# Retrieves and displays the current Pomodoro timer status using
# a configurable format string with tmux color support.
#
# Configuration Options:
#   @pomodoro_format    - Format string for openpomodoro-cli (default: "%r")
#   @pomodoro_color     - tmux color for the status (default: "red")
#   @pomodoro_directory - Directory path for openpomodoro-cli (default: "")
#
# Globals:
#   None
# Arguments:
#   None
# Outputs:
#   The Pomodoro status formatted according to @pomodoro_format with color
# Returns:
#   0 on success
# Dependencies:
#   - openpomodoro-cli: Command-line Pomodoro timer

_tmux_source_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=tmux_core.sh
source "$_tmux_source_dir/tmux_core.sh"

# Main entry point for the Pomodoro status script.
#
# Checks if openpomodoro-cli is installed and retrieves the status
# using the configured format string, wrapped in tmux color codes.
#
# Globals:
#   None
# Arguments:
#   None
# Outputs:
#   Formatted Pomodoro status string (e.g., "#[fg=red]23:45#[default]")
# Returns:
#   0 on success
main() {
	local icon
	local none
	local color
	local status
	local format

	# Check if pomodoro is installed
	if ! command -v pomodoro >/dev/null 2>&1; then
		return 0
	fi

	# Get configuration options
	icon=""
	color="$(_tmux_get_option "@pomodoro_color" "red")"
	format="$(_tmux_get_option "@pomodoro_format" "$icon %r")"

	none="$icon 0:00"
	# Get status from openpomodoro-cli
	status=$(pomodoro status --format "$format" | xargs || true)

	# If no active Pomodoro, replace format specifiers with default values
	if [[ -z "$status" ]]; then
		status="$none"
	fi

	if [[ "$status" == "$none" ]]; then
		# No active Pomodoro
		color="default"
	fi

	# Output with tmux color codes
	echo "#[fg=$color]$status#[default]"
}

main
