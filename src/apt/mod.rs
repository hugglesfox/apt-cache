pub mod parser;

use std::process::Command;
use std::str;

#[derive(Debug)]
pub enum AptError {
    NotFound(String),
}

/// A generic function for running and parsing apt-cache commands
pub fn apt_cache(
    cmd: &str,
    pkg: &str,
    parser: &dyn Fn(&str) -> Option<&str>,
) -> Option<Vec<String>> {
    let cmd = Command::new("apt-cache")
        .arg(cmd)
        .arg(pkg)
        .output()
        .expect("Failed to run apt");
    let output = str::from_utf8(&cmd.stdout);
    let packages: Vec<String> = output
        .unwrap()
        .lines()
        .filter_map(|pkg| parser(pkg))
        .map(|s| s.to_string())
        .collect();

    match packages.len() {
        0 => None,
        _ => Some(packages),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::parser::*;

    #[test]
    fn test_apt_cache() {
        assert!(apt_cache("search", "bash", &search)
            .unwrap()
            .contains(&"bash".to_string()));
        assert!(apt_cache("search", "does-not-exist", &search).is_none());
    }
}
