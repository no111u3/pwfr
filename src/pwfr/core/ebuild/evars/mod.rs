//! Ebuild and Eclass embedded variables
//!
//! Ebuild and Eclass internal, versioning and global variables

#[derive(Debug, PartialEq)]
pub struct EVars {}

impl EVars {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let evars_one = EVars::new();
        let evars_two = EVars {};

        assert_eq!(evars_one, evars_two);
    }
}
