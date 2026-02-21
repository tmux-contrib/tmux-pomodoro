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
#   break (>=600s) + running   → blue   + LONG_PAUSE icon + remaining time
#   break (>=600s) + paused    → yellow + LONG_PAUSE icon + remaining time
#   break (>=600s) + completed → green  + LONG_PAUSE icon
#   break (>=600s) + aborted   → red    + LONG_PAUSE icon
#   break (<600s)  + running (>=3000s elapsed) → grey + AWAY icon
#   break (<600s)  + running   → blue   + SHORT_PAUSE icon + remaining time
#   break (<600s)  + paused    → yellow + SHORT_PAUSE icon + remaining time
#   break (<600s)  + completed → green  + SHORT_PAUSE icon
#   break (<600s)  + aborted   → red    + SHORT_PAUSE icon
#   none              → grey   + IDLE icon
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
_POMODORO_ICON_DONE=$(printf '\xEE\x80\x81')        # U+E001 POMODORO_DONE
_POMODORO_ICON_TICKING=$(printf '\xEE\x80\x83')     # U+E003 POMODORO_TICKING
_POMODORO_ICON_SQUASHED=$(printf '\xEE\x80\x84')    # U+E004 POMODORO_SQUASHED
_POMODORO_ICON_SHORT_PAUSE=$(printf '\xEE\x80\x85') # U+E005 SHORT_PAUSE
_POMODORO_ICON_LONG_PAUSE=$(printf '\xEE\x80\x86')  # U+E006 LONG_PAUSE
_POMODORO_ICON_AWAY=$(printf '\xEE\x80\x87')        # U+E007 AWAY
_POMODORO_ICON_IDLE=$(printf '\xEF\x89\x92')        # U+F252 nf-fa-hourglass_half

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
{%- elif kind == 'break' and state == 'running' and planned_secs >= 600 -%}\
#[fg=blue]${_POMODORO_ICON_LONG_PAUSE} {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}#[default]\
{%- elif kind == 'break' and state == 'paused' and planned_secs >= 600 -%}\
#[fg=yellow]${_POMODORO_ICON_LONG_PAUSE} {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}#[default]\
{%- elif kind == 'break' and state == 'completed' and planned_secs >= 600 -%}\
#[fg=green]${_POMODORO_ICON_LONG_PAUSE}#[default]\
{%- elif kind == 'break' and state == 'aborted' and planned_secs >= 600 -%}\
#[fg=red]${_POMODORO_ICON_LONG_PAUSE}#[default]\
{%- elif kind == 'break' and state == 'running' and elapsed_secs >= 3000 -%}\
#[fg=colour8]${_POMODORO_ICON_AWAY}#[default]\
{%- elif kind == 'break' and state == 'running' -%}\
#[fg=blue]${_POMODORO_ICON_SHORT_PAUSE} {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}#[default]\
{%- elif kind == 'break' and state == 'paused' -%}\
#[fg=yellow]${_POMODORO_ICON_SHORT_PAUSE} {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}#[default]\
{%- elif kind == 'break' and state == 'completed' -%}\
#[fg=green]${_POMODORO_ICON_SHORT_PAUSE}#[default]\
{%- elif kind == 'break' and state == 'aborted' -%}\
#[fg=red]${_POMODORO_ICON_SHORT_PAUSE}#[default]\
{%- elif kind == 'none' -%}\
#[fg=default]${_POMODORO_ICON_IDLE}#[default]\
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
