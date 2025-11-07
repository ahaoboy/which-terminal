/// Main detection coordinator that orchestrates terminal detection
use crate::platform::PlatformDetector;
use crate::types::{Terminal, TerminalInfo};
use crate::utils;

#[cfg(target_os = "windows")]
use crate::platform::windows::WindowsDetector;

#[cfg(target_os = "macos")]
use crate::platform::macos::MacOSDetector;

#[cfg(target_os = "linux")]
use crate::platform::linux::LinuxDetector;

#[cfg(target_os = "android")]
use crate::platform::android::AndroidDetector;

/// Coordinates terminal detection across different strategies
pub struct DetectionCoordinator;

impl DetectionCoordinator {
    /// Creates a new detection coordinator
    pub fn new() -> Self {
        Self
    }

    /// Performs terminal detection using a priority-based strategy
    pub fn detect(&self) -> Option<TerminalInfo> {
        // Try fast path first (common environment variables)
        if let Some(info) = self.detect_fast_path() {
            return Some(info);
        }

        // Try platform-specific detection
        if let Some(info) = self.detect_platform_specific() {
            return Some(info);
        }

        // Fall back to generic detection
        self.detect_generic()
    }

    /// Fast path detection using common environment variables
    fn detect_fast_path(&self) -> Option<TerminalInfo> {
        // Check TERM_PROGRAM (macOS and some cross-platform terminals)
        if let Some(term_program) = utils::get_env("TERM_PROGRAM") {
            let terminal = match term_program.as_str() {
                "Apple_Terminal" => Terminal::AppleTerminal,
                "iTerm.app" => Terminal::ITerm2,
                "Hyper" => Terminal::Hyper,
                "Tabby" => Terminal::Tabby,
                "WezTerm" => Terminal::WezTerm,
                _ => return None,
            };

            let version = utils::get_env("TERM_PROGRAM_VERSION");
            return Some(TerminalInfo::with_version(terminal, version));
        }

        // Check for Windows Terminal
        if utils::has_env("WT_SESSION") || utils::has_env("WT_PROFILE_ID") {
            return Some(TerminalInfo::new(Terminal::WindowsTerminal));
        }

        // Check for Termux
        if let Some(termux_version) = utils::get_env("TERMUX_VERSION") {
            return Some(TerminalInfo::with_version(
                Terminal::Termux,
                Some(termux_version),
            ));
        }

        // Check for ConEmu
        if utils::has_env("ConEmuPID") {
            return Some(TerminalInfo::new(Terminal::ConEmu));
        }

        // Check for Cmder
        if utils::has_env("CMDER_ROOT") {
            return Some(TerminalInfo::new(Terminal::Cmder));
        }

        // Check for Alacritty
        if utils::env_contains("TERM", "alacritty") {
            return Some(TerminalInfo::new(Terminal::Alacritty));
        }

        // Check for Kitty
        if utils::has_env("KITTY_WINDOW_ID") || utils::env_contains("TERM", "kitty") {
            return Some(TerminalInfo::new(Terminal::Kitty));
        }

        None
    }

    /// Platform-specific detection
    fn detect_platform_specific(&self) -> Option<TerminalInfo> {
        #[cfg(target_os = "windows")]
        {
            let detector = WindowsDetector;
            if detector.is_applicable() {
                return detector.detect();
            }
        }

        #[cfg(target_os = "macos")]
        {
            let detector = MacOSDetector;
            if detector.is_applicable() {
                return detector.detect();
            }
        }

        #[cfg(target_os = "linux")]
        {
            let detector = LinuxDetector;
            if detector.is_applicable() {
                return detector.detect();
            }
        }

        #[cfg(target_os = "android")]
        {
            let detector = AndroidDetector;
            if detector.is_applicable() {
                return detector.detect();
            }
        }

        None
    }

    /// Generic fallback detection using TERM environment variable
    fn detect_generic(&self) -> Option<TerminalInfo> {
        if let Some(term) = utils::get_env("TERM") {
            let terminal = match term.as_str() {
                "xterm" | "xterm-256color" | "xterm-color" => Terminal::XTerm,
                "rxvt" | "rxvt-unicode" | "rxvt-unicode-256color" => Terminal::Rxvt,
                _ => Terminal::Generic(term.clone()),
            };

            Some(TerminalInfo::new(terminal))
        } else {
            None
        }
    }
}
