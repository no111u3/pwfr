//! Ebuild types and traits
//!
//! Main functionality of for ebuild entity

use std::path::PathBuf;

pub mod builtins;
pub mod eapi;
pub mod efile;
pub mod evars;
pub mod executor;
pub mod fuzzy;
pub mod path;
pub mod pattern;
pub mod utils;
pub mod variable;

// Ebuild entity
#[derive(Debug, PartialEq)]
pub struct Ebuild {
    path: PathBuf,
}

impl Ebuild {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let ebuild_one = Ebuild {
            path: PathBuf::from("test/path"),
        };
        let ebuild_two = Ebuild::new(PathBuf::from("test/path"));

        assert_eq!(ebuild_one, ebuild_two);
    }
}
