//! Ebuild types and traits
//!
//! Main functionality of for ebuild entity

pub mod builtins;
pub mod efile;
pub mod executor;
pub mod fuzzy;
pub mod path;
pub mod pattern;
pub mod utils;
pub mod variable;

// Ebuild entity
#[derive(Debug, PartialEq)]
pub struct Ebuild {}

impl Ebuild {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let ebuild_one = Ebuild {};
        let ebuild_two = Ebuild::new();

        assert_eq!(ebuild_one, ebuild_two);
    }
}
