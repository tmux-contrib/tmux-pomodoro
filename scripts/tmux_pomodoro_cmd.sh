#!/usr/bin/env bash

# Pomodoro command dispatcher.
#
# Usage: tmux_pomodoro_cmd.sh <command>
#
# Commands:
#   focus  - Toggle focus session only (start/pause/resume); warns if break is active
#   break  - Toggle break session only (start/pause/resume); warns if focus is active
#   stop   - Stop and reset the current session

_tmux_root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$_tmux_root_dir/scripts/tmux_core.sh"

_tmux_display_message() {
  local message="$1"
  local display
  display="$(_tmux_get_option "@pomodoro-notify" "on")"
  [[ "$display" == "on" ]] && tmux display-message "$message"
}

session_state=$(pomodoro status --format "{{ state }}" 2>/dev/null || echo "none")

if [[ "$1" == "focus" || "$1" == "break" ]]; then
  session_kind=$(pomodoro status --format "{{ kind }}" 2>/dev/null || echo "none")
fi

case "$1" in
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
    _tmux_display_message "$(pomodoro start --mode focus 2>&1)"
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
    _tmux_display_message "$(pomodoro start --mode break 2>&1)"
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
