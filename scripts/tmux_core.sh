#!/usr/bin/env bash

# Get a tmux option value.
#
# Retrieves the value of a global tmux option. If the option is not set,
# returns the provided default value.
#
# Globals:
#   None
# Arguments:
#   $1 - The name of the tmux option to retrieve
#   $2 - The default value to return if the option is not set
# Outputs:
#   The option value or default value to stdout
# Returns:
#   0 on success
_tmux_get_option() {
	local option="$1"
	local default_value="$2"
	local option_value

	option_value="$(tmux show-option -gqv "$option")"
	[[ -n "$option_value" ]] && echo "$option_value" || echo "$default_value"
}

# Set a tmux option value.
#
# Sets a global tmux option to the specified value.
#
# Globals:
#   None
# Arguments:
#   $1 - The name of the tmux option to set
#   $2 - The value to set the option to
# Returns:
#   0 on success, non-zero on failure
_tmux_set_option() {
	local option="$1"
	local value="$2"

	tmux set-option -gq "$option" "$value"
}

# Bind a key in a tmux key table to a shell command.
#
# Registers a tmux key binding in the specified key table that runs the
# specified shell command via run-shell when the key is pressed.
#
# Globals:
#   None
# Arguments:
#   $1 - Key table name (e.g., "prefix", "pomodoro")
#   $2 - The key to bind (e.g., "f", "b", "s")
#   $3 - The shell command to run when the key is pressed
# Returns:
#   0 on success, non-zero on failure
_tmux_bind_key() {
	local table="$1"
	local key="$2"
	local command="$3"
	tmux bind-key -T "$table" "$key" run-shell "$command"
}

# Bind a key in a tmux key table to switch into another key table.
#
# Registers a tmux key binding that, when pressed, switches the client into
# the specified target key table (chord entry point).
#
# Globals:
#   None
# Arguments:
#   $1 - Source key table name (e.g., "prefix")
#   $2 - The key to bind
#   $3 - The target key table to switch into
# Returns:
#   0 on success, non-zero on failure
_tmux_bind_switch() {
	local table="$1"
	local key="$2"
	local target_table="$3"
	tmux bind-key -T "$table" "$key" switch-client -T "$target_table"
}

# Interpolate a pattern in content with a value.
#
# Replaces a pattern in the given content string with the specified value.
# This is useful for substituting tmux format strings with actual values.
#
# Globals:
#   None
# Arguments:
#   $1 - The content string containing the pattern
#   $2 - The pattern to replace
#   $3 - The value to substitute for the pattern
# Outputs:
#   The content with the pattern replaced by the value
# Returns:
#   0 on success
_tmux_interpolate() {
	local content=$1
	local pattern=$2
	local value=$3

	content=${content/$pattern/$value}

	echo "$content"
}
