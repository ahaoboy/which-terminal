/// Windows-specific terminal detection
use crate::platform::PlatformDetector;
use crate::types::{Terminal, TerminalInfo};
use crate::utils;
use crate::version::{VersionExtractor, env::EnvVersionExtractor, exec::ExecVersionExtractor};

pub struct WindowsDetector;

impl PlatformDetector for WindowsDetector {
    fn detect(&self) -> Option<TerminalInfo> {
        // Check for Windows Terminal
        if utils::has_env("WT_SESSION") || utils::has_env("WT_PROFILE_ID") {
            let version = ExecVersionExtractor.extract(&Terminal::WindowsTerminal);
            return Some(TerminalInfo::with_version(
                Terminal::WindowsTerminal,
                version,
            ));
        }

        // Check for ConEmu
        if utils::has_env("ConEmuPID") || utils::has_env("ConEmuBuild") {
            let version = utils::get_env("ConEmuBuild");
            return Some(TerminalInfo::with_version(Terminal::ConEmu, version));
        }

        // Check for Cmder
        if utils::has_env("CMDER_ROOT") {
            return Some(TerminalInfo::new(Terminal::Cmder));
        }

        // Check for cross-platform terminals on Windows
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

        // Check for PowerShell
        if utils::has_env("PSModulePath") {
            // Distinguish between PowerShell and Command Prompt
            if utils::get_env("PROMPT").is_some() || utils::get_env("COMSPEC").is_some() {
                return Some(TerminalInfo::new(Terminal::PowerShell));
            }
        }

        // Fallback to Command Prompt if COMSPEC is set
        if utils::has_env("COMSPEC") {
            return Some(TerminalInfo::new(Terminal::CommandPrompt));
        }

        None
    }

    fn is_applicable(&self) -> bool {
        cfg!(target_os = "windows")
    }
}
