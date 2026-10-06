# tmux-pomodoro

> A full-featured Pomodoro timer for tmux with color-coded status, chord keybindings, and interactive duration menus — powered by the **tmux-pomodoro** CLI built in this repository.

[![CI](https://github.com/tmux-contrib/tmux-pomodoro/actions/workflows/ci.yml/badge.svg)](https://github.com/tmux-contrib/tmux-pomodoro/actions/workflows/ci.yml) [![Release](https://img.shields.io/github/v/release/tmux-contrib/tmux-pomodoro)](https://github.com/tmux-contrib/tmux-pomodoro/releases) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Prerequisites

- [`tmux-pomodoro`](https://crates.io/crates/tmux-pomodoro) — the Pomodoro
  timer CLI built in this repository

### Installing tmux-pomodoro

A single binary with no runtime dependencies.

**Nix** (recommended):

```bash
nix profile install github:tmux-contrib/tmux-pomodoro
```

**Download:** each [release](https://github.com/tmux-contrib/tmux-pomodoro/releases/latest)
has a binary per platform: `tmux-pomodoro-aarch64-apple-darwin`,
`tmux-pomodoro-x86_64-apple-darwin`, `tmux-pomodoro-x86_64-unknown-linux-musl`
and `tmux-pomodoro-aarch64-unknown-linux-musl`. The Linux binaries are static
and run on any distribution.

```bash
curl -fsSL --create-dirs -o ~/.local/bin/tmux-pomodoro \
  https://github.com/tmux-contrib/tmux-pomodoro/releases/latest/download/tmux-pomodoro-aarch64-apple-darwin
chmod +x ~/.local/bin/tmux-pomodoro
```

**Cargo:**

```bash
cargo install tmux-pomodoro
```

## Installation

### Using TPM (Tmux Plugin Manager)

Add the following line to your `~/.tmux.conf`:

```tmux
set -g @plugin 'tmux-contrib/tmux-pomodoro'
```

Then press `prefix + I` to install the plugin.

### Manual Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/tmux-contrib/tmux-pomodoro ~/.tmux/plugins/tmux-pomodoro
   ```

2. Add this line to your `~/.tmux.conf`:

   ```tmux
   run-shell ~/.tmux/plugins/tmux-pomodoro/main.tmux
   ```

3. Reload tmux configuration:

   ```bash
   tmux source-file ~/.tmux.conf
   ```

## Usage

Add `#{pomodoro}` to your `status-right` or `status-left`:

```tmux
set -g status-right "#{pomodoro} | %H:%M"
```

The plugin color-codes the remaining time automatically based on the current
session state and kind:

| State | Kind  | Color   | Example output |
| ----- | ----- | ------- | -------------- |
| any   | focus | red     | ` 20:45`      |
| any   | break | blue    | ` 05:00`      |
| none  | none  | default | ` 00:00`      |

## Keybindings

Press `prefix + p` to enter the pomodoro key table, then:

| Key | Action                                      |
|-----|---------------------------------------------|
| `f` | Smart toggle: start focus / pause / resume  |
| `b` | Start a break session                       |
| `s` | Stop and reset the current session          |

The smart toggle (`f`) checks the current state:
- **running** → pauses the session (`tmux-pomodoro stop`)
- **anything else** → starts/resumes (`tmux-pomodoro start`)

### Customizing the chord prefix

The `p` key is configurable. To use a different key, set `@pomodoro-key` in
your `~/.tmux.conf` **before** the plugin loads:

```tmux
set -g @pomodoro-key "P"   # use prefix+P instead
```

> **Note:** tmux binds `prefix + p` to `previous-window` by default. This
> plugin overwrites that binding. To keep previous-window accessible, rebind it
> before loading the plugin:
>
> ```tmux
> bind-key N previous-window
> ```

### Customizing sub-keys

The three sub-keys are individually configurable via `@pomodoro-key-focus`,
`@pomodoro-key-break`, and `@pomodoro-key-stop`:

```tmux
set -g @pomodoro-key-focus "f"   # default
set -g @pomodoro-key-break "b"   # default
set -g @pomodoro-key-stop  "s"   # default
```

For example, to use Ctrl-key variants instead:

```tmux
set -g @pomodoro-key-focus "C-f"
set -g @pomodoro-key-break "C-b"
set -g @pomodoro-key-stop  "C-s"
```

### Disabling notifications

By default, each action (start, pause, resume, stop) shows a tmux status-bar
notification. To suppress all notifications, set `@pomodoro-notify` to
`off` in your `~/.tmux.conf` **before** the plugin loads:

```tmux
set -g @pomodoro-notify "off"
```

## CLI Commands

Control the timer directly from your terminal:

```bash
# Start a 25-minute focus session (default)
tmux-pomodoro start

# Start a 5-minute break session
tmux-pomodoro start --mode break

# Start a focus session with a custom duration
tmux-pomodoro start --mode focus --duration 45m

# Pause a running session
tmux-pomodoro stop

# Abort (reset) the current session
tmux-pomodoro stop --reset

# Display current status (text format)
tmux-pomodoro status

# Display current status as JSON
tmux-pomodoro status --output json

# Display with a custom MiniJinja template
tmux-pomodoro status --format "{{ kind }} | {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}"
```

### Template Variables

The `--format` flag accepts a [MiniJinja](https://docs.rs/minijinja) template. The following variables are available:

| Variable         | Type    | Description                                              | Example                                             |
| ---------------- | ------- | -------------------------------------------------------- | --------------------------------------------------- |
| `kind`           | string  | Session type                                             | `focus`, `break`, `none`                            |
| `state`          | string  | Current lifecycle state                                  | `running`, `paused`, `completed`, `aborted`, `none` |
| `planned_secs`   | integer | Planned session duration in seconds                      | `1500`                                              |
| `elapsed_secs`   | integer | Total elapsed time in seconds                            | `300`                                               |
| `remaining_secs` | integer | Time remaining in seconds (clamped to zero when expired) | `1200`                                              |

Time formatting with MiniJinja's `format` filter:

```
{{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}
```

## Configuration

Create `$XDG_CONFIG_HOME/pomodoro/config.toml` (typically
`~/.config/pomodoro/config.toml`) to override the default durations:

```toml
focus_duration = "25m"
break_duration = "5m"
```

Durations use [humantime](https://docs.rs/humantime) format (`s`, `m`, `h`, and combinations).


## Hooks

Place executable scripts in `~/.config/pomodoro/hooks/` to run custom logic
when session state changes.

| File          | Fired on                         |
| ------------- | -------------------------------- |
| `hooks/start` | `started`, `resumed`             |
| `hooks/stop`  | `paused`, `aborted`, `completed` |

Each script receives a JSON payload on **stdin**:

```json
{
  "session": {
    "id": "019612a0-...",
    "kind": "focus",
    "planned_secs": 1500,
    "created_at": "2024-01-01T10:00:00Z"
  },
  "session_event": {
    "id": "019612a1-...",
    "kind": "started",
    "session_id": "019612a0-...",
    "created_at": "2024-01-01T10:00:00Z"
  }
}
```

A missing hook file is silently skipped. Hook failures do not affect the CLI.

**`~/.config/pomodoro/hooks/start`**

```sh
#!/bin/sh

payload=$(cat)

kind=$(echo "$payload" | jq -r '.session.kind')
event=$(echo "$payload" | jq -r '.session_event.kind')

case "$event" in
  started)  say "Started a new $kind session." ;;
  resumed)  say "Resumed the $kind session." ;;
esac
```

**`~/.config/pomodoro/hooks/stop`**

```sh
#!/bin/sh

payload=$(cat)

kind=$(echo "$payload" | jq -r '.session.kind')
event=$(echo "$payload" | jq -r '.session_event.kind')

case "$event" in
  paused)    say "Paused the $kind session." ;;
  aborted)   say "Aborted the $kind session." ;;
  completed) say "The $kind session is completed." ;;
esac
```

```sh
chmod +x ~/.config/pomodoro/hooks/start ~/.config/pomodoro/hooks/stop
```

## Troubleshooting

### Status bar shows nothing

1. Check if tmux-pomodoro is installed:

   ```bash
   which tmux-pomodoro
   ```

2. Verify tmux-pomodoro works:

   ```bash
   tmux-pomodoro status
   ```

3. Start a session to test:

   ```bash
   tmux-pomodoro start
   ```

4. Reload tmux configuration:

   ```bash
   tmux source-file ~/.tmux.conf
   ```

### Icons not displaying

Your terminal may not support the Nerd Font icons or emoji used in the format
template. Ensure you have a [Nerd Font](https://www.nerdfonts.com/) installed
and configured in your terminal emulator.

### Status not updating

tmux status bars refresh based on the `status-interval` option. For
second-level accuracy:

```tmux
set -g status-interval 1
```

### Permission denied errors

Ensure the scripts are executable:

```bash
chmod +x ~/.tmux/plugins/tmux-pomodoro/main.tmux
chmod +x ~/.tmux/plugins/tmux-pomodoro/scripts/*.sh
```

## How It Works

1. The plugin registers a `#{pomodoro}` format string that tmux will interpolate
2. When tmux renders the status bar, it executes `scripts/tmux_pomodoro.sh`
3. The script queries `tmux-pomodoro status --format "<template>"` where the
   template embeds tmux color codes based on `state` and `kind`
4. The colored output is written directly to the status bar
5. If no Pomodoro is active (`state` is `none`), nothing is displayed
6. If the tmux-pomodoro CLI is not installed, nothing is displayed

## Development

### Prerequisites

Install dependencies using [Nix](https://nixos.org/):

```sh
nix develop
```

This drops you into a shell with `bash`, `tmux`, `bats`, and the Rust toolchain
pinned in `rust-toolchain.toml` (`cargo`, `rustc`, `rustfmt`, `clippy`,
`rust-analyzer`).

Or install manually: `bash`, `tmux`, `bats`, and [Rust](https://rustup.rs/).

### Running Tests

```sh
bats tests/   # plugin
cargo test    # CLI
```

### Building the CLI

```sh
# With Nix
nix build

# With Cargo
cargo install --path .
```

### Debugging

Enable trace output with the `DEBUG` environment variable:

```sh
DEBUG=1 /path/to/tmux-pomodoro/scripts/tmux_pomodoro.sh
```

## Related Projects

- [tmux-keyboard](https://github.com/tmux-contrib/tmux-keyboard) — Display
  keyboard layout in tmux
- [tmux-flow](https://github.com/tmux-contrib/tmux-flow) — Display Flow app
  status in tmux

## License

MIT License - see [LICENSE](LICENSE) file for details.
