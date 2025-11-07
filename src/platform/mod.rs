/// Platform-specific terminal detection modules
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "android")]
pub mod android;

use crate::types::TerminalInfo;

/// Trait for platform-specific terminal detectors
pub trait PlatformDetector {
    /// Attempts to detect the terminal on this platform
    fn detect(&self) -> Option<TerminalInfo>;

    /// Checks if this detector is applicable for the current platform
    fn is_applicable(&self) -> bool;
}
