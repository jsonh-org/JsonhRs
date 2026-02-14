use std::{collections::HashSet, iter::Peekable, str::Chars};
use serde_json::Number;
use serde_json::Value;
use yield_return::Iter;

use crate::JsonhToken;
use crate::JsonTokenType;

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
        let mut current_elements: Vec<Value> = Vec::new();
        let mut current_property_name: Option<String> = None;

        let submit_element = |current_elements: &mut Vec<Value>, current_property_name: &mut Option<String>, element: Value| -> bool {
            // Root value
            if current_elements.is_empty() {
                return true;
            }
            // Array item
            if current_property_name.is_none() {
                current_elements.last_mut().unwrap().as_array_mut().unwrap().push(element);
                return false;
            }
            // Object property
            else {
                current_elements.last_mut().unwrap()[current_property_name.as_ref().unwrap()] = element;
                *current_property_name = None;
                return false;
            }
        };
        let start_element = |current_elements: &mut Vec<Value>, current_property_name: &mut Option<String>, element: Value| -> () {
            submit_element(current_elements, current_property_name, element.clone());
            current_elements.push(element);
        };
        let mut parse_next_element = |current_elements: &mut Vec<Value>, current_property_name: &mut Option<String>| -> Result<Value, String> {
            for token_result in self.read_element() {
                // Check error
                let token: JsonhToken = token_result?;

                match token.json_type {
                    // Null
                    JsonTokenType::Null => {
                        let element: Value = Value::Null;
                        if submit_element(current_elements, current_property_name, element.clone()) {
                            return Ok(element);
                        }
                    },
                    // True
                    JsonTokenType::True => {
                        let element: Value = Value::Bool(true);
                        if submit_element(current_elements, current_property_name, element.clone()) {
                            return Ok(element);
                        }
                    },
                    // False
                    JsonTokenType::False => {
                        let element: Value = Value::Bool(false);
                        if submit_element(current_elements, current_property_name, element.clone()) {
                            return Ok(element);
                        }
                    },
                    // String
                    JsonTokenType::String => {
                        let element: Value = Value::String(token.value);
                        if submit_element(current_elements, current_property_name, element.clone()) {
                            return Ok(element);
                        }
                    },
                    // Number
                    JsonTokenType::Number => {
                        let result = token.value.parse::<u32>(); // TODO
                        if result.is_err() {
                            return Err("Failed to parse integer".to_string());
                        }
                        let element: Value = Value::Number(Number::from(result.unwrap()));
                        if submit_element(current_elements, current_property_name, element.clone()) {
                            return Ok(element);
                        }
                    },
                    // Start Object
                    JsonTokenType::StartObject => {
                        let element: Value = Value::Object(serde_json::Map::new());
                        start_element(current_elements, current_property_name, element);
                    },
                    // Start Array
                    JsonTokenType::StartArray => {
                        let element: Value = Value::Array(Vec::new());
                        start_element(current_elements, current_property_name, element);
                    },
                    // End Object/Array
                    JsonTokenType::EndObject | JsonTokenType::EndArray => {
                        // Nested element
                        if current_elements.len() > 1 {
                            current_elements.pop();
                        }
                        // Root element
                        else {
                            return Ok(current_elements.last().unwrap().clone());
                        }
                    },
                    // Property Name
                    JsonTokenType::PropertyName => {
                        *current_property_name = Some(token.value);
                    },
                    // Comment
                    JsonTokenType::Comment => (),
                    // Not implemented
                    _ => return Err("Token type not implemented".to_string())
                }
            }

            // End of input
            return Err("Expected token, got end of input".to_string());
        };

        // Parse next element
        let next_element: Result<Value, String> = parse_next_element(&mut current_elements, &mut current_property_name);

        // Ensure exactly one element
        if next_element.is_ok() {
            // TODO
        }

        return next_element;
    }
    /// Tries to find the given property name in the reader.
    /// 
    /// For example, to find `c`:
    /// ```
    /// // Original position
    /// {
    ///   "a": "1",
    ///   "b": {
    ///     "c": "2"
    ///   },
    ///   "c":/* Final position */ "3"
    /// }
    /// ```
    pub fn find_property_value(&mut self, property_name: &str) -> bool {
        let mut current_depth: u32 = 0;

        for token_result in self.read_element() {
            // Check error
            let token: JsonhToken = match token_result {
                Ok(token) => token,
                Err(_) => return false,
            };

            match token.json_type {
                // Start structure
                JsonTokenType::StartObject | JsonTokenType::StartArray => {
                    current_depth += 1;
                },
                // End structure
                JsonTokenType::EndObject | JsonTokenType::EndArray => {
                    current_depth -= 1;
                },
                // Property name
                JsonTokenType::PropertyName => {
                    if current_depth == 1 && token.value == property_name {
                        // Path found
                        return true;
                    }
                },
                // Other
                _ => ()
            }
        }

        // Path not found
        return false;
    }
    /// Reads whitespace and returns whether the reader contains another token.
    pub fn has_token(&mut self) -> bool {
        // Whitespace
        self.read_whitespace();

        // Peek char
        return self.peek().is_some();
    }
    /// Reads comments and whitespace and errors if the reader contains another element.
    pub fn read_end_of_elements(&mut self) -> Iter<'_, Result<JsonhToken, String>> {
        return Iter::new(|mut y| async move {
            // Comments & whitespace
            for token_result in self.read_comments_and_whitespace() {
                if token_result.is_err() {
                    y.ret(token_result).await;
                    return;
                }
                y.ret(token_result).await;
            }

            // Peek char
            if self.peek().is_none() {
                y.ret(Err("Expected end of elements".to_string())).await;
            }
        });
    }
    /// Reads a single element from the reader.
    pub fn read_element(&mut self) -> Iter<'_, Result<JsonhToken, String>> {
        return Iter::new(|mut y| async move {
            y.ret(Ok(JsonhToken::new(JsonTokenType::Comment, "a".to_string()))).await;
        });
    }

    fn read_comments_and_whitespace(&mut self) -> Iter<'_, Result<JsonhToken, String>> {
        return Iter::new(|mut y| async move {
            loop {
                // Whitespace
                self.read_whitespace();

                // Comment
                if matches!(self.peek(), Some('#') | Some('/')) {

                }

                y.ret(Err("todo".to_string())).await; // TODO
            }
        });
    }
    fn read_whitespace(&mut self) -> () {
        loop {
            // Peek char
            let next: char = match self.peek() {
                Some(next) => next,
                None => return,
            };

            // Whitespace
            if char::is_whitespace(next) {
                self.read();
            }
            // End of whitespace
            else {
                return;
            }
        }
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