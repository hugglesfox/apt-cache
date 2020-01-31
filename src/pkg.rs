use crate::apt::parser::{search, depends};
use crate::apt::{apt_cache, AptError};

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
}
