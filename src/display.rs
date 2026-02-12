use crate::timezone::{get_country_flag, City};
use chrono::{DateTime, Local, Utc};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use std::time::Duration;

pub fn run_clock(stdout: &mut impl Write, cities: &[City]) -> io::Result<()> {
    loop {
        // Clear the entire screen and move cursor to top-left
        execute!(stdout, terminal::Clear(ClearType::All))?;
        execute!(stdout, cursor::MoveTo(0, 0))?;

        let now: DateTime<Utc> = Utc::now();
        let local_now = Local::now();

        // header
        writeln!(stdout, "\r")?;
        writeln!(
            stdout,
            "  ╔════════════════════════════════════════════════════════════════════════════════╗\r"
        )?;
        writeln!(
            stdout,
            "  ║                          🌍  World Clock  🌍                                   ║\r"
        )?;
        writeln!(
            stdout,
            "  ╚════════════════════════════════════════════════════════════════════════════════╝\r"
        )?;

        // display current timezone
        writeln!(
            stdout,
            "  Current TimeZone: {}\r",
            local_now.format("%Y-%m-%d %H:%M:%S")
        )?;
        writeln!(stdout, "\r")?;

        // display cities in rows of 4
        let mut iter = cities.iter();
        loop {
            let city1 = iter.next();
            let city2 = iter.next();
            let city3 = iter.next();
            let city4 = iter.next();

            if city1.is_none() {
                break;
            }

            // top line
            write!(stdout, "  ┌────────────────────┐")?;
            if city2.is_some() {
                write!(stdout, "  ┌────────────────────┐")?;
            }
            if city3.is_some() {
                write!(stdout, "  ┌────────────────────┐")?;
            }
            if city4.is_some() {
                write!(stdout, "  ┌────────────────────┐")?;
            }
            writeln!(stdout, "\r")?;

            // city names
            if let Some(c1) = city1 {
                write!(
                    stdout,
                    "  │ {} {:15} │",
                    get_country_flag(&c1.name),
                    c1.name
                )?;
                if let Some(c2) = city2 {
                    write!(
                        stdout,
                        "  │ {} {:15} │",
                        get_country_flag(&c2.name),
                        c2.name
                    )?;
                    if let Some(c3) = city3 {
                        write!(
                            stdout,
                            "  │ {} {:15} │",
                            get_country_flag(&c3.name),
                            c3.name
                        )?;
                        if let Some(c4) = city4 {
                            write!(
                                stdout,
                                "  │ {} {:15} │",
                                get_country_flag(&c4.name),
                                c4.name
                            )?;
                        }
                    }
                }
            }
            writeln!(stdout, "\r")?;

            // time
            if let Some(c1) = city1 {
                let time1 = now.with_timezone(&c1.timezone);
                write!(
                    stdout,
                    "  │   {:^14}   │",
                    time1.format("%H:%M:%S").to_string()
                )?;
                if let Some(c2) = city2 {
                    let time2 = now.with_timezone(&c2.timezone);
                    write!(
                        stdout,
                        "  │   {:^14}   │",
                        time2.format("%H:%M:%S").to_string()
                    )?;
                    if let Some(c3) = city3 {
                        let time3 = now.with_timezone(&c3.timezone);
                        write!(
                            stdout,
                            "  │   {:^14}   │",
                            time3.format("%H:%M:%S").to_string()
                        )?;
                        if let Some(c4) = city4 {
                            let time4 = now.with_timezone(&c4.timezone);
                            write!(
                                stdout,
                                "  │   {:^14}   │",
                                time4.format("%H:%M:%S").to_string()
                            )?;
                        }
                    }
                }
            }
            writeln!(stdout, "\r")?;

            // date
            if let Some(c1) = city1 {
                let time1 = now.with_timezone(&c1.timezone);
                write!(
                    stdout,
                    "  │   {:^14}   │",
                    time1.format("%Y-%m-%d").to_string()
                )?;
                if let Some(c2) = city2 {
                    let time2 = now.with_timezone(&c2.timezone);
                    write!(
                        stdout,
                        "  │   {:^14}   │",
                        time2.format("%Y-%m-%d").to_string()
                    )?;
                    if let Some(c3) = city3 {
                        let time3 = now.with_timezone(&c3.timezone);
                        write!(
                            stdout,
                            "  │   {:^14}   │",
                            time3.format("%Y-%m-%d").to_string()
                        )?;
                        if let Some(c4) = city4 {
                            let time4 = now.with_timezone(&c4.timezone);
                            write!(
                                stdout,
                                "  │   {:^14}   │",
                                time4.format("%Y-%m-%d").to_string()
                            )?;
                        }
                    }
                }
            }
            writeln!(stdout, "\r")?;

            // bottom line 
            write!(stdout, "  └────────────────────┘")?;
            if city2.is_some() {
                write!(stdout, "  └────────────────────┘")?;
            }
            if city3.is_some() {
                write!(stdout, "  └────────────────────┘")?;
            }
            if city4.is_some() {
                write!(stdout, "  └────────────────────┘")?;
            }
            writeln!(stdout, "\r")?;
            writeln!(stdout, "\r")?;
        }

        writeln!(stdout, "  Press Ctrl+C to quit\r")?;
        writeln!(stdout, "\r")?;
        stdout.flush()?;

        if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            }) = event::read()?
            {
                if modifiers.contains(event::KeyModifiers::CONTROL) {
                    break;
                }
            }
        }
    }

    Ok(())
}

pub fn run_simple_clock(cities: &[City]) -> io::Result<()> {
    let mut stdout = io::stdout();

    loop {
        let now: DateTime<Utc> = Utc::now();
        let local_now = Local::now();

        // Move cursor to top and clear
        print!("\x1B[2J\x1B[H");

        // display current timezone
        println!(
            "Current TimeZone: {}\n",
            local_now.format("%Y-%m-%d %H:%M:%S")
        );

        for city in cities {
            let time = now.with_timezone(&city.timezone);
            println!(
                "{} {:15} {}",
                get_country_flag(&city.name),
                city.name,
                time.format("%Y-%m-%d %H:%M:%S")
            );
        }

        println!("\nPress Ctrl+C to quit");
        stdout.flush()?;

        std::thread::sleep(Duration::from_secs(1));

        // Check for Ctrl+C to exit
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            }) = event::read()?
            {
                if modifiers.contains(event::KeyModifiers::CONTROL) {
                    break;
                }
            }
        }
    }

    Ok(())
}
