# pomodoro

A command-line Pomodoro timer. Sessions and events are persisted in a local
SQLite database so the timer survives process restarts.

## Installation

```sh
cargo install --path .
```

## Commands

### `start`

Start a new session or resume a paused one.

```
pomodoro start [OPTIONS]
```

| Option                      | Default        | Description                                              |
| --------------------------- | -------------- | -------------------------------------------------------- |
| `-m, --mode <MODE>`         | `focus`        | Session mode: `focus` or `break`                         |
| `-d, --duration <DURATION>` | 25 min / 5 min | Session length in humantime format (e.g. `25m`, `1h30m`) |

**Behaviour**

| Current state       | Result                  |
| ------------------- | ----------------------- |
| No session          | Starts a new session    |
| Running             | No-op (already running) |
| Paused              | Resumes the session     |
| Completed / Aborted | Starts a new session    |

**Examples**

```sh
pomodoro start                   # 25-minute focus session
pomodoro start --mode break      # 5-minute break
pomodoro start --duration 45m    # custom duration
```

---

### `stop`

Pause or abort the current session.

```
pomodoro stop [OPTIONS]
```

| Option        | Default | Description                             |
| ------------- | ------- | --------------------------------------- |
| `-r, --reset` | false   | Abort the session instead of pausing it |

**Examples**

```sh
pomodoro stop           # pause
pomodoro stop --reset   # abort
```

---

### `status`

Display the current session state.

```
pomodoro status [OPTIONS]
```

| Option                    | Default | Description                                    |
| ------------------------- | ------- | ---------------------------------------------- |
| `-o, --output <FORMAT>`   | `text`  | Output format: `text` or `json`                |
| `-f, --format <TEMPLATE>` | —       | Custom [MiniJinja] template (text output only) |

When a running session has no time left, `status` automatically records a
`completed` event.

**Text output**

The default template:

```
focus | running | elapsed 12:34 | remaining 12:26
```

Supply a custom template via `--format`:

```sh
pomodoro status --format "{{ remaining_secs }}s left"
```

Available template variables:

| Variable         | Type    | Description                                            |
| ---------------- | ------- | ------------------------------------------------------ |
| `kind`           | string  | `focus` or `break`                                     |
| `state`          | string  | `running`, `paused`, `completed`, `aborted`, or `none` |
| `planned_secs`   | integer | Planned duration in seconds                            |
| `elapsed_secs`   | integer | Elapsed time in seconds                                |
| `remaining_secs` | integer | Remaining time in seconds (clamped to 0)               |

**JSON output**

```sh
pomodoro status --output json
```

```json
{
  "kind": "focus",
  "state": "running",
  "planned_secs": 1500,
  "elapsed_secs": 300,
  "remaining_secs": 1200
}
```

---

## Configuration

Create `$XDG_CONFIG_HOME/pomodoro/config.toml` (typically
`~/.config/pomodoro/config.toml`) to override the default durations:

```toml
focus_duration = "25m"
break_duration = "5m"
```

Durations use [humantime] format (`s`, `m`, `h`, and combinations).

---

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

[MiniJinja]: https://docs.rs/minijinja
[humantime]: https://docs.rs/humantime
