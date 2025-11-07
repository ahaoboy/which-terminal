/// Linux-specific terminal detection
use crate::platform::PlatformDetector;
use crate::types::{Terminal, TerminalInfo};
use crate::utils;
use crate::version::{VersionExtractor, env::EnvVersionExtractor, exec::ExecVersionExtractor};

pub struct LinuxDetector;

impl PlatformDetector for LinuxDetector {
    fn detect(&self) -> Option<TerminalInfo> {
        // Check for GNOME Terminal
        if utils::has_env("GNOME_TERMINAL_SERVICE") || utils::has_env("GNOME_TERMINAL_SCREEN") {
            let version = ExecVersionExtractor.extract(&Terminal::GnomeTerminal);
            return Some(TerminalInfo::with_version(Terminal::GnomeTerminal, version));
        }

        // Check for Konsole
        if utils::has_env("KONSOLE_VERSION") || utils::has_env("KONSOLE_DBUS_SESSION") {
            let version = EnvVersionExtractor
                .extract(&Terminal::Konsole)
                .or_else(|| ExecVersionExtractor.extract(&Terminal::Konsole));
            return Some(TerminalInfo::with_version(Terminal::Konsole, version));
        }

        // Check for Terminator
        if utils::has_env("TERMINATOR_UUID") {
            return Some(TerminalInfo::new(Terminal::Terminator));
        }

        // Check for Tilix
        if utils::has_env("TILIX_ID") {
            let version = ExecVersionExtractor.extract(&Terminal::Tilix);
            return Some(TerminalInfo::with_version(Terminal::Tilix, version));
        }

        // Check for cross-platform terminals
        if utils::env_contains("TERM", "alacritty") {
            let version = ExecVersionExtractor.extract(&Terminal::Alacritty);
            return Some(TerminalInfo::with_version(Terminal::Alacritty, version));
        }

        if utils::has_env("KITTY_WINDOW_ID") || utils::env_contains("TERM", "kitty") {
            let version = ExecVersionExtractor.extract(&Terminal::Kitty);
            return Some(TerminalInfo::with_version(Terminal::Kitty, version));
        }

        if utils::env_contains("TERM_PROGRAM", "WezTerm") {
            let version = EnvVersionExtractor
                .extract(&Terminal::WezTerm)
                .or_else(|| ExecVersionExtractor.extract(&Terminal::WezTerm));
            return Some(TerminalInfo::with_version(Terminal::WezTerm, version));
        }

        if utils::env_contains("TERM_PROGRAM", "Hyper") {
            let version = EnvVersionExtractor.extract(&Terminal::Hyper);
            return Some(TerminalInfo::with_version(Terminal::Hyper, version));
        }

        if utils::env_contains("TERM_PROGRAM", "Tabby") {
            let version = EnvVersionExtractor.extract(&Terminal::Tabby);
            return Some(TerminalInfo::with_version(Terminal::Tabby, version));
        }

        // Check for XTerm
        if utils::env_contains("TERM", "xterm") {
            let version = ExecVersionExtractor.extract(&Terminal::XTerm);
            return Some(TerminalInfo::with_version(Terminal::XTerm, version));
        }

        // Check for Rxvt
        if utils::env_contains("TERM", "rxvt") {
            return Some(TerminalInfo::new(Terminal::Rxvt));
        }

        None
    }

    fn is_applicable(&self) -> bool {
        cfg!(target_os = "linux")
    }
}
