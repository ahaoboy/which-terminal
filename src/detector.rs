/// Main detection coordinator that orchestrates terminal detection

use crate::platform::UnifiedDetector;
use crate::types::TerminalInfo;

/// Coordinates terminal detection across different strategies
pub struct DetectionCoordinator;

impl DetectionCoordinator {
    /// Creates a new detection coordinator
    pub fn new() -> Self {
        Self
    }

    /// Performs terminal detection using the unified detector
    pub fn detect(&self) -> Option<TerminalInfo> {
        UnifiedDetector::detect()
    }
}
