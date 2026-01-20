# tmux-pomodoro

A tmux plugin that integrates [openpomodoro-cli](https://github.com/open-pomodoro/openpomodoro-cli) with tmux status bar, displaying your Pomodoro timer status with customizable icons and formats.

## Prerequisites

- [openpomodoro-cli](https://github.com/open-pomodoro/openpomodoro-cli) - Command-line Pomodoro timer

### Installing openpomodoro-cli

```bash
# macOS with Homebrew
brew install open-pomodoro/tap/openpomodoro-cli

# Or download from releases
# https://github.com/open-pomodoro/openpomodoro-cli/releases
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

The plugin provides a `#{pomodoro_status}` format string that can be used in your tmux status bar configuration.

### Basic Setup

Add `#{pomodoro_status}` to your `status-right` or `status-left` option:

```tmux
set -g status-right "#{pomodoro_status} | %H:%M"
```

When a Pomodoro is active, you'll see the time remaining in red:
```
23:45 | 14:30
```

When no Pomodoro is running, it will show `00:00`.

## Configuration

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `@pomodoro_format` | `"%r"` | Format string passed to openpomodoro-cli |
| `@pomodoro_color` | `"red"` | tmux color for the status text |

The format string can include any text and icons you want, along with these variables:

| Variable | Description | Example |
|----------|-------------|---------|
| `%r` | Time remaining (mm:ss) | `23:45` |
| `%R` | Time remaining (minutes) | `23` |
| `%!r` | Time remaining with ❗️ if done | `23:45` or `❗️00:00` |
| `%c` | Completed Pomodoros today | `3` |
| `%g` | Daily goal | `8` |
| `%d` | Task description | `Write documentation` |
| `%t` | Task tags | `writing,docs` |

## Configuration Examples

### Default

The default format shows just the time remaining in red:

```tmux
# This is the default - you don't need to set it
set -g @pomodoro_format "%r"
set -g @pomodoro_color "red"
set -g status-right "#{pomodoro_status} | %H:%M"
```

Output: `23:45 | 14:30` (in red)

### With Icon and Progress

Add a tomato icon and show progress toward daily goal:

```tmux
set -g @pomodoro_format "🍅 %r %c/%g"
set -g status-right "#{pomodoro_status} | %H:%M"
```

Output: `🍅 23:45 3/8 | 14:30` (in red)

### With Alert Indicator

Use `%!r` to show an alert emoji when time is up:

```tmux
set -g @pomodoro_format "🍅 %!r %c/%g"
set -g status-right "#{pomodoro_status} | %H:%M"
```

Output: `🍅 23:45 3/8 | 14:30` or `🍅 ❗️00:00 4/8 | 14:30`

### Detailed

Show task description:

```tmux
set -g @pomodoro_format "🍅 %r [%d]"
set -g status-right "#{pomodoro_status} | %H:%M"
```

Output: `🍅 23:45 [Write documentation] | 14:30`

### ASCII-only

For terminals without emoji support:

```tmux
set -g @pomodoro_format "> %r %c/%g"
set -g status-right "#{pomodoro_status} | %H:%M"
```

Output: `> 23:45 3/8 | 14:30`

### Custom Icons and Color

Use your preferred icons and color:

```tmux
set -g @pomodoro_format "⏱ %r"
set -g @pomodoro_color "yellow"
set -g status-right "#{pomodoro_status} | %H:%M"
```

Output: `⏱ 23:45 | 14:30` (in yellow)

### Available Colors

tmux supports these color names: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, or color numbers like `colour0` through `colour255`.

### Complete Configuration Example

```tmux
# Install plugin
set -g @plugin 'tmux-contrib/tmux-pomodoro'

# Optional: Customize format and color
set -g @pomodoro_format "🍅 %r %c/%g"
set -g @pomodoro_color "red"

# Add to status bar
set -g status-right "#{pomodoro_status} | %H:%M"

# Initialize TPM (keep this at the very bottom)
run '~/.tmux/plugins/tpm/tpm'
```

## Troubleshooting

### Status bar shows nothing

1. Check if openpomodoro-cli is installed:
   ```bash
   which openpomodoro-cli
   ```

2. Verify openpomodoro-cli works:
   ```bash
   openpomodoro-cli status
   ```

3. Check if you have an active Pomodoro:
   ```bash
   openpomodoro-cli start "Test task"
   ```

4. Reload tmux configuration:
   ```bash
   tmux source-file ~/.tmux.conf
   ```

### Icons not displaying

Your terminal may not support emoji. Use ASCII characters in your format instead:

```tmux
set -g @pomodoro_format "> %r %c/%g"
```

### Status not updating

tmux status bars refresh based on the `status-interval` option. To make the Pomodoro status update more frequently:

```tmux
set -g status-interval 1  # Update every second
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
3. The script queries `openpomodoro-cli status --format "<your format>"`
4. The output is displayed in your status bar with the configured color
5. If no Pomodoro is active, it displays `00:00`
6. If openpomodoro-cli is not installed, nothing is displayed

## Related Projects

- [openpomodoro-cli](https://github.com/open-pomodoro/openpomodoro-cli) - The underlying Pomodoro timer
- [tmux-keyboard](https://github.com/tmux-contrib/tmux-keyboard) - Display keyboard layout in tmux
- [tmux-flow](https://github.com/tmux-contrib/tmux-flow) - Display Flow app status in tmux

## License

MIT License - see [LICENSE](LICENSE) file for details.
