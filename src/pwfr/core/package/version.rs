//! Package version
//!
//! Version entity

use crate::core::package::use_flags::UseFlags;

#[derive(Debug, PartialEq)]
pub struct Version {
    version: String,
    use_flags: UseFlags,
}

impl Version {
    pub fn new() -> Self {
        Self {
            version: String::new(),
            use_flags: UseFlags::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let version_one = Version::new();
        let version_two = Version {
            version: String::new(),
            use_flags: UseFlags::new(),
        };

        assert_eq!(version_one, version_two);
    }
}
