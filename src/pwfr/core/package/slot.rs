//! Package slot
//!
//! Slot entity

use crate::core::package::version::Version;

#[derive(Debug, PartialEq)]
pub struct Slot {
    slot: String,
    versions: Vec<Version>,
}

impl Slot {
    pub fn new() -> Self {
        Self {
            slot: String::new(),
            versions: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let slot_one = Slot::new();
        let slot_two = Slot {
            slot: String::new(),
            versions: Vec::new(),
        };

        assert_eq!(slot_one, slot_two);
    }
}
