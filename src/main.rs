mod config;
mod display;
mod flags;
mod timezone;

use crossterm::{
    cursor, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::env;
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let simple_mode = args.contains(&"-s".to_string());

    let cities = config::load_cities().unwrap_or_else(|e| {
        eprintln!(
            "Warning: Failed to load config: {}. Using default timezones.",
            e
        );
        timezone::get_default_cities()
    });

    if cities.is_empty() {
        eprintln!("Error: No valid timezones configured.");
        return Ok(());
    }

    if simple_mode {
        display::run_simple_clock(&cities)
    } else {
        let mut stdout = stdout();
        crossterm::terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

        let result = display::run_clock(&mut stdout, &cities);

        execute!(stdout, LeaveAlternateScreen, cursor::Show)?;
        crossterm::terminal::disable_raw_mode()?;

        result
    }
}
