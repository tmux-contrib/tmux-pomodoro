#!/usr/bin/env bash
set -euo pipefail
[[ -z "${DEBUG:-}" ]] || set -x

_tmux_pomodoro_cmd_source_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

[[ -f "$_tmux_pomodoro_cmd_source_dir/tmux_core.sh" ]] || {
	echo "tmux-pomodoro: missing tmux_core.sh" >&2
	exit 1
}

# shellcheck source=tmux_core.sh
source "$_tmux_pomodoro_cmd_source_dir/tmux_core.sh"

# Absolute path to this script, used by menu items to call back into the dispatcher.
_tmux_pomodoro_cmd="$_tmux_pomodoro_cmd_source_dir/tmux_pomodoro_cmd.sh"

_tmux_display_message() {
	local message="$1"
	local display
	display="$(_tmux_get_option "@pomodoro-notify" "on")"
	if [[ "$display" == "on" ]]; then
		tmux display-message "$message"
	fi
}

_tmux_display_menu() {
	local kind="$1" start="$2" end="$3" increment="$4"
	local title
	title=" ${kind^} Duration "

	local -a args=(-x R -y S -T "$title")
	local key=1 duration
	for ((duration = start; duration <= end; duration += increment)); do
		args+=("${duration} minutes" "$key" "run-shell '$_tmux_pomodoro_cmd $kind ${duration}m > /dev/null 2>&1'")
		((key++))
	done

	tmux display-menu "${args[@]}"
}

main() {
	local session_command="$1"
	local session_duration="${2:-}"
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
				_tmux_display_menu "focus" 15 60 5
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
				_tmux_display_menu "break" 5 30 5
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
