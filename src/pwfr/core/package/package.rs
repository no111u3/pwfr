//! Package unit
//!
//! Package entity

use crate::core::package::slot::Slot;

#[derive(Debug, PartialEq)]
pub struct Package {
    name: String,
    slots: Vec<Slot>,
}

impl Package {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            slots: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let package_one = Package::new();
        let package_two = Package {
            name: String::new(),
            slots: Vec::new(),
        };

        assert_eq!(package_one, package_two);
    }
}
