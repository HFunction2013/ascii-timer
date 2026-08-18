# Ascii Timer

A terminal-based countdown timer and stopwatch that displays time in large ASCII art using FIGlet fonts. Built with Rust.

## Features

- **Countdown Mode**: Count down from a specified time duration
- **Stopwatch Mode** (`-s`): Count up from zero
- **Verbose Mode** (`-v`): Display time with millisecond precision
- **ASCII Art Display**: Renders time using Toilet fonts
- **Keyboard Controls**: Press `Enter` to exit at any time
- **Signal Handling**: Gracefully handles Ctrl+C and ignores Ctrl+Z
- **Terminal Cleanup**: Automatically restores terminal state on exit

## Installation

### Prerequisites

- Rust toolchain (rustc, cargo)

### Build from source

```bash
# or `gh repo clone HFunction2013/ascii-timer`
git clone https://github.com/HFunction2013/ascii-timer.git
cd ascii-timer
cargo build --release
```

The binary will be located at `target/release/ascii-timer`.

## Usage

```
ascii-timer [OPTIONS] [TIME]
```

### Options

| Flag | Long Form | Description |
|------|-----------|-------------|
| `-v` | `--verbose` | Show time with milliseconds |
| `-s` | `--stopwatch` | Run in stopwatch mode (count up) |

### Time Format

In countdown mode, specify the duration using human-readable formats supported by the `humantime` crate:

| Format | Example | Meaning |
|--------|---------|---------|
| Plain number | `5000` | Milliseconds |
| Seconds | `30s` | 30 seconds |
| Minutes | `5m` | 5 minutes |
| Hours | `2h` | 2 hours |
| Combined | `1h30m15s` | 1 hour 30 minutes 15 seconds |

### Examples

**Countdown from 10 seconds:**
```bash
ascii-timer 10s
```

**Countdown with milliseconds display:**
```bash
ascii-timer -v 90s
```

**Stopwatch mode:**
```bash
ascii-timer -s
```

**Stopwatch with milliseconds:**
```bash
# or -sv
ascii-timer -s -v
```

**Countdown from 5 minutes:**
```bash
ascii-timer 5m
```

## Controls

- **Enter**: Exit the timer immediately
- **Ctrl+C**: Gracefully terminate the program
- **Ctrl+Z**: Ignored (prevents accidental suspension)

## Dependencies

- https://crates.io/crates/crossterm — Terminal manipulation (alternate screen, cursor hiding)
- https://crates.io/crates/figlet-rs — ASCII art font rendering via FIGlet
- https://crates.io/crates/humantime — Human-readable duration parsing
- https://crates.io/crates/libc — Unix signal handling (SIGTSTP)
- https://crates.io/crates/ctrlc — Cross-platform Ctrl+C handler

## Notes

- The program switches to an alternate terminal screen during execution and restores the original screen on exit.
- In countdown mode without `-v`, the displayed time rounds up to the nearest second.
- Requires a terminal that supports ANSI escape codes (most modern terminals do).

## License

MIT
