#!/usr/bin/env bash

[ -z "$DEBUG" ] || set -x

# Display the current Pomodoro timer status in the tmux status bar.
#
# Queries the pomodoro CLI and outputs a tmux-formatted status string with
# color-coding based on the session kind:
#
#   focus + running   → red    + TICKING icon + remaining time
#   focus + paused    → yellow + TICKING icon + remaining time
#   focus + completed → green  + DONE icon
#   focus + aborted   → red    + SQUASHED icon
#   break (>=3000s)   → grey   + AWAY icon
#   break (>=600s)    → blue   + LONG_PAUSE icon + remaining time
#   break (<600s)     → blue   + SHORT_PAUSE icon + remaining time
#   none              → (empty)
#   (else/unknown)    → default + INTERNAL_INTERRUPTION icon
#
# Arguments:
#   None
# Outputs:
#   tmux-formatted Pomodoro status string
# Returns:
#   0 on success
# Dependencies:
#   - pomodoro: Command-line Pomodoro timer

# Pomicon symbols (https://github.com/gabrielelana/pomicons) via Unicode PUA codepoints.
_POMODORO_ICON_DONE=$(printf '\xEE\x80\x81')                  # U+E001 POMODORO_DONE
_POMODORO_ICON_TICKING=$(printf '\xEE\x80\x83')               # U+E003 POMODORO_TICKING
_POMODORO_ICON_SQUASHED=$(printf '\xEE\x80\x84')              # U+E004 POMODORO_SQUASHED
_POMODORO_ICON_SHORT_PAUSE=$(printf '\xEE\x80\x85')           # U+E005 SHORT_PAUSE
_POMODORO_ICON_LONG_PAUSE=$(printf '\xEE\x80\x86')            # U+E006 LONG_PAUSE
_POMODORO_ICON_AWAY=$(printf '\xEE\x80\x87')                  # U+E007 AWAY
_POMODORO_ICON_INTERNAL_INTERRUPTION=$(printf '\xEE\x80\x89') # U+E009 INTERNAL_INTERRUPTION

# MiniJinja template that embeds tmux color codes and pomicons based on session state and kind.
_POMODORO_FORMAT="\
{%- if kind == 'focus' and state == 'running' -%}\
#[fg=red]${_POMODORO_ICON_TICKING} {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}#[default]\
{%- elif kind == 'focus' and state == 'paused' -%}\
#[fg=yellow]${_POMODORO_ICON_TICKING} {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}#[default]\
{%- elif kind == 'focus' and state == 'completed' -%}\
#[fg=green]${_POMODORO_ICON_DONE}#[default]\
{%- elif kind == 'focus' and state == 'aborted' -%}\
#[fg=red]${_POMODORO_ICON_SQUASHED}#[default]\
{%- elif kind == 'break' and elapsed_secs >= 3000 -%}\
#[fg=colour8]${_POMODORO_ICON_AWAY}#[default]\
{%- elif kind == 'break' and planned_secs >= 600 -%}\
#[fg=blue]${_POMODORO_ICON_LONG_PAUSE} {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}#[default]\
{%- elif kind == 'break' -%}\
#[fg=blue]${_POMODORO_ICON_SHORT_PAUSE} {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}#[default]\
{%- else -%}\
#[fg=default]${_POMODORO_ICON_INTERNAL_INTERRUPTION}#[default]\
{%- endif -%}"

# Main entry point.
#
# Globals:
#   POMODORO_FORMAT - MiniJinja template with embedded tmux color codes
# Arguments:
#   None
# Outputs:
#   tmux-formatted status string (e.g., "#[fg=red]<icon> 20:45#[default]")
# Returns:
#   0 on success
main() {
	local status

	if ! command -v pomodoro >/dev/null 2>&1; then
		return 0
	fi

	status=$(pomodoro status --format "$_POMODORO_FORMAT" 2>/dev/null || true)

	[[ -n "$status" ]] && echo "$status"
}

main
