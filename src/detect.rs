use crate::exec::extract;
/// Unified terminal detection for all platforms
///
/// This module consolidates detection logic for all platforms to handle
/// cross-platform scenarios (e.g., SSH connections, remote terminals)
use crate::types::{Terminal, TerminalInfo};
use crate::utils;
use std::path::Path;

/// Detects the terminal using a comprehensive strategy
pub fn detect() -> Option<TerminalInfo> {
    // Try Windows-specific terminals
    if let Some(info) = detect_windows_terminals() {
        return Some(info);
    }

    // Try macOS-specific terminals
    if let Some(info) = detect_macos_terminals() {
        return Some(info);
    }

    // Try Linux-specific terminals
    if let Some(info) = detect_linux_terminals() {
        return Some(info);
    }

    // Try Android-specific terminals
    if let Some(info) = detect_android_terminals() {
        return Some(info);
    }

    // Try cross-platform terminals
    if let Some(info) = detect_cross_platform_terminals() {
        return Some(info);
    }

    // Generic fallback
    detect_generic()
}

/// Detects Windows-specific terminals
fn detect_windows_terminals() -> Option<TerminalInfo> {
    // Windows Terminal
    if utils::has_env("WT_SESSION") || utils::has_env("WT_PROFILE_ID") {
        let version = extract(&Terminal::WindowsTerminal);
        return Some(TerminalInfo::with_version(
            Terminal::WindowsTerminal,
            version,
        ));
    }

    // ConEmu
    if utils::has_env("ConEmuPID") || utils::has_env("ConEmuBuild") {
        let version = utils::get_env("ConEmuBuild");
        return Some(TerminalInfo::with_version(Terminal::ConEmu, version));
    }

    // Cmder
    if utils::has_env("CMDER_ROOT") {
        return Some(TerminalInfo::new(Terminal::Cmder));
    }

    // PowerShell
    if utils::has_env("PSModulePath") {
        return Some(TerminalInfo::new(Terminal::PowerShell));
    }

    // Command Prompt
    if utils::has_env("COMSPEC") && !utils::has_env("PSModulePath") {
        return Some(TerminalInfo::new(Terminal::CommandPrompt));
    }

    None
}

/// Detects macOS-specific terminals
fn detect_macos_terminals() -> Option<TerminalInfo> {
    // Check TERM_PROGRAM (primary detection method on macOS)
    if let Some(term_program) = utils::get_env("TERM_PROGRAM") {
        let (terminal, version) = match term_program.as_str() {
            "Apple_Terminal" => {
                let version = extract(&Terminal::AppleTerminal);
                (Terminal::AppleTerminal, version)
            }
            "iTerm.app" => {
                let version = extract(&Terminal::ITerm2);
                (Terminal::ITerm2, version)
            }
            _ => return None,
        };

        return Some(TerminalInfo::with_version(terminal, version));
    }

    // Check for iTerm2 via session ID
    if utils::has_env("ITERM_SESSION_ID") {
        let version = extract(&Terminal::ITerm2);
        return Some(TerminalInfo::with_version(Terminal::ITerm2, version));
    }

    None
}

/// Detects Linux-specific terminals
fn detect_linux_terminals() -> Option<TerminalInfo> {
    // GNOME Terminal
    if utils::has_env("GNOME_TERMINAL_SERVICE") || utils::has_env("GNOME_TERMINAL_SCREEN") {
        let version = extract(&Terminal::GnomeTerminal);
        return Some(TerminalInfo::with_version(Terminal::GnomeTerminal, version));
    }

    // Konsole
    if utils::has_env("KONSOLE_VERSION") || utils::has_env("KONSOLE_DBUS_SESSION") {
        let version = extract(&Terminal::Konsole).or_else(|| extract(&Terminal::Konsole));
        return Some(TerminalInfo::with_version(Terminal::Konsole, version));
    }

    // Terminator
    if utils::has_env("TERMINATOR_UUID") {
        return Some(TerminalInfo::new(Terminal::Terminator));
    }

    // Tilix
    if utils::has_env("TILIX_ID") {
        let version = extract(&Terminal::Tilix);
        return Some(TerminalInfo::with_version(Terminal::Tilix, version));
    }

    // XTerm
    if utils::env_contains("TERM", "xterm") && !utils::has_env("TERM_PROGRAM") {
        let version = extract(&Terminal::XTerm);
        return Some(TerminalInfo::with_version(Terminal::XTerm, version));
    }

    // Rxvt
    if utils::env_contains("TERM", "rxvt") {
        return Some(TerminalInfo::new(Terminal::Rxvt));
    }

    None
}

/// Detects Android-specific terminals
fn detect_android_terminals() -> Option<TerminalInfo> {
    // Termux via TERMUX_VERSION
    if let Some(version) = utils::get_env("TERMUX_VERSION") {
        return Some(TerminalInfo::with_version(Terminal::Termux, Some(version)));
    }

    // Termux via PREFIX
    if let Some(prefix) = utils::get_env("PREFIX") {
        if prefix.contains("com.termux") {
            let version = extract(&Terminal::Termux);
            return Some(TerminalInfo::with_version(Terminal::Termux, version));
        }
    }

    // Termux installation directory
    if Path::new("/data/data/com.termux").exists() {
        let version = extract(&Terminal::Termux);
        return Some(TerminalInfo::with_version(Terminal::Termux, version));
    }

    None
}

/// Detects cross-platform terminals (work on multiple OSes)
fn detect_cross_platform_terminals() -> Option<TerminalInfo> {
    // Check TERM_PROGRAM for cross-platform terminals
    if let Some(term_program) = utils::get_env("TERM_PROGRAM") {
        let (terminal, version) = match term_program.as_str() {
            "Hyper" => {
                let version = extract(&Terminal::Hyper);
                (Terminal::Hyper, version)
            }
            "Tabby" => {
                let version = extract(&Terminal::Tabby);
                (Terminal::Tabby, version)
            }
            "WezTerm" => {
                let version = extract(&Terminal::WezTerm).or_else(|| extract(&Terminal::WezTerm));
                (Terminal::WezTerm, version)
            }
            _ => return None,
        };

        return Some(TerminalInfo::with_version(terminal, version));
    }

    // Alacritty
    if utils::env_contains("TERM", "alacritty") {
        let version = extract(&Terminal::Alacritty);
        return Some(TerminalInfo::with_version(Terminal::Alacritty, version));
    }

    // Kitty
    if utils::has_env("KITTY_WINDOW_ID") || utils::env_contains("TERM", "kitty") {
        let version = extract(&Terminal::Kitty);
        return Some(TerminalInfo::with_version(Terminal::Kitty, version));
    }

    None
}

/// Generic fallback detection using TERM environment variable
fn detect_generic() -> Option<TerminalInfo> {
    if let Some(term) = utils::get_env("TERM") {
        let terminal = Terminal::Generic(term.clone());
        Some(TerminalInfo::new(terminal))
    } else {
        None
    }
}
