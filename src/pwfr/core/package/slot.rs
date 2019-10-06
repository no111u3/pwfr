//! Package slot
//!
//! Slot entity

use crate::core::package::version::Version;

#[derive(Debug, PartialEq)]
pub struct Slot {
    slot: i32,
    versions: Vec<Version>,
}

impl Slot {
    pub fn new() -> Self {
        Self {
            slot: 0,
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
            slot: 0,
            versions: Vec::new(),
        };

        assert_eq!(slot_one, slot_two);
    }
}
