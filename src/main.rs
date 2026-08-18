// Use crossterm for terminal manipulation
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
// Use figlet for ASCII art text rendering
use figlet_rs::Toilet;
// Use libc for Unix signal handling
use libc::signal;
use std::env;
use std::io::{Write, stdout};
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

// Guard struct to restore terminal state on exit
struct AtExit;
impl Drop for AtExit {
    fn drop(&mut self) {
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = AtExit;

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut verbose = false;
    let mut stopwatch = false;
    let mut time_arg = None;

    for arg in args {
        match arg.as_str() {
            "-v" | "--verbose" => verbose = true, // Verbose mode with milliseconds
            "-s" | "--stopwatch" => stopwatch = true, // Stopwatch mode
            _ => {
                if time_arg.is_none() {
                    time_arg = Some(arg.clone()); // Time argument
                }
            }
        }
    }

    // Setup Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    // Ignore SIGTSTP (Ctrl+Z)
    unsafe {
        signal(libc::SIGTSTP, libc::SIG_IGN);
    }

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?; // Switch to alternate screen

    // Load FIGlet font
    let font = Toilet::future().unwrap();

    // Thread to handle keyboard input (Enter to quit)
    let running_key = running.clone();
    std::thread::spawn(move || {
        while running_key.load(Ordering::SeqCst) {
            if event::poll(Duration::from_millis(50)).unwrap_or(false)
                && let Event::Key(key_event) =
                    event::read().unwrap_or(Event::Key(crossterm::event::KeyEvent::new(
                        KeyCode::Null,
                        crossterm::event::KeyModifiers::NONE,
                    )))
                && key_event.code == KeyCode::Enter
            {
                running_key.store(false, Ordering::SeqCst);
                break;
            }
        }
    });

    if stopwatch {
        // Stopwatch mode: count up from zero
        let start = Instant::now();
        while running.load(Ordering::SeqCst) {
            execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
            let elapsed = start.elapsed();

            if verbose {
                // Display with milliseconds precision
                let total_ms = elapsed.as_millis();
                let h = total_ms / 3_600_000;
                let m = (total_ms % 3_600_000) / 60_000;
                let s = (total_ms % 60_000) / 1000;
                let ms = total_ms % 1000;
                let t = if h > 0 {
                    format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
                } else {
                    format!("{:02}:{:02}.{:03}", m, s, ms)
                };
                print!("{}", font.convert(&t).unwrap());
                stdout.flush()?;
                std::thread::sleep(Duration::from_millis(10));
            } else {
                // Display with seconds precision
                let total_secs = elapsed.as_secs();
                let h = total_secs / 3600;
                let m = (total_secs % 3600) / 60;
                let s = total_secs % 60;
                let t = if h > 0 {
                    format!("{:02}:{:02}:{:02}", h, m, s)
                } else {
                    format!("{:02}:{:02}", m, s)
                };
                print!("{}", font.convert(&t).unwrap());
                stdout.flush()?;
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    } else {
        // Countdown mode: count down from specified time
        let time = match time_arg {
            Some(arg) => arg,
            None => {
                eprintln!("Usage: ascii-timer [-v] [-s] <TIME>");
                std::process::exit(1);
            }
        };
        // If argument is a plain number, treat it as milliseconds
        let time = if time.parse::<i32>().is_ok() {
            format!("{time}ms")
        } else {
            time
        };
        let total_duration: Duration = humantime::Duration::from_str(&time)?.into();

        let start = Instant::now();
        loop {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            let elapsed = start.elapsed();
            if elapsed >= total_duration {
                // Display "TIME'S UP!" when countdown finishes
                execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
                print!("{}", font.convert("TIME'S UP!").unwrap());
                stdout.flush()?;
                while running.load(Ordering::SeqCst) {
                    std::thread::sleep(Duration::from_millis(50));
                }
                break;
            }

            execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;
            let remaining = total_duration - elapsed;

            if verbose {
                // Display remaining time with milliseconds
                let total_ms = remaining.as_millis();
                let h = total_ms / 3_600_000;
                let m = (total_ms % 3_600_000) / 60_000;
                let s = (total_ms % 60_000) / 1000;
                let ms = total_ms % 1000;
                let t = if h > 0 {
                    format!("{:02}:{:02}:{:02}.{:03}", h, m, s, ms)
                } else {
                    format!("{:02}:{:02}.{:03}", m, s, ms)
                };
                print!("{}", font.convert(&t).unwrap());
                stdout.flush()?;
                std::thread::sleep(Duration::from_millis(10));
            } else {
                // Display remaining time rounded up to next second
                let secs = remaining.as_secs_f64().ceil() as u64;
                let h = secs / 3600;
                let m = (secs % 3600) / 60;
                let s = secs % 60;
                let t = if h > 0 {
                    format!("{:02}:{:02}:{:02}", h, m, s)
                } else {
                    format!("{:02}:{:02}", m, s)
                };
                print!("{}", font.convert(&t).unwrap());
                stdout.flush()?;
                std::thread::sleep(Duration::from_secs(1).min(remaining));
            }
        }
    }

    // Restore terminal state
    execute!(stdout, Show, LeaveAlternateScreen)?;
    Ok(())
}
