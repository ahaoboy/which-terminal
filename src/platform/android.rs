/// Android-specific terminal detection
use crate::platform::PlatformDetector;
use crate::types::{Terminal, TerminalInfo};
use crate::utils;
use crate::version::{VersionExtractor, env::EnvVersionExtractor};
use std::path::Path;

pub struct AndroidDetector;

impl PlatformDetector for AndroidDetector {
    fn detect(&self) -> Option<TerminalInfo> {
        // Check for Termux via TERMUX_VERSION
        if let Some(version) = utils::get_env("TERMUX_VERSION") {
            return Some(TerminalInfo::with_version(Terminal::Termux, Some(version)));
        }

        // Check for Termux via PREFIX environment variable
        if let Some(prefix) = utils::get_env("PREFIX") {
            if prefix.contains("com.termux") {
                let version = EnvVersionExtractor.extract(&Terminal::Termux);
                return Some(TerminalInfo::with_version(Terminal::Termux, version));
            }
        }

        // Check for Termux installation directory
        if Path::new("/data/data/com.termux").exists() {
            let version = EnvVersionExtractor.extract(&Terminal::Termux);
            return Some(TerminalInfo::with_version(Terminal::Termux, version));
        }

        None
    }

    fn is_applicable(&self) -> bool {
        cfg!(target_os = "android")
    }
}
