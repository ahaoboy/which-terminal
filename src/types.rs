use std::fmt;
use std::str::FromStr;

/// Represents known terminal emulators across different platforms
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Terminal {
    // Windows terminals
    WindowsTerminal,
    CommandPrompt,
    PowerShell,
    ConEmu,
    Cmder,

    // macOS terminals
    AppleTerminal,
    ITerm2,

    // Cross-platform terminals
    Alacritty,
    Kitty,
    Hyper,
    WezTerm,
    Tabby,

    // Linux terminals
    GnomeTerminal,
    Konsole,
    XTerm,
    Rxvt,
    Terminator,
    Tilix,

    // Android terminals
    Termux,

    // Generic/Unknown terminal with TERM value
    Generic(String),
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminal::WindowsTerminal => write!(f, "Windows Terminal"),
            Terminal::CommandPrompt => write!(f, "Command Prompt"),
            Terminal::PowerShell => write!(f, "PowerShell"),
            Terminal::ConEmu => write!(f, "ConEmu"),
            Terminal::Cmder => write!(f, "Cmder"),
            Terminal::AppleTerminal => write!(f, "Terminal.app"),
            Terminal::ITerm2 => write!(f, "iTerm2"),
            Terminal::Alacritty => write!(f, "Alacritty"),
            Terminal::Kitty => write!(f, "Kitty"),
            Terminal::Hyper => write!(f, "Hyper"),
            Terminal::WezTerm => write!(f, "WezTerm"),
            Terminal::Tabby => write!(f, "Tabby"),
            Terminal::GnomeTerminal => write!(f, "GNOME Terminal"),
            Terminal::Konsole => write!(f, "Konsole"),
            Terminal::XTerm => write!(f, "XTerm"),
            Terminal::Rxvt => write!(f, "Rxvt"),
            Terminal::Terminator => write!(f, "Terminator"),
            Terminal::Tilix => write!(f, "Tilix"),
            Terminal::Termux => write!(f, "Termux"),
            Terminal::Generic(name) => write!(f, "{}", name),
        }
    }
}

impl FromStr for Terminal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "windows terminal" | "windowsterminal" => Ok(Terminal::WindowsTerminal),
            "command prompt" | "cmd" | "cmd.exe" => Ok(Terminal::CommandPrompt),
            "powershell" | "pwsh" => Ok(Terminal::PowerShell),
            "conemu" => Ok(Terminal::ConEmu),
            "cmder" => Ok(Terminal::Cmder),
            "terminal.app" | "apple_terminal" | "apple terminal" => Ok(Terminal::AppleTerminal),
            "iterm2" | "iterm.app" | "iterm" => Ok(Terminal::ITerm2),
            "alacritty" => Ok(Terminal::Alacritty),
            "kitty" => Ok(Terminal::Kitty),
            "hyper" => Ok(Terminal::Hyper),
            "wezterm" => Ok(Terminal::WezTerm),
            "tabby" => Ok(Terminal::Tabby),
            "gnome-terminal" | "gnome terminal" => Ok(Terminal::GnomeTerminal),
            "konsole" => Ok(Terminal::Konsole),
            "xterm" | "xterm-256color" => Ok(Terminal::XTerm),
            "rxvt" | "rxvt-unicode" => Ok(Terminal::Rxvt),
            "terminator" => Ok(Terminal::Terminator),
            "tilix" => Ok(Terminal::Tilix),
            "termux" => Ok(Terminal::Termux),
            _ => Ok(Terminal::Generic(s.to_string())),
        }
    }
}

/// Complete terminal information including version and environment details
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalInfo {
    /// The detected terminal type
    pub terminal: Terminal,
    /// Optional version string
    pub version: Option<String>,
    /// Value of TERM environment variable
    pub term_env: Option<String>,
    /// Value of COLORTERM environment variable
    pub colorterm_env: Option<String>,
}

impl TerminalInfo {
    /// Creates a new TerminalInfo with the specified terminal type
    pub fn new(terminal: Terminal) -> Self {
        Self {
            terminal,
            version: None,
            term_env: std::env::var("TERM").ok(),
            colorterm_env: std::env::var("COLORTERM").ok(),
        }
    }

    /// Creates a new TerminalInfo with terminal type and version
    pub fn with_version(terminal: Terminal, version: Option<String>) -> Self {
        Self {
            terminal,
            version,
            term_env: std::env::var("TERM").ok(),
            colorterm_env: std::env::var("COLORTERM").ok(),
        }
    }
}

impl fmt::Display for TerminalInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(version) = &self.version {
            write!(f, "{} ({})", self.terminal, version)
        } else {
            write!(f, "{}", self.terminal)
        }
    }
}

impl FromStr for TerminalInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try to parse "Terminal (version)" format
        if let Some(pos) = s.find('(') {
            let terminal_str = s[..pos].trim();
            let version_str = s[pos + 1..].trim_end_matches(')').trim();
            let terminal = Terminal::from_str(terminal_str)?;
            Ok(TerminalInfo::with_version(
                terminal,
                Some(version_str.to_string()),
            ))
        } else {
            // Just terminal name
            let terminal = Terminal::from_str(s)?;
            Ok(TerminalInfo::new(terminal))
        }
    }
}

/// Detection error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectionError {
    /// No terminal could be detected
    NoTerminalDetected,
    /// Partial detection succeeded but some information is missing
    PartialDetection(String),
    /// Version query failed
    VersionQueryFailed(String),
}

impl fmt::Display for DetectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DetectionError::NoTerminalDetected => {
                write!(f, "No terminal could be detected")
            }
            DetectionError::PartialDetection(msg) => {
                write!(f, "Partial detection: {}", msg)
            }
            DetectionError::VersionQueryFailed(msg) => {
                write!(f, "Version query failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for DetectionError {}
