/// Executable query-based version extraction
use crate::types::Terminal;
use crate::version::VersionExtractor;
use std::process::Command;

/// Extracts version information by querying terminal executables
pub struct ExecVersionExtractor;

impl ExecVersionExtractor {
    /// Attempts to get version by running a command
    fn query_command(cmd: &str, args: &[&str]) -> Option<String> {
        let output = Command::new(cmd).args(args).output().ok()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            // Try to extract version from output
            Self::extract_version_from_output(&stdout)
                .or_else(|| Self::extract_version_from_output(&stderr))
        } else {
            None
        }
    }

    /// Extracts version number from command output
    fn extract_version_from_output(text: &str) -> Option<String> {
        // Look for version patterns like "1.2.3", "v1.2.3", "version 1.2.3"
        for word in text.split_whitespace() {
            let clean = word
                .trim_start_matches('v')
                .trim_start_matches('V')
                .trim_end_matches(',')
                .trim_end_matches(';');

            // Check if it looks like a version (e.g., "1.2.3" or "1.2")
            let parts: Vec<&str> = clean.split('.').collect();
            if parts.len() >= 2
                && parts
                    .iter()
                    .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
            {
                return Some(clean.to_string());
            }
        }
        None
    }
}

impl VersionExtractor for ExecVersionExtractor {
    fn extract(&self, terminal: &Terminal) -> Option<String> {
        match terminal {
            Terminal::Alacritty => Self::query_command("alacritty", &["--version"]),
            Terminal::Kitty => Self::query_command("kitty", &["--version"]),
            Terminal::WezTerm => Self::query_command("wezterm", &["--version"]),
            Terminal::GnomeTerminal => Self::query_command("gnome-terminal", &["--version"]),
            Terminal::Konsole => Self::query_command("konsole", &["--version"]),
            Terminal::XTerm => Self::query_command("xterm", &["-version"]),
            Terminal::Tilix => Self::query_command("tilix", &["--version"]),
            #[cfg(target_os = "windows")]
            Terminal::WindowsTerminal => {
                // Try PowerShell command to get Windows Terminal version
                Self::query_command(
                    "powershell",
                    &[
                        "-NoProfile",
                        "-Command",
                        "(Get-AppxPackage Microsoft.WindowsTerminal).Version",
                    ],
                )
            }
            _ => None,
        }
    }
}
