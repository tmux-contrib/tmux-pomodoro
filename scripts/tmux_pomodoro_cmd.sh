#!/usr/bin/env bash

# Pomodoro command dispatcher.
#
# Usage: tmux_pomodoro_cmd.sh <command>
#
# Commands:
#   focus  - Smart toggle: running → stop (pause), anything else → start
#   break  - Start a break session
#   stop   - Stop and reset the current session

case "$1" in
  focus)
    state=$(pomodoro status --format "{{ state }}" 2>/dev/null || echo "none")
    case "$state" in
      running) pomodoro stop ;;
      *)       pomodoro start --mode focus ;;
    esac
    ;;
  break)
    pomodoro start --mode break
    ;;
  stop)
    pomodoro stop --reset
    ;;
  *)
    echo "Usage: $(basename "$0") <focus|break|stop>" >&2
    exit 1
    ;;
esac
