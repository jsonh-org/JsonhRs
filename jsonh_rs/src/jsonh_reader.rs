use std::{collections::HashSet, iter::Peekable, str::Chars};
use serde_json::Value;

pub struct JsonhReader<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> JsonhReader<'a> {
    /// Constructs a reader that reads JSONH from a peekable character iterator.
    pub fn from_peekable_chars(source: Peekable<Chars<'a>>) -> Self {
        return Self { source: source };
    }
    /// Constructs a reader that reads JSONH from a character iterator.
    pub fn from_chars(source: Chars<'a>) -> Self {
        return Self::from_peekable_chars(source.peekable());
    }
    /// Constructs a reader that reads JSONH from a string slice.
    pub fn from_str(source: &'a str) -> Self {
        return Self::from_chars(source.chars());
    }
    /// Constructs a reader that reads JSONH from a string.
    pub fn from_string(source: &'a String) -> Self {
        return Self::from_str(source.as_str());
    }

    /// Parses a single element from a peekable character iterator.
    pub fn parse_element_from_peekable_chars(source: Peekable<Chars<'a>>) -> Result<Value, String> {
        return Self::from_peekable_chars(source).parse_element();
    }
    /// Parses a single element from a character iterator.
    pub fn parse_element_from_chars(source: Chars<'a>) -> Result<Value, String> {
        return Self::from_chars(source).parse_element();
    }
    /// Parses a single element from a string slice.
    pub fn parse_element_from_str(source: &'a str) -> Result<Value, String> {
        return Self::from_str(source).parse_element();
    }
    /// Parses a single element from a string.
    pub fn parse_element_from_string(source: &'a String) -> Result<Value, String> {
        return Self::from_string(source).parse_element();
    }

    /// Parses a single element from a text reader.
    pub fn parse_element(&mut self) -> Result<Value, String> {
        return Ok(Value::String("example".to_string()));
    }

    fn peek(&mut self) -> Option<char> {
        return self.source.peek().copied();
    }
    fn read(&mut self) -> Option<char> {
        return self.source.next();
    }
    fn read_one(&mut self, option: char) -> bool {
        if self.peek() == Some(option) {
            self.read();
            return true;
        }
        return false;
    }
    fn read_any(&mut self, options: &HashSet<char>) -> Option<char> {
        // Peek char
        let next: char = self.peek()?;
        // Match option
        if !options.contains(&next) {
            return None;
        }
        // Option matched
        self.read();
        return Some(next);
    }
}