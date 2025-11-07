/// Utility functions for terminal detection
use std::env;

/// Safely retrieves an environment variable value
pub fn get_env(key: &str) -> Option<String> {
    env::var(key).ok()
}

/// Checks if an environment variable exists (regardless of value)
pub fn has_env(key: &str) -> bool {
    env::var(key).is_ok()
}
