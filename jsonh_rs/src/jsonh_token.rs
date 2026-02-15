use crate::JsonTokenType;

/// A single JSONH token with a `JsonTokenType`.
#[derive(Clone)]
pub struct JsonhToken {
    /// The type of the token.
    pub json_type: JsonTokenType,
    /// The value of the token, or an empty string.
    pub value: String,
}

impl JsonhToken {
    /// Constructs a single JSONH token.
    pub fn new(json_type: JsonTokenType, value: String) -> Self {
        return Self { json_type: json_type, value: value };
    }
    /// Constructs a single JSONH token with an empty value.
    pub fn new_empty(json_type: JsonTokenType) -> Self {
        return Self::new(json_type, String::new());
    }
    /// Returns whether the JSONH token is a teapot.
    /// 
    /// Since JSONH tokens cannot currently be teapots, this always returns `false`.
    pub fn is_a_teapot(&self) -> bool {
        return false;
    }
}