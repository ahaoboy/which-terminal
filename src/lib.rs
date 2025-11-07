mod exec;
mod types;
mod utils;
use exec::extract;
use std::path::Path;
pub use types::{Terminal, TerminalInfo};

/// Detects the terminal using a comprehensive strategy
pub fn which_terminal() -> Option<TerminalInfo> {
    // Priority 1: Check TERM_PROGRAM first (most reliable cross-platform method)
    if let Some(info) = detect_by_term_program() {
        return Some(info);
    }

    // Priority 2: Check highly specific environment variables
    if let Some(info) = detect_by_specific_env_vars() {
        return Some(info);
    }

    // Priority 3: Check terminal-specific environment variables
    if let Some(info) = detect_by_terminal_env_vars() {
        return Some(info);
    }

    // Priority 4: Check TERM environment variable patterns
    if let Some(info) = detect_by_term_patterns() {
        return Some(info);
    }

    // Priority 5: Generic fallback
    detect_generic()
}

/// Detects terminal using TERM_PROGRAM environment variable (highest priority)
fn detect_by_term_program() -> Option<TerminalInfo> {
    let term_program = utils::get_env("TERM_PROGRAM")?;
    let version = utils::get_env("TERM_PROGRAM_VERSION");

    let terminal = match term_program.as_str() {
        // macOS native
        "Apple_Terminal" => Terminal::AppleTerminal,
        "iTerm.app" => Terminal::ITerm2,

        // Cross-platform popular terminals (sorted by popularity)
        "vscode" => Terminal::VSCode,
        "kiro" => Terminal::Kiro,
        "WezTerm" => Terminal::WezTerm,
        "Hyper" => Terminal::Hyper,
        "Tabby" => Terminal::Tabby,

        // Unknown TERM_PROGRAM
        _ => return None,
    };

    Some(TerminalInfo::with_version(terminal, version))
}

/// Detects terminal using highly specific environment variables (second priority)
/// These are unique identifiers that are very reliable
fn detect_by_specific_env_vars() -> Option<TerminalInfo> {
    // Windows Terminal (very specific)
    if utils::has_env("WT_SESSION") || utils::has_env("WT_PROFILE_ID") {
        let version = extract(&Terminal::WindowsTerminal);
        return Some(TerminalInfo::with_version(
            Terminal::WindowsTerminal,
            version,
        ));
    }

    // Termux (Android) - very specific
    if let Some(version) = utils::get_env("TERMUX_VERSION") {
        return Some(TerminalInfo::with_version(Terminal::Termux, Some(version)));
    }

    // iTerm2 session ID (macOS)
    if utils::has_env("ITERM_SESSION_ID") {
        let version = extract(&Terminal::ITerm2);
        return Some(TerminalInfo::with_version(Terminal::ITerm2, version));
    }

    // ConEmu (Windows)
    if utils::has_env("ConEmuPID") || utils::has_env("ConEmuBuild") {
        let version = utils::get_env("ConEmuBuild");
        return Some(TerminalInfo::with_version(Terminal::ConEmu, version));
    }

    // Kitty (cross-platform)
    if utils::has_env("KITTY_WINDOW_ID") {
        let version = extract(&Terminal::Kitty);
        return Some(TerminalInfo::with_version(Terminal::Kitty, version));
    }

    None
}

/// Detects terminal using terminal-specific environment variables (third priority)
fn detect_by_terminal_env_vars() -> Option<TerminalInfo> {
    // GNOME Terminal (Linux)
    if utils::has_env("GNOME_TERMINAL_SERVICE") || utils::has_env("GNOME_TERMINAL_SCREEN") {
        let version = extract(&Terminal::GnomeTerminal);
        return Some(TerminalInfo::with_version(Terminal::GnomeTerminal, version));
    }

    // Konsole (Linux/KDE)
    if utils::has_env("KONSOLE_VERSION") || utils::has_env("KONSOLE_DBUS_SESSION") {
        let version = utils::get_env("KONSOLE_VERSION").or_else(|| extract(&Terminal::Konsole));
        return Some(TerminalInfo::with_version(Terminal::Konsole, version));
    }

    // Terminator (Linux)
    if utils::has_env("TERMINATOR_UUID") {
        return Some(TerminalInfo::new(Terminal::Terminator));
    }

    // Tilix (Linux)
    if utils::has_env("TILIX_ID") {
        let version = extract(&Terminal::Tilix);
        return Some(TerminalInfo::with_version(Terminal::Tilix, version));
    }

    // Cmder (Windows)
    if utils::has_env("CMDER_ROOT") {
        return Some(TerminalInfo::new(Terminal::Cmder));
    }

    // Termux via PREFIX (Android)
    if let Some(prefix) = utils::get_env("PREFIX")
        && prefix.contains("com.termux")
    {
        let version = extract(&Terminal::Termux);
        return Some(TerminalInfo::with_version(Terminal::Termux, version));
    }

    // Termux installation directory (Android)
    if Path::new("/data/data/com.termux").exists() {
        let version = extract(&Terminal::Termux);
        return Some(TerminalInfo::with_version(Terminal::Termux, version));
    }

    None
}

/// Detects terminal using TERM environment variable patterns (fourth priority)
fn detect_by_term_patterns() -> Option<TerminalInfo> {
    let term = utils::get_env("TERM")?;

    // Alacritty
    if term.contains("alacritty") {
        let version = extract(&Terminal::Alacritty);
        return Some(TerminalInfo::with_version(Terminal::Alacritty, version));
    }

    // Kitty
    if term.contains("kitty") {
        let version = extract(&Terminal::Kitty);
        return Some(TerminalInfo::with_version(Terminal::Kitty, version));
    }

    // XTerm
    if term.contains("xterm") {
        let version = extract(&Terminal::XTerm);
        return Some(TerminalInfo::with_version(Terminal::XTerm, version));
    }

    // Rxvt
    if term.contains("rxvt") {
        return Some(TerminalInfo::new(Terminal::Rxvt));
    }

    // PowerShell (Windows) - check after TERM patterns
    if utils::has_env("PSModulePath") {
        return Some(TerminalInfo::new(Terminal::PowerShell));
    }

    // Command Prompt (Windows) - lowest priority Windows terminal
    if utils::has_env("COMSPEC") && !utils::has_env("PSModulePath") {
        return Some(TerminalInfo::new(Terminal::CommandPrompt));
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
