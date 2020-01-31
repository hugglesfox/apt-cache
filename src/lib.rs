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

use apt::parser::{depends, recommends, search};
use apt::{apt_cache, AptError};
use serde::{Deserialize, Serialize};
use std::io;
use std::process::{Command, Output};

#[derive(PartialEq, Deserialize, Serialize)]
pub struct Package {
    name: String,
}

impl Package {
    /// Create a new package
    ///
    /// Returns Err if package is not in the apt cache.
    pub fn new<T: Copy + Into<String>>(s: T) -> Result<Self, AptError> {
        match apt_cache("search", &s.into(), &search) {
            Some(p) => {
                if p.contains(&s.into()) {
                    Ok(Package { name: s.into() })
                } else {
                    Err(AptError::NotFound(format!(
                        "The package \"{}\" was not found",
                        s.into()
                    )))
                }
            }
            None => Err(AptError::NotFound(format!(
                "The package \"{}\" was not found",
                s.into()
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
