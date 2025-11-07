mod detector;
mod platform;
mod types;
mod utils;
mod version;

pub use types::{DetectionError, Terminal, TerminalInfo};

use detector::DetectionCoordinator;

pub fn which_terminal() -> Option<TerminalInfo> {
    let coordinator = DetectionCoordinator::new();
    coordinator.detect()
}
