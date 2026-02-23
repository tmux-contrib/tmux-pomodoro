#!/usr/bin/env bash

# Pomodoro command dispatcher.
#
# Usage: tmux_pomodoro_cmd.sh <command>
#
# Commands:
#   focus  - Toggle focus session only (start/pause/resume); warns if break is active
#   break  - Toggle break session only (start/pause/resume); warns if focus is active
#   stop   - Stop and reset the current session

session_state=$(pomodoro status --format "{{ state }}" 2>/dev/null || echo "none")

if [[ "$1" == "focus" || "$1" == "break" ]]; then
  session_kind=$(pomodoro status --format "{{ kind }}" 2>/dev/null || echo "none")
fi

case "$1" in
  focus)
    case "$session_state" in
      running)
        if [[ "$session_kind" == "focus" ]]; then
          tmux display-message "$(pomodoro stop 2>&1)"
        else
          tmux display-message "Cannot start focus — a break session is already in progress"
        fi
        ;;
      paused)
        if [[ "$session_kind" == "focus" ]]; then
          tmux display-message "$(pomodoro start --mode focus 2>&1)"
        else
          tmux display-message "Cannot resume focus — a break session is paused"
        fi
        ;;
      *)
        tmux display-message "$(pomodoro start --mode focus 2>&1)"
        ;;
    esac
    ;;
  break)
    case "$session_state" in
      running)
        if [[ "$session_kind" == "break" ]]; then
          tmux display-message "$(pomodoro stop 2>&1)"
        else
          tmux display-message "Cannot start break — a focus session is already in progress"
        fi
        ;;
      paused)
        if [[ "$session_kind" == "break" ]]; then
          tmux display-message "$(pomodoro start --mode break 2>&1)"
        else
          tmux display-message "Cannot resume break — a focus session is paused"
        fi
        ;;
      *)
        tmux display-message "$(pomodoro start --mode break 2>&1)"
        ;;
    esac
    ;;
  stop)
    if [[ "$session_state" == "running" || "$session_state" == "paused" ]]; then
      tmux display-message "$(pomodoro stop --reset 2>&1)"
    else
      tmux display-message "No active session to stop"
    fi
    ;;
  *)
    echo "Usage: $(basename "$0") <focus|break|stop>" >&2
    exit 1
    ;;
esac
