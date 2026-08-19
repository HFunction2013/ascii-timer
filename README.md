# Terminal Timer

A terminal-based countdown timer and stopwatch that displays time in ASCII art using Toilet future font. Built with Rust.

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
# or `gh repo clone HFunction2013/terminal-timer`
git clone https://github.com/HFunction2013/terminal-timer.git
cd terminal-timer
cargo build --release
```

The binary will be located at `target/release/terminal-timer`.

## Usage

```
terminal-timer [OPTIONS] [TIME]
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
terminal-timer 10s
```

**Countdown with milliseconds display:**
```bash
terminal-timer -v 90s
```

**Stopwatch mode:**
```bash
terminal-timer -s
```

**Stopwatch with milliseconds:**
```bash
# or -sv
terminal-timer -s -v
```

**Countdown from 5 minutes:**
```bash
terminal-timer 5m
```

## Controls

- **Enter**: Exit the timer immediately
- **Ctrl+C**: Gracefully terminate the program
- **Ctrl+Z**: Ignored (prevents accidental suspension)

## Dependencies

- https://crates.io/crates/crossterm = "0.28"  — Terminal manipulation (alternate screen, cursor hiding)
- https://crates.io/crates/figlet-rs = "1.0.0" — ASCII art font rendering via FIGlet
- https://crates.io/crates/humantime = "2.1"   — Human-readable duration parsing
- https://crates.io/crates/libc = "0.2.186"    — Unix signal handling (SIGTSTP), Unix dependency only.
- https://crates.io/crates/ctrlc = "3.4"       — Cross-platform Ctrl+C handler
- https://crates.io/crates/clap = "4"          - Argument Parsing.

## Notes

- The program switches to an alternate terminal screen during execution and restores the original screen on exit.
- In countdown mode without `-v`, the displayed time rounds up to the nearest second.
- Requires a terminal that supports ANSI escape codes (most modern terminals do).

## License

MIT
