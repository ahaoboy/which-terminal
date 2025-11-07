/// macOS-specific terminal detection
use crate::platform::PlatformDetector;
use crate::types::{Terminal, TerminalInfo};
use crate::utils;
use crate::version::{VersionExtractor, env::EnvVersionExtractor, exec::ExecVersionExtractor};

pub struct MacOSDetector;

impl PlatformDetector for MacOSDetector {
    fn detect(&self) -> Option<TerminalInfo> {
        // Check TERM_PROGRAM (primary detection method on macOS)
        if let Some(term_program) = utils::get_env("TERM_PROGRAM") {
            let (terminal, version) = match term_program.as_str() {
                "Apple_Terminal" => {
                    let version = EnvVersionExtractor.extract(&Terminal::AppleTerminal);
                    (Terminal::AppleTerminal, version)
                }
                "iTerm.app" => {
                    let version = EnvVersionExtractor.extract(&Terminal::ITerm2);
                    (Terminal::ITerm2, version)
                }
                "Hyper" => {
                    let version = EnvVersionExtractor.extract(&Terminal::Hyper);
                    (Terminal::Hyper, version)
                }
                "Tabby" => {
                    let version = EnvVersionExtractor.extract(&Terminal::Tabby);
                    (Terminal::Tabby, version)
                }
                "WezTerm" => {
                    let version = EnvVersionExtractor
                        .extract(&Terminal::WezTerm)
                        .or_else(|| ExecVersionExtractor.extract(&Terminal::WezTerm));
                    (Terminal::WezTerm, version)
                }
                _ => return None,
            };

            return Some(TerminalInfo::with_version(terminal, version));
        }

        // Check for iTerm2 via session ID
        if utils::has_env("ITERM_SESSION_ID") {
            let version = EnvVersionExtractor.extract(&Terminal::ITerm2);
            return Some(TerminalInfo::with_version(Terminal::ITerm2, version));
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

        None
    }

    fn is_applicable(&self) -> bool {
        cfg!(target_os = "macos")
    }
}
