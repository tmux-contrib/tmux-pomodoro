#!/usr/bin/env bash

# Pomodoro command dispatcher.
#
# Usage: tmux_pomodoro_cmd.sh <command> [duration]
#
# Commands:
#   focus  - Toggle focus session only (start/pause/resume); warns if break is active
#   break  - Toggle break session only (start/pause/resume); warns if focus is active
#   stop   - Stop and reset the current session

_tmux_pomodoro_cmd_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$_tmux_pomodoro_cmd_dir/scripts/tmux_core.sh"

# Absolute path to this script, used by menu items to call back into the dispatcher.
_tmux_pomodoro_cmd="$_tmux_pomodoro_cmd_dir/scripts/tmux_pomodoro_cmd.sh"

# Display a tmux message if the @pomodoro-notify option is enabled.
#
# Globals:
#   None
# Arguments:
#   $1 - The message text to display
# Returns:
#   0 on success
_tmux_display_message() {
	local message="$1"
	local display
	display="$(_tmux_get_option "@pomodoro-notify" "on")"
	if [[ "$display" == "on" ]]; then
		tmux display-message "$message"
	fi
}

# Show an interactive duration picker menu for focus sessions.
#
# Each item calls back into this script with an explicit duration, so the
# selected value is passed directly to `pomodoro start --duration`.
#
# Globals:
#   _tmux_pomodoro_cmd - Absolute path to this script
# Arguments:
#   None
# Returns:
#   0 on success
_tmux_focus_menu() {
	tmux display-menu -x R -y S -T " Focus Duration " \
		"15 minutes" "1" "run-shell '$_tmux_pomodoro_cmd focus 15m > /dev/null 2>&1'" \
		"20 minutes" "2" "run-shell '$_tmux_pomodoro_cmd focus 20m > /dev/null 2>&1'" \
		"25 minutes" "3" "run-shell '$_tmux_pomodoro_cmd focus 25m > /dev/null 2>&1'" \
		"30 minutes" "4" "run-shell '$_tmux_pomodoro_cmd focus 30m > /dev/null 2>&1'" \
		"40 minutes" "5" "run-shell '$_tmux_pomodoro_cmd focus 40m > /dev/null 2>&1'" \
		"50 minutes" "6" "run-shell '$_tmux_pomodoro_cmd focus 50m > /dev/null 2>&1'" \
		"60 minutes" "7" "run-shell '$_tmux_pomodoro_cmd focus 60m > /dev/null 2>&1'"
}

# Show an interactive duration picker menu for break sessions.
#
# Each item calls back into this script with an explicit duration, so the
# selected value is passed directly to `pomodoro start --duration`.
#
# Globals:
#   _tmux_pomodoro_cmd - Absolute path to this script
# Arguments:
#   None
# Returns:
#   0 on success
_tmux_break_menu() {
	tmux display-menu -x R -y S -T " Break Duration " \
		"10 minutes" "1" "run-shell '$_tmux_pomodoro_cmd break 10m > /dev/null 2>&1'" \
		"15 minutes" "2" "run-shell '$_tmux_pomodoro_cmd break 15m > /dev/null 2>&1'" \
		"20 minutes" "3" "run-shell '$_tmux_pomodoro_cmd break 20m > /dev/null 2>&1'" \
		"30 minutes" "4" "run-shell '$_tmux_pomodoro_cmd break 30m > /dev/null 2>&1'"
}

# Main entry point for the pomodoro command dispatcher.
#
# Reads the current session state and kind once, then dispatches to the
# appropriate action based on the command and session state. For focus and
# break commands with no duration argument, shows an interactive menu.
#
# Globals:
#   _tmux_pomodoro_cmd_dir - Absolute path to the plugin root directory
# Arguments:
#   $1 - Command: focus | break | stop
#   $2 - Optional duration string (e.g. 25m, 1h30m); only used by focus and break
# Returns:
#   0 on success, 1 on invalid command
main() {
	local session_command="$1"
	local session_duration="$2"
	local session_state session_kind

	session_state=$(pomodoro status --format "{{ state }}" 2>/dev/null || echo "none")

	if [[ "$session_command" == "focus" || "$session_command" == "break" ]]; then
		session_kind=$(pomodoro status --format "{{ kind }}" 2>/dev/null || echo "none")
	fi

	case "$session_command" in
	focus)
		case "$session_state" in
		running)
			if [[ "$session_kind" == "focus" ]]; then
				_tmux_display_message "$(pomodoro stop 2>&1)"
			else
				_tmux_display_message "Cannot start focus — a break session is already in progress"
			fi
			;;
		paused)
			if [[ "$session_kind" == "focus" ]]; then
				_tmux_display_message "$(pomodoro start --mode focus 2>&1)"
			else
				_tmux_display_message "Cannot resume focus — a break session is paused"
			fi
			;;
		*)
			if [[ -n "$session_duration" ]]; then
				_tmux_display_message "$(pomodoro start --mode focus --duration "$session_duration" 2>&1)"
			else
				_tmux_focus_menu
			fi
			;;
		esac
		;;
	break)
		case "$session_state" in
		running)
			if [[ "$session_kind" == "break" ]]; then
				_tmux_display_message "$(pomodoro stop 2>&1)"
			else
				_tmux_display_message "Cannot start break — a focus session is already in progress"
			fi
			;;
		paused)
			if [[ "$session_kind" == "break" ]]; then
				_tmux_display_message "$(pomodoro start --mode break 2>&1)"
			else
				_tmux_display_message "Cannot resume break — a focus session is paused"
			fi
			;;
		*)
			if [[ -n "$session_duration" ]]; then
				_tmux_display_message "$(pomodoro start --mode break --duration "$session_duration" 2>&1)"
			else
				_tmux_break_menu
			fi
			;;
		esac
		;;
	stop)
		if [[ "$session_state" == "running" || "$session_state" == "paused" ]]; then
			_tmux_display_message "$(pomodoro stop --reset 2>&1)"
		else
			_tmux_display_message "No active session to stop"
		fi
		;;
	*)
		echo "Usage: $(basename "$0") <focus|break|stop>" >&2
		exit 1
		;;
	esac
}

main "$@"
