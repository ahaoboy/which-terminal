/// Executable query-based version extraction
use crate::types::Terminal;
use std::process::Command;

fn query_command(cmd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(cmd).args(args).output().ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Try to extract version from output
        extract_version_from_output(&stdout)
            .or_else(|| extract_version_from_output(&stderr))
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

pub(crate) fn extract(terminal: &Terminal) -> Option<String> {
    match terminal {
        Terminal::Alacritty => query_command("alacritty", &["--version"]),
        Terminal::Kitty => query_command("kitty", &["--version"]),
        Terminal::WezTerm => query_command("wezterm", &["--version"]),
        Terminal::GnomeTerminal => query_command("gnome-terminal", &["--version"]),
        Terminal::Konsole => query_command("konsole", &["--version"]),
        Terminal::XTerm => query_command("xterm", &["-version"]),
        Terminal::Tilix => query_command("tilix", &["--version"]),
        #[cfg(target_os = "windows")]
        Terminal::WindowsTerminal => {
            // Try PowerShell command to get Windows Terminal version
            query_command(
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
