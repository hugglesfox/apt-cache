//! Parsers extract relevent package names each line of an output from an `apt-cache` command.

/// Parses each line of the output of `apt-cache search`
pub fn search(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}

/// Parses each line of the output of `apt-cache depends`
pub fn depends(s: &str) -> Option<&str> {
    s.trim().strip_prefix("Depends: ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(
            search("package_name long description").unwrap(),
            "package_name"
        )
    }

    #[test]
    fn test_depends() {
        assert_eq!(depends("  Depends: debianutils").unwrap(), "debianutils");
        assert!(depends("bash").is_none())
    }
}
