use crate::timezone::City;
use chrono_tz::Tz;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize)]
struct Config {
    timezones: Vec<String>,
}

fn find_config_path() -> Option<PathBuf> {
    let current_config = PathBuf::from("./config.toml");
    if current_config.exists() {
        return Some(current_config);
    }

    if let Some(home_dir) = dirs::home_dir() {
        let user_config = home_dir.join(".wclock").join("config.toml");
        if user_config.exists() {
            return Some(user_config);
        }
    }

    None
}

pub fn load_cities() -> Result<Vec<City>, Box<dyn std::error::Error>> {
    let config_path = find_config_path().ok_or("No config file found")?;
    let config_str = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_str)?;

    let mut cities = Vec::new();
    for tz_str in config.timezones {
        if let Ok(timezone) = Tz::from_str(&tz_str) {
            let parts: Vec<&str> = tz_str.split('/').collect();
            let name = parts
                .last()
                .map(|s| s.replace('_', " "))
                .unwrap_or_else(|| tz_str.replace('_', " "));
            cities.push(City::new(name, timezone));
        } else {
            eprintln!("Warning: Invalid timezone '{}' in config", tz_str);
        }
    }

    Ok(cities)
}
