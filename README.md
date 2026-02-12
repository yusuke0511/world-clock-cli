# wclock 🌍

A beautiful real-time world clock CLI tool that displays times across major cities with elegant card-style or simple list formatting.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

## Features

- 🎨 **Two Display Modes**
  - **Card Mode**: Elegant 4-column card layout with country flags
  - **Simple Mode**: Compact list view for quick reference
- 🔄 **Real-time Updates**: Live clock updates every second
- 🌐 **Customizable**: Configure your own list of cities via config file
- 🚀 **Fast & Lightweight**: Built with Rust for performance
- 🎯 **Smart Defaults**: Includes 20 major cities out of the box
- 🛡️ **Robust**: Handles invalid timezones gracefully

## Installation

### Homebrew (Recommended)

```bash
brew tap yusuke0511/tap
brew install wclock
```

### GitHub Releases

Download pre-built binaries for your platform from [Releases](https://github.com/yusuke0511/world-clock-cli/releases/latest):
- Linux (x86_64)
- macOS (Intel & Apple Silicon)
- Windows (x86_64)

### From Source

1. Install Rust toolchain from [rustup.rs](https://rustup.rs/)

2. Clone and install:
```bash
git clone https://github.com/yusuke0511/world-clock-cli.git
cd world-clock-cli
cargo install --path .
```

### Using Cargo

```bash
cargo install world-clock-cli
```

## Usage

### Card Mode (Default)

Display times in a beautiful card layout:

```bash
wclock
```

### Simple Mode

Display times in a compact list:

```bash
wclock -s
```

### Exit

Press `Ctrl+C` to quit in either mode.

## Configuration

`wclock` looks for configuration files in the following locations (in order):

1. `./config.toml` (current directory)
2. `~/.wclock/config.toml` (user home directory)

### Example Configuration

Create `~/.wclock/config.toml`:

```toml
# wclock configuration file
# List of timezones to display (IANA timezone format)

timezones = [
    # Asia
    "Asia/Tokyo",
    "Asia/Seoul",
    "Asia/Shanghai",
    "Asia/Hong_Kong",
    "Asia/Singapore",
    "Asia/Dubai",
    
    # Europe
    "Europe/London",
    "Europe/Paris",
    "Europe/Berlin",
    "Europe/Moscow",
    
    # Americas
    "America/New_York",
    "America/Los_Angeles",
    "America/Chicago",
    "America/Toronto",
    "America/Sao_Paulo",
    "America/Mexico_City",
    
    # Oceania
    "Australia/Sydney",
    "Pacific/Auckland",
    
    # Africa
    "Africa/Cairo",
    "Africa/Johannesburg",
]
```

### Supported Timezones

Use any valid [IANA timezone identifier](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). Examples:
- `America/New_York`
- `Europe/London`
- `Asia/Tokyo`
- `Australia/Sydney`

### Country Flags

The tool includes comprehensive flag support for **550+ cities** from the IANA timezone database. Flag mappings are organized by geographic region for easy maintenance:

- **Africa**: 54 cities
- **Americas**: 168 cities  
- **Asia**: 99 cities
- **Australia**: 23 cities
- **Europe**: 64 cities
- **Pacific**: 44 cities
- **Other regions**: 100+ cities (Atlantic, Indian, Antarctica, etc.)

Cities without specific flag mappings display a 🌐 globe icon.

## Default Cities

When no config file is found, wclock displays these cities:
- Tokyo, Seoul, Singapore, Dubai (Asia)
- Moscow, London, Paris (Europe)
- New York, Los Angeles, Chicago, Toronto (Americas)
- Sydney, Auckland (Oceania)

## Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .
```

## Project Structure

```
world-clock-cli/
├── src/
│   ├── main.rs         # Entry point and CLI argument parsing
│   ├── config.rs       # Configuration file loading
│   ├── display.rs      # Display logic (card & simple modes)
│   ├── timezone.rs     # Timezone data structures
│   └── flags/          # Country flag mappings (organized by region)
│       ├── mod.rs      # Flag module integration
│       ├── africa.rs   # African cities
│       ├── america.rs  # American cities
│       ├── asia.rs     # Asian cities
│       ├── australia.rs # Australian cities
│       ├── europe.rs   # European cities
│       ├── pacific.rs  # Pacific cities
│       └── other.rs    # Other regions
├── config.toml         # Example configuration
├── Cargo.toml          # Rust dependencies
├── LICENSE             # MIT License
└── README.md           # This file
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Adding New City Flags

Flag mappings are organized by geographic region in the `src/flags/` directory. To add or update a flag:

1. Find the appropriate region file (e.g., `src/flags/asia.rs` for Asian cities)
2. Add or modify the city entry in the `get_flag()` function:

```rust
pub fn get_flag(city_name: &str) -> Option<&'static str> {
    match city_name {
        "YourCity" => Some("🇾🇨"),  // Your country flag
        // ... existing entries ...
        _ => None,
    }
}
```

3. The main `get_country_flag()` function in `src/flags/mod.rs` automatically searches all regions.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [chrono](https://github.com/chronotope/chrono) for timezone handling
- [crossterm](https://github.com/crossterm-rs/crossterm) for terminal UI
- Country flags from Unicode emoji set

---

**Made with ❤️ and Rust**
