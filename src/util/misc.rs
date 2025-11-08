//! Miscellaneous utility functions

/// Check if the current process is running on Windows
pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

/// Check if the current process is running on Linux
pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

/// Get a human-readable platform name
pub fn platform_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else {
        "Unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = platform_name();
        assert!(platform == "Windows" || platform == "Linux" || platform == "macOS" || platform == "Unknown");

        // At least one should be true
        assert!(is_windows() || is_linux() || cfg!(target_os = "macos"));
    }
}
