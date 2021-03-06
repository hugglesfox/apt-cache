//! # apt-cache
//!
//! A rust crate to interface the apt-cache command.
//!
//! **Warning:** Will only work on machines with `apt` installed!
//!
//! ## Exmaple
//! ```rust
//! use apt_cache::Package;
//!
//! let git = Package::new("git").unwrap();
//! let libc = Package::new("libc6").unwrap();
//! assert!(git.depends().unwrap().contains(&libc))
//! ```

#![feature(str_strip)]
pub mod apt;

use std::path::Path;
use apt::parser::{depends, recommends, search};
use apt::{apt_cache, AptError};
use serde::{Deserialize, Serialize};
use std::io;
use std::process::{Command, Output};

#[derive(PartialEq, Deserialize, Serialize)]
pub struct Package {
    pub name: String,
}

impl Package {
    /// Create a new package
    ///
    /// Returns Err if package is not in the apt cache.
    pub fn new<T: AsRef<str>>(s: T) -> Result<Self, AptError> {
        match apt_cache("search", s.as_ref(), &search) {
            Some(p) => {
                if p.contains(&s.as_ref().to_string()) {
                    Ok(Package { name: s.as_ref().to_string() })
                } else {
                    Err(AptError::NotFound(format!(
                        "The package \"{}\" was not found",
                        s.as_ref()
                    )))
                }
            }
            None => Err(AptError::NotFound(format!(
                "The package \"{}\" was not found",
                s.as_ref()
            ))),
        }
    }

    /// Get packages marked as depends.
    pub fn depends(&self) -> Option<Vec<Package>> {
        apt_cache("depends", self.name.as_str(), &depends).and_then(|v| {
            Some(
                v.iter()
                    .map(|p| Package::new(p).expect("Error parsing dependancy"))
                    .collect::<Vec<Package>>(),
            )
        })
    }

    /// Get packages marked as recommends.
    pub fn recommends(&self) -> Option<Vec<Package>> {
        apt_cache("depends", self.name.as_str(), &recommends).and_then(|v| {
            Some(
                v.iter()
                    .map(|p| Package::new(p).expect("Error parsing dependancy"))
                    .collect::<Vec<Package>>(),
            )
        })
    }

    pub fn get_source<P: AsRef<Path>>(&self, dest: P) -> io::Result<Output> {
        Command::new("apt").arg("source").arg(self.name.as_str()).current_dir(dest).output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_pkg() -> Package {
        Package::new("bash").unwrap()
    }

    #[test]
    fn test_depends() {
        assert!(create_pkg()
            .depends()
            .unwrap()
            .contains(&Package::new("base-files").unwrap()))
    }
}
