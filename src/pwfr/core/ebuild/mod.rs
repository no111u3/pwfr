//! Ebuild types and traits
//!
//! Main functionality of for ebuild entity

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
