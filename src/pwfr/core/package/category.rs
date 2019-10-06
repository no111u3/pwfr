//! Package category
//!
//! Category entity

use std::collections::btree_map::BTreeMap;

use crate::core::package::package::Package;

#[derive(Debug, PartialEq)]
pub struct Category {
    name: String,
    packages: BTreeMap<String, Package>,
}

impl Category {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            packages: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let category_one = Category::new();
        let category_two = Category {
            name: String::new(),
            packages: BTreeMap::new(),
        };

        assert_eq!(category_one, category_two);
    }
}
