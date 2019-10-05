//! Ebuild and Eclass variables
//!
//! Variable internals
//! Inspired by https://crates.io/crates/nsh (github: https://github.com/seiyanuta/nsh)
use crate::core::ebuild::efile;

/// A variable value.
#[derive(Debug)]
pub enum Value {
    String(String),
    Array(Vec<String>),
    Function(Box<efile::Command>),
}

/// A efile variable.
#[derive(Debug)]
pub struct Variable {
    // The inner value. `None` represents *null*.
    value: Option<Value>,
}

impl Variable {
    /// Creates a `Variable`. This does not add to the
    /// any scope.
    pub fn new(value: Option<Value>) -> Self {
        Self { value }
    }

    /// Returns a reference to the inner value.
    #[inline]
    pub fn value(&self) -> &Option<Value> {
        &self.value
    }

    /// References its value as `$var`.
    pub fn as_str(&self) -> &str {
        match &self.value {
            Some(Value::String(value)) => value,
            Some(Value::Function(_)) => "(function)",
            Some(Value::Array(elements)) => match elements.get(0) {
                Some(element) => element.as_str(),
                _ => "",
            },
            None => "",
        }
    }

    /// References its value as `$var[expr]`.
    pub fn value_at(&self, index: usize) -> &str {
        match &self.value {
            Some(Value::Array(elements)) => match elements.get(index) {
                Some(element) => element.as_str(),
                _ => "",
            },
            _ => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let var = Variable::new(Option::None);
        assert_eq!(var.as_str(), "");
    }

    #[test]
    fn string_value() {
        const HELLO: &str = "Hello, World!";
        let var = Variable::new(Option::Some(Value::String(String::from(HELLO))));
        assert_eq!(var.as_str(), HELLO);
    }

    #[test]
    fn array_value() {
        let hello = vec!["Hello,".to_string(), "World".to_string()];
        let var = Variable::new(Option::Some(Value::Array(hello)));
        assert_eq!(var.as_str(), "Hello,");
    }

    #[test]
    fn function_value() {
        let function = efile::Command::FunctionDef {
            name: "func1".to_string(),
            body: Box::new(efile::Command::Return { status: Some(0) }),
        };
        let var = Variable::new(Option::Some(Value::Function(Box::new(function))));
        assert_eq!(var.as_str(), "(function)");
    }
}
