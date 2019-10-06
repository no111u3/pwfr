//! Package use flags
//!
//! Use flags entity

#[derive(Debug, PartialEq)]
pub struct UseFlags {
    full: Vec<String>,
    active: Vec<String>,
}

impl UseFlags {
    pub fn new() -> Self {
        Self {
            full: Vec::new(),
            active: Vec::new(),
        }
    }

    pub fn get_full(&self) -> &Vec<String> {
        &self.full
    }

    pub fn get_active(&self) -> &Vec<String> {
        &self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let uses_one = UseFlags::new();
        let uses_two = UseFlags {
            full: Vec::new(),
            active: Vec::new(),
        };

        assert_eq!(uses_one, uses_two);

        let empty: Vec<String> = Vec::new();
        assert_eq!(uses_one.get_full(), &empty);
        assert_eq!(uses_one.get_active(), &empty);
    }
}
