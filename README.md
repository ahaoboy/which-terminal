# which-terminal

A cross-platform Rust library for detecting the current terminal emulator and its version.

[![Crates.io](https://img.shields.io/crates/v/which-terminal.svg)](https://crates.io/crates/which-terminal)
[![Documentation](https://docs.rs/which-terminal/badge.svg)](https://docs.rs/which-terminal)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 🚀 **Fast detection** - Environment-based detection with minimal overhead
- 🌍 **Cross-platform** - Supports Windows, macOS, Linux, and Android
- 📦 **Zero dependencies** - No external dependencies for core functionality
- 🔍 **20+ terminals** - Detects popular terminal emulators across all platforms
- 📝 **Version info** - Retrieves version information when available
- 🦀 **Pure Rust** - Written entirely in Rust with idiomatic APIs

## Supported Terminals

### Windows
- Windows Terminal
- Command Prompt (cmd.exe)
- PowerShell
- ConEmu
- Cmder

### macOS
- Terminal.app (Apple Terminal)
- iTerm2

### Linux
- GNOME Terminal
- Konsole
- XTerm
- Rxvt
- Terminator
- Tilix

### Cross-platform
- Alacritty
- Kitty
- Hyper
- WezTerm
- Tabby

### Android
- Termux

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
which-terminal = "0.1"
```

## Quick Start

```rust
fn main() {
    println!("{:#?}", which_terminal::which_terminal());
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Adding New Terminals

To add support for a new terminal:

1. Add the terminal variant to the `Terminal` enum in `src/types.rs`
2. Add detection logic to the appropriate platform module
3. Add version extraction if available
4. Update documentation and tests

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by various terminal detection libraries in other languages
- Thanks to all terminal emulator developers for providing detection mechanisms

## Related Projects

- [term](https://crates.io/crates/term) - Terminal formatting library
- [console](https://crates.io/crates/console) - Terminal and console abstraction
- [crossterm](https://crates.io/crates/crossterm) - Cross-platform terminal manipulation
