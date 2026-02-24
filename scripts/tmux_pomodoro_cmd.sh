#!/usr/bin/env bash

# Pomodoro command dispatcher.
#
# Usage: tmux_pomodoro_cmd.sh <command>
#
# Commands:
#   focus  - Toggle focus session only (start/pause/resume); warns if break is active
#   break  - Toggle break session only (start/pause/resume); warns if focus is active
#   stop   - Stop and reset the current session

_tmux_pomodoro_cmd_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$_tmux_pomodoro_cmd_dir/scripts/tmux_core.sh"

# Self command
_tmux_pomodoro_cmd="$_tmux_pomodoro_cmd_dir/scripts/tmux_pomodoro_cmd.sh"

_tmux_display_message() {
  local message="$1"
  local display
  display="$(_tmux_get_option "@pomodoro-notify" "on")"
  if [[ "$display" == "on" ]]; then
    tmux display-message "$message"
  fi
}

_tmux_focus_menu() {
  tmux display-menu -T " Focus Duration " \
    "15 minutes" "1" "run-shell '$_tmux_pomodoro_cmd focus 15m > /dev/null 2>&1'" \
    "20 minutes" "2" "run-shell '$_tmux_pomodoro_cmd focus 20m > /dev/null 2>&1'" \
    "25 minutes" "3" "run-shell '$_tmux_pomodoro_cmd focus 25m > /dev/null 2>&1'" \
    "30 minutes" "4" "run-shell '$_tmux_pomodoro_cmd focus 30m > /dev/null 2>&1'" \
    "40 minutes" "5" "run-shell '$_tmux_pomodoro_cmd focus 40m > /dev/null 2>&1'" \
    "50 minutes" "6" "run-shell '$_tmux_pomodoro_cmd focus 50m > /dev/null 2>&1'" \
    "60 minutes" "7" "run-shell '$_tmux_pomodoro_cmd focus 60m > /dev/null 2>&1'"
}

_tmux_break_menu() {
  tmux display-menu -T " Break Duration " \
    "10 minutes" "1" "run-shell '$_tmux_pomodoro_cmd break 10m > /dev/null 2>&1'" \
    "15 minutes" "2" "run-shell '$_tmux_pomodoro_cmd break 15m > /dev/null 2>&1'" \
    "20 minutes" "3" "run-shell '$_tmux_pomodoro_cmd break 20m > /dev/null 2>&1'" \
    "30 minutes" "4" "run-shell '$_tmux_pomodoro_cmd break 30m > /dev/null 2>&1'"
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
    if [[ -n "$2" ]]; then
      _tmux_display_message "$(pomodoro start --mode focus --duration "$2" 2>&1)"
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
    if [[ -n "$2" ]]; then
      _tmux_display_message "$(pomodoro start --mode break --duration "$2" 2>&1)"
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
