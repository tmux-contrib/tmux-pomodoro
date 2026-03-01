#!/usr/bin/env bash
# test_helper.bash — shared setup for BATS tests.
# Stubs out `tmux` so tmux_core.sh can be sourced without a running server.

# Stub tmux command (tests don't need a real tmux server)
tmux() { :; }
export -f tmux

# Source the library under test
TMUX_POMODORO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$TMUX_POMODORO_ROOT/scripts/tmux_core.sh"
