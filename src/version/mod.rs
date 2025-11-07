/// Version extraction utilities
pub mod env;
pub mod exec;

use crate::types::Terminal;

/// Trait for version extraction strategies
pub trait VersionExtractor {
    /// Attempts to extract version information for the given terminal
    fn extract(&self, terminal: &Terminal) -> Option<String>;
}
