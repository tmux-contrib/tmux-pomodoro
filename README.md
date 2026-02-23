# tmux-pomodoro

A tmux plugin that integrates with the **pomodoro** CLI to display your
Pomodoro timer status in the tmux status bar with automatic color-coding based
on session state and type.

## Prerequisites

- [pomodoro](/crates/pomodoro/README.md) — the Pomodoro
  timer CLI built in this repository

### Installing pomodoro

Build and install the CLI from source using Cargo:

```bash
cargo install --path crates/pomodoro
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

Add `#{pomodoro_status}` to your `status-right` or `status-left`:

```tmux
set -g status-right "#{pomodoro_status} | %H:%M"
```

The plugin color-codes the remaining time automatically based on the current
session state and kind:

| State | Kind  | Color   | Example output |
| ----- | ----- | ------- | -------------- |
| any   | focus | red     | ` 20:45`      |
| any   | break | blue    | ` 05:00`      |
| none  | none  | default | ` 00:00`      |

## Keybindings

Press `prefix + P` to enter the pomodoro key table, then:

| Key | Action                                      |
|-----|---------------------------------------------|
| `f` | Smart toggle: start focus / pause / resume  |
| `b` | Start a break session                       |
| `s` | Stop and reset the current session          |

The smart toggle (`f`) checks the current state:
- **running** → pauses the session (`pomodoro stop`)
- **anything else** → starts/resumes (`pomodoro start`)

### Customizing the chord prefix

The `P` key is configurable. To use a different key, set `@pomodoro-key` in
your `~/.tmux.conf` **before** the plugin loads:

```tmux
set -g @pomodoro-key "p"   # use prefix+p instead
```

## CLI Commands

Control the timer directly from your terminal:

```bash
# Start a 25-minute focus session (default)
pomodoro start

# Start a 5-minute break session
pomodoro start --mode break

# Start a focus session with a custom duration
pomodoro start --mode focus --duration 45m

# Pause a running session
pomodoro stop

# Abort (reset) the current session
pomodoro stop --reset

# Display current status (text format)
pomodoro status

# Display current status as JSON
pomodoro status --output json

# Display with a custom MiniJinja template
pomodoro status --format "{{ kind }} | {{ '%02d:%02d' | format(remaining_secs // 60, remaining_secs % 60) }}"
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

## Troubleshooting

### Status bar shows nothing

1. Check if pomodoro is installed:

   ```bash
   which pomodoro
   ```

2. Verify pomodoro works:

   ```bash
   pomodoro status
   ```

3. Start a session to test:

   ```bash
   pomodoro start
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

1. The plugin registers a `#{pomodoro_status}` format string that tmux will interpolate
2. When tmux renders the status bar, it executes `scripts/tmux_pomodoro.sh`
3. The script queries `pomodoro status --format "<template>"` where the
   template embeds tmux color codes based on `state` and `kind`
4. The colored output is written directly to the status bar
5. If no Pomodoro is active (`state` is `none`), nothing is displayed
6. If the pomodoro CLI is not installed, nothing is displayed

## Related Projects

- [tmux-keyboard](https://github.com/tmux-contrib/tmux-keyboard) — Display
  keyboard layout in tmux
- [tmux-flow](https://github.com/tmux-contrib/tmux-flow) — Display Flow app
  status in tmux

## License

MIT License - see [LICENSE](LICENSE) file for details.
