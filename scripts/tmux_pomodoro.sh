#!/usr/bin/env bash

# Get the current Pomodoro status from openpomodoro-cli.
#
# Retrieves and displays the current Pomodoro timer status using
# a configurable format string with tmux color support.
#
# Configuration Options:
#   @pomodoro_format - Format string for openpomodoro-cli (default: "%r")
#   @pomodoro_color  - tmux color for the status (default: "red")
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
	local format
	local color
	local status

	# Check if openpomodoro-cli is installed
	if ! command -v openpomodoro-cli >/dev/null 2>&1; then
		return 0
	fi

	# Get configuration options
	color="$(_tmux_get_option "@pomodoro_color" "red")"
	format="$(_tmux_get_option "@pomodoro_format" " %r")"

	# Get status from openpomodoro-cli
	status=$(openpomodoro-cli status --format "$format" 2>/dev/null | xargs || true)

	# If no active Pomodoro, show 00:00
	if [[ -z "$status" ]]; then
		status=" 00:00"
	fi

	# Output with tmux color codes
	echo "#[fg=$color]$status#[default]"
}

main
