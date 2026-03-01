#!/usr/bin/env bash
set -euo pipefail
[[ -z "${DEBUG:-}" ]] || set -x

_tmux_pomodoro_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

[[ -f "$_tmux_pomodoro_root/scripts/tmux_core.sh" ]] || {
	echo "tmux-pomodoro: missing tmux_core.sh" >&2
	exit 1
}

# shellcheck source=scripts/tmux_core.sh
source "$_tmux_pomodoro_root/scripts/tmux_core.sh"

pomodoro_status="#($_tmux_pomodoro_root/scripts/tmux_pomodoro.sh)"
pomodoro_status_pattern="\#{pomodoro_status}"

_tmux_interpolate() {
	local content=$1
	local pattern=$2
	local value=$3

	echo "${content/$pattern/$value}"
}

_tmux_bind_key() {
	local table="$1"
	local key="$2"
	local command="$3"
	tmux bind-key -T "$table" "$key" run-shell "$command > /dev/null 2>&1"
}

_tmux_bind_switch() {
	local table="$1"
	local key="$2"
	local target_table="$3"
	tmux bind-key -T "$table" "$key" switch-client -T "$target_table"
}

_tmux_update_option() {
	local option="$1"
	local option_value
	local new_option_value

	option_value="$(_tmux_get_option "$option")"
	new_option_value="$(_tmux_interpolate "$option_value" "$pomodoro_status_pattern" "$pomodoro_status")"

	_tmux_set_option "$option" "$new_option_value"
}

_tmux_setup_keybindings() {
	local pomodoro_key key_focus key_break key_stop key_focus_menu key_break_menu
	pomodoro_key="$(_tmux_get_option "@pomodoro-key" "p")"
	key_focus="$(_tmux_get_option "@pomodoro-key-focus" "f")"
	key_focus_menu="$(_tmux_get_option "@pomodoro-key-focus-menu" "F")"
	key_break="$(_tmux_get_option "@pomodoro-key-break" "b")"
	key_break_menu="$(_tmux_get_option "@pomodoro-key-break-menu" "B")"
	key_stop="$(_tmux_get_option "@pomodoro-key-stop" "s")"

	_tmux_bind_switch "prefix" "$pomodoro_key" "pomodoro"

	_tmux_bind_key "pomodoro" "$key_focus"      "$_tmux_pomodoro_root/scripts/tmux_pomodoro_cmd.sh focus 25m"
	_tmux_bind_key "pomodoro" "$key_break"      "$_tmux_pomodoro_root/scripts/tmux_pomodoro_cmd.sh break 5m"
	_tmux_bind_key "pomodoro" "$key_stop"       "$_tmux_pomodoro_root/scripts/tmux_pomodoro_cmd.sh stop"
	_tmux_bind_key "pomodoro" "$key_focus_menu" "$_tmux_pomodoro_root/scripts/tmux_pomodoro_cmd.sh focus"
	_tmux_bind_key "pomodoro" "$key_break_menu" "$_tmux_pomodoro_root/scripts/tmux_pomodoro_cmd.sh break"
}

main() {
	_tmux_update_option "status-left"
	_tmux_update_option "status-right"
	_tmux_setup_keybindings
}

main
