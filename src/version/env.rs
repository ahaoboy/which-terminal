/// Environment variable-based version extraction
use crate::types::Terminal;
use crate::utils;
use crate::version::VersionExtractor;

/// Extracts version information from environment variables
pub struct EnvVersionExtractor;

impl VersionExtractor for EnvVersionExtractor {
    fn extract(&self, terminal: &Terminal) -> Option<String> {
        match terminal {
            Terminal::Termux => utils::get_env("TERMUX_VERSION"),
            Terminal::Konsole => utils::get_env("KONSOLE_VERSION"),
            Terminal::AppleTerminal | Terminal::ITerm2 | Terminal::Hyper | Terminal::Tabby => {
                utils::get_env("TERM_PROGRAM_VERSION")
            }
            _ => None,
        }
    }
}
