use std::char;
use std::{iter::Peekable, str::Chars};
use serde_json::Number;
use serde_json::Value;
use yield_return::LocalIter;

use crate::JsonhToken;
use crate::JsonTokenType;
use crate::JsonhReaderOptions;
use crate::JsonhVersion;

pub struct JsonhReader<'a> {
    /// The peekable character iterator to read characters from.
    pub source: Peekable<Chars<'a>>,
    /// The options to use when reading JSONH.
    pub options: JsonhReaderOptions,
    /// The number of characters read from `source`.
    pub char_counter: u64,
    /// The current recursion depth of the reader.
    pub depth: i32,
}

impl<'a> JsonhReader<'a> {
    /// Characters that cannot be used unescaped in quoteless strings.
    fn reserved_chars(&self) -> &'static [char] { if self.options.supports_version(JsonhVersion::V2) { &Self::RESERVED_CHARS_V2 } else { &Self::RESERVED_CHARS_V1 } }
    /// Characters that cannot be used unescaped in quoteless strings in JSONH V1.
    const RESERVED_CHARS_V1: &'static [char] = &['\\', ',', ':', '[', ']', '{', '}', '/', '#', '"', '\''];
    /// Characters that cannot be used unescaped in quoteless strings in JSONH V2.
    const RESERVED_CHARS_V2: &'static [char] = &['\\', ',', ':', '[', ']', '{', '}', '/', '#', '"', '\'', '@'];
    /// Characters that are considered newlines.
    const NEWLINE_CHARS: &'static [char] = &['\n', '\r', '\u{2028}', '\u{2029}'];

    /// Constructs a reader that reads JSONH from a peekable character iterator.
    pub fn from_peekable_chars(source: Peekable<Chars<'a>>, options: JsonhReaderOptions) -> Self {
        return Self { source: source, options: options, char_counter: 0, depth: 0 };
    }
    /// Constructs a reader that reads JSONH from a character iterator.
    pub fn from_chars(source: Chars<'a>, options: JsonhReaderOptions) -> Self {
        return Self::from_peekable_chars(source.peekable(), options);
    }
    /// Constructs a reader that reads JSONH from a string slice.
    pub fn from_str(source: &'a str, options: JsonhReaderOptions) -> Self {
        return Self::from_chars(source.chars(), options);
    }
    /// Constructs a reader that reads JSONH from a string.
    pub fn from_string(source: &'a String, options: JsonhReaderOptions) -> Self {
        return Self::from_str(source.as_str(), options);
    }

    /// Parses a single element from a peekable character iterator.
    pub fn parse_element_from_peekable_chars(source: Peekable<Chars<'a>>, options: JsonhReaderOptions) -> Result<Value, &'static str> {
        return Self::from_peekable_chars(source, options).parse_element();
    }
    /// Parses a single element from a character iterator.
    pub fn parse_element_from_chars(source: Chars<'a>, options: JsonhReaderOptions) -> Result<Value, &'static str> {
        return Self::from_chars(source, options).parse_element();
    }
    /// Parses a single element from a string slice.
    pub fn parse_element_from_str(source: &'a str, options: JsonhReaderOptions) -> Result<Value, &'static str> {
        return Self::from_str(source, options).parse_element();
    }
    /// Parses a single element from a string.
    pub fn parse_element_from_string(source: &'a String, options: JsonhReaderOptions) -> Result<Value, &'static str> {
        return Self::from_string(source, options).parse_element();
    }

    /// Parses a single element from a text reader.
    pub fn parse_element(&mut self) -> Result<Value, &'static str> {
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
        let mut parse_next_element = |current_elements: &mut Vec<Value>, current_property_name: &mut Option<String>| -> Result<Value, &'static str> {
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
                            return Err("Failed to parse integer");
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
                    _ => return Err("Token type not implemented")
                }
            }

            // End of input
            return Err("Expected token, got end of input");
        };

        // Parse next element
        let next_element: Result<Value, &'static str> = parse_next_element(&mut current_elements, &mut current_property_name);

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
        let mut current_depth: i32 = 0;

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
    pub fn read_end_of_elements(&mut self) -> LocalIter<'_, Result<JsonhToken, &'static str>> {
        return LocalIter::new(|mut y| async move {
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
                y.ret(Err("Expected end of elements")).await;
            }
        });
    }
    /// Reads a single element from the reader.
    pub fn read_element(&mut self) -> LocalIter<'_, Result<JsonhToken, &'static str>> {
        return LocalIter::new(|mut y| async move {
            // Comments & whitespace
            for token_result in self.read_comments_and_whitespace() {
                if token_result.is_err() {
                    y.ret(token_result).await;
                    return;
                }
                y.ret(token_result).await;
            }

            // Peek char
            let Some(next) = self.peek() else {
                y.ret(Err("Expected token, got end of input")).await;
                return;
            };

            // Object
            if next == '{' {
                for token_result in self.read_object() {
                    if token_result.is_err() {
                        y.ret(token_result).await;
                        return;
                    }
                    y.ret(token_result).await;
                }
            }
            // Array
            else if next == '[' {
                for token_result in self.read_array() {
                    if token_result.is_err() {
                        y.ret(token_result).await;
                        return;
                    }
                    y.ret(token_result).await;
                }
            }
            // Primitive value (null, true, false, string, number)
            else {
                let token_result: Result<JsonhToken, &'static str> = self.read_primitive_element();
                if token_result.is_err() {
                    y.ret(token_result).await;
                    return;
                }

                // Detect braceless object from property name
                for token_result2 in self.read_braceless_object_or_end_of_primitive(token_result.unwrap()) {
                    if token_result2.is_err() {
                        y.ret(token_result2).await;
                        return;
                    }
                    y.ret(token_result2).await;
                }
            }
        });
    }

    fn read_object(&mut self) -> LocalIter<'_, Result<JsonhToken, &'static str>> {
        return LocalIter::new(|mut y| async move {
            // Opening brace
            if !self.read_one('{') {
                // Braceless object
                for token_result in self.read_braceless_object(None) {
                    if token_result.is_err() {
                        y.ret(token_result).await;
                        return;
                    }
                    y.ret(token_result).await;
                }
                return;
            }
            // Start of object
            y.ret(Ok(JsonhToken::new_empty(JsonTokenType::StartObject))).await;
            self.depth += 1;

            // Check exceeded max depth
            if self.depth > self.options.max_depth {
                y.ret(Err("Exceeded max depth")).await;
                return;
            }

            loop {
                // Comments & whitespace
                for token_result in self.read_comments_and_whitespace() {
                    if token_result.is_err() {
                        y.ret(token_result).await;
                        return;
                    }
                    y.ret(token_result).await;
                }

                let Some(next) = self.peek() else {
                    // End of incomplete object
                    if self.options.incomplete_inputs {
                        self.depth -= 1;
                        y.ret(Ok(JsonhToken::new_empty(JsonTokenType::EndObject))).await;
                        return;
                    }
                    // Missing closing brace
                    y.ret(Err("Expected `}` to end object, got end of input")).await;
                    return;
                };

                // Closing brace
                if next == '}' {
                    // End of object
                    self.read();
                    self.depth -= 1;
                    y.ret(Ok(JsonhToken::new_empty(JsonTokenType::EndObject))).await;
                    return;
                }
                // Property
                else {
                    for token_result in self.read_property(None) {
                        if token_result.is_err() {
                            y.ret(token_result).await;
                            return;
                        }
                        y.ret(token_result).await;
                    }
                }
            }
        });
    }
    fn read_braceless_object(&mut self, property_name_tokens: Option<Vec<JsonhToken>>) -> LocalIter<'_, Result<JsonhToken, &'static str>> {
        return LocalIter::new(|mut y| async move {
            // Start of object
            y.ret(Ok(JsonhToken::new_empty(JsonTokenType::StartObject))).await;
            self.depth += 1;

            // Check exceeded max depth
            if self.depth > self.options.max_depth {
                y.ret(Err("Exceeded max depth")).await;
                return;
            }

            // Initial tokens
            if property_name_tokens.is_some() {
                for initial_token_result in self.read_property(property_name_tokens) {
                    if initial_token_result.is_err() {
                        y.ret(initial_token_result).await;
                        return;
                    }
                    y.ret(initial_token_result).await;
                }
            }

            loop {
                // Comments & whitespace
                for token_result in self.read_comments_and_whitespace() {
                    if token_result.is_err() {
                        y.ret(token_result).await;
                        return;
                    }
                    y.ret(token_result).await;
                }

                if self.peek().is_none() {
                    // End of braceless object
                    self.depth -= 1;
                    y.ret(Ok(JsonhToken::new_empty(JsonTokenType::EndObject))).await;
                    return;
                };

                // Property
                for token_result in self.read_property(None) {
                    if token_result.is_err() {
                        y.ret(token_result).await;
                        return;
                    }
                    y.ret(token_result).await;
                }
            }
        });
    }
    fn read_braceless_object_or_end_of_primitive(&mut self, primitive_token: JsonhToken) -> LocalIter<'_, Result<JsonhToken, &'static str>> {
        return LocalIter::new(|mut y| async move {
            // Comments & whitespace
            let mut property_name_tokens: Vec<JsonhToken> = Vec::new();
            for comment_or_whitespace_token_result in self.read_comments_and_whitespace() {
                if comment_or_whitespace_token_result.is_err() {
                    y.ret(comment_or_whitespace_token_result).await;
                    return;
                }
                property_name_tokens.push(comment_or_whitespace_token_result.unwrap());
            }

            // Primitive
            if !self.read_one(':') {
                // Primitive
                y.ret(Ok(primitive_token)).await;
                // Comments & whitespace
                for comment_or_whitespace_token in property_name_tokens {
                    y.ret(Ok(comment_or_whitespace_token)).await;
                }
                // End of primitive
                return;
            }

            // Property name
            property_name_tokens.push(JsonhToken::new(JsonTokenType::PropertyName, primitive_token.value));

            // Braceless object
            for object_token in self.read_braceless_object(Some(property_name_tokens)) {
                if object_token.is_err() {
                    y.ret(object_token).await;
                    return;
                }
                y.ret(object_token).await;
            }
        });
    }
    fn read_property(&mut self, property_name_tokens: Option<Vec<JsonhToken>>) -> LocalIter<'_, Result<JsonhToken, &'static str>> {
        return LocalIter::new(|mut y| async move {
            // Property name
            if !property_name_tokens.is_none() {
                for token in property_name_tokens.unwrap() {
                    y.ret(Ok(token)).await;
                }
            }
            else {
                for token in self.read_property_name() {
                    if token.is_err() {
                        y.ret(token).await;
                        return;
                    }
                    y.ret(token).await;
                }
            }

            // Comments & whitespace
            for token_result in self.read_comments_and_whitespace() {
                if token_result.is_err() {
                    y.ret(token_result).await;
                    return;
                }
                y.ret(token_result).await;
            }

            // Property value
            for token_result in self.read_element() {
                if token_result.is_err() {
                    y.ret(token_result).await;
                    return;
                }
                y.ret(token_result).await;
            }

            // Comments & whitespace
            for token_result in self.read_comments_and_whitespace() {
                if token_result.is_err() {
                    y.ret(token_result).await;
                    return;
                }
                y.ret(token_result).await;
            }

            // Optional comma
            self.read_one(',');
        });
    }
    fn read_property_name(&mut self) -> LocalIter<'_, Result<JsonhToken, &'static str>> {
        return LocalIter::new(|mut y| async move {
            // String
            let string_result: Result<JsonhToken, &'static str> = self.read_string();
            if string_result.is_err() {
                y.ret(string_result).await;
                return;
            }

            // Comments & whitespace
            for token_result in self.read_comments_and_whitespace() {
                if token_result.is_err() {
                    y.ret(token_result).await;
                    return;
                }
                y.ret(token_result).await;
            }

            // Colon
            if !self.read_one(':') {
                y.ret(Err("Expected `:` after property name in object")).await;
                return;
            }

            // End of property name
            y.ret(Ok(JsonhToken::new(JsonTokenType::PropertyName, string_result.unwrap().value))).await;
        });
    }
    fn read_array(&mut self) -> LocalIter<'_, Result<JsonhToken, &'static str>> {
        return LocalIter::new(|mut y| async move {
            // Opening bracket
            if !self.read_one('[') {
                y.ret(Err("Expected `[` to start array")).await;
                return;
            }
            // Start of array
            y.ret(Ok(JsonhToken::new_empty(JsonTokenType::StartArray))).await;
            self.depth += 1;

            // Check exceeded max depth
            if self.depth > self.options.max_depth {
                y.ret(Err("Exceeded max depth")).await;
                return;
            }

            loop {
                // Comments & whitespace
                for token_result in self.read_comments_and_whitespace() {
                    if token_result.is_err() {
                        y.ret(token_result).await;
                        return;
                    }
                    y.ret(token_result).await;
                }

                let Some(next) = self.peek() else {
                    // End of incomplete array
                    if self.options.incomplete_inputs {
                        self.depth -= 1;
                        y.ret(Ok(JsonhToken::new_empty(JsonTokenType::EndArray))).await;
                        return;
                    }
                    // Missing closing bracket
                    y.ret(Err("Expected `]` to end array, got end of input")).await;
                    return;
                };

                // Closing bracket
                if next == ']' {
                    // End of array
                    self.read();
                    self.depth -= 1;
                    y.ret(Ok(JsonhToken::new_empty(JsonTokenType::EndArray))).await;
                    return;
                }
                // Item
                else {
                    for token_result in self.read_item() {
                        if token_result.is_err() {
                            y.ret(token_result).await;
                            return;
                        }
                        y.ret(token_result).await;
                    }
                }
            }
        });
    }
    fn read_item(&mut self) -> LocalIter<'_, Result<JsonhToken, &'static str>> {
        return LocalIter::new(|mut y| async move {
            todo!();
        });
    }
    fn read_string(&mut self) -> Result<JsonhToken, &'static str> {
        todo!();
    }
    fn read_quoteless_string(&mut self, initial_chars: &str, is_verbatim: bool) -> Result<JsonhToken, &'static str> {
        todo!();
    }
    fn detect_quoteless_string(&mut self, whitespace_chars: &mut String) -> bool {
        todo!();
    }
    fn read_number(&mut self, number_builder: &mut String) -> Result<JsonhToken, &'static str> {
        todo!();
    }
    fn read_number_no_exponent(&mut self, number_builder: &mut String, base_digits: &str, has_base_specifier: bool, has_leading_zero: bool) -> Result<(), &'static str> {
        // Leading underscore
        if !has_base_specifier && !has_leading_zero && self.peek() == Some('_') {
            return Err("Leading `_` in number");
        }

        let mut is_fraction: bool = false;
        let mut is_empty: bool = true;

        // Leading zero (not base specifier)
        if has_leading_zero {
            is_empty = false;
        }

        loop {
            // Peek char
            let Some(next) = self.peek() else {
                break;
            };

            // Digit
            if base_digits.contains(next.to_ascii_lowercase()) {
                self.read();
                number_builder.push(next);
                is_empty = false;
            }
            // Dot
            else if next == '.' {
                // Disallow dot following underscore
                if number_builder.ends_with('_') {
                    return Err("`.` must not follow `_` in number");
                }

                self.read();
                number_builder.push(next);
                is_empty = false;

                // Duplicate dot
                if is_fraction {
                    return Err("Duplicate `.` in number");
                }
                is_fraction = true;
            }
            // Underscore
            else if next == '_' {
                // Disallow underscore following dot
                if number_builder.ends_with('.') {
                    return Err("`_` must not follow `.` in number");
                }

                self.read();
                number_builder.push(next);
                is_empty = false;
            }
            // Other
            else {
                break;
            }
        }

        // Ensure not empty
        if is_empty {
            return Err("Empty number");
        }

        // Ensure at least one digit
        if !number_builder.chars().any(|c| !matches!(c, '.' | '-' | '+' | '_')) {
            return Err("Number must have at least one digit");
        }

        // Trailing underscore
        if number_builder.ends_with('_') {
            return Err("Trailing `_` in number");
        }

        // End of number
        return Ok(());
    }
    fn read_number_or_quoteless_string(&mut self) -> Result<JsonhToken, &'static str> {
        // Read number
        let mut number_builder: String = String::new();
        let number: Result<JsonhToken, &'static str> = self.read_number(&mut number_builder);
        if number.is_ok() {
            // Try read quoteless string starting with number
            let mut whitespace_chars: String = String::new();
            if self.detect_quoteless_string(&mut whitespace_chars) {
                return self.read_quoteless_string((number.unwrap().value + whitespace_chars.as_str()).as_str(), false);
            }
            // Otherwise, accept number
            else {
                return number;
            }
        }
        // Read quoteless string starting with malformed number
        else {
            return self.read_quoteless_string(number_builder.as_str(), false);
        }
    }
    fn read_primitive_element(&mut self) -> Result<JsonhToken, &'static str> {
        // Peek char
        let Some(next) = self.peek() else {
            return Err("Expected primitive element, got end of input");
        };

        // Number
        if matches!(next, '0'..='9' | '-' | '+' | '.') {
            return self.read_number_or_quoteless_string();
        }
        // String
        else if matches!(next, '"' | '\'') || (self.options.supports_version(JsonhVersion::V2) && next == '@') {
            return self.read_string();
        }
        // Quoteless string (or named literal)
        else {
            return self.read_quoteless_string("", false);
        }
    }
    fn read_comments_and_whitespace(&mut self) -> LocalIter<'_, Result<JsonhToken, &'static str>> {
        return LocalIter::new(|mut y| async move {
            loop {
                // Whitespace
                self.read_whitespace();

                // Comment
                if matches!(self.peek(), Some('#') | Some('/')) {
                    let comment_result: Result<JsonhToken, &'static str> = self.read_comment();
                    if comment_result.is_err() {
                        y.ret(comment_result).await;
                        return;
                    }
                    y.ret(comment_result).await;
                }
                // End of comments
                else {
                    return;
                }
            }
        });
    }
    fn read_comment(&mut self) -> Result<JsonhToken, &'static str> {
        let mut block_comment: bool = false;
        let mut start_nest_counter: i32 = 0;

        // Hash-style comment
        if self.read_one('#') {
        }
        else if self.read_one('/') {
            // Line-style comment
            if self.read_one('/') {
            }
            // Block-style comment
            else if self.read_one('*') {
                block_comment = true;
            }
            // Nestable block-style comment
            else if self.options.supports_version(JsonhVersion::V2) && self.peek() == Some('=') {
                block_comment = true;
                while self.read_one('=') {
                    start_nest_counter += 1;
                }
                if !self.read_one('*') {
                    return Err("Expected `*` after start of nesting block comment");
                }
            }
            else {
                return Err("Unexpected `/`");
            }
        }
        else {
            return Err("Unexpected character");
        }

        // Read comment
        let mut comment_builder: String = String::new();

        loop {
            // Read char
            let next: Option<char> = self.read();

            if block_comment {
                // Error
                if next.is_none() {
                    return Err("Expected end of block comment, got end of input");
                }

                // End of block comment
                if next == Some('*') {
                    // End of nestable block comment
                    if self.options.supports_version(JsonhVersion::V2) {
                        // Count nests
                        let mut end_nest_counter: i32 = 0;
                        while end_nest_counter < start_nest_counter && self.read_one('=') {
                            end_nest_counter += 1;
                        }
                        // Partial end nestable block comment was actually part of comment
                        if end_nest_counter < start_nest_counter || self.peek() != Some('/') {
                            comment_builder.push('*');
                            while end_nest_counter > 0 {
                                comment_builder.push('=');
                                end_nest_counter -= 1;
                            }
                            continue;
                        }
                    }

                    // End of block comment
                    if self.read_one('/') {
                        return Ok(JsonhToken::new(JsonTokenType::Comment, comment_builder));
                    }
                }
            }
            else {
                // End of line comment
                if next.is_none() || Self::NEWLINE_CHARS.contains(&next.unwrap()) {
                    return Ok(JsonhToken::new(JsonTokenType::Comment, comment_builder));
                }
            }

            // Comment char
            comment_builder.push(next.unwrap());
        }
    }
    fn read_whitespace(&mut self) -> () {
        loop {
            // Peek char
            let Some(next) = self.peek() else {
                return;
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
    fn read_hex_sequence<const LENGTH: usize>(&mut self) -> Result<u32, &'static str> {
        const { assert!(LENGTH <= 8); };

        let mut value: u32 = 0;

        for _index in 0..LENGTH {
            let next: Option<char> = self.read();

            // Hex digit
            if matches!(next, Some('0'..='9' | 'A'..='F' | 'a'..='f')) {
                // Get hex digit
                let digit: char = next.unwrap();
                // Convert hex digit to integer
                let integer: u32 = match digit {
                    'A'..='F' => (digit as u32) - ('A' as u32) + 10,
                    'a'..='f' => (digit as u32) - ('a' as u32) + 10,
                    _ => (digit as u32) - ('0' as u32)
                };
                // Aggregate digit into value
                value = (value * 16) + integer;
            }
            // Unexpected char
            else {
                return Err("Incorrect number of hexadecimal digits in unicode escape sequence");
            }
        }

        // Return aggregated value
        return Ok(value);
    }
    fn read_escape_sequence(&mut self, high_surrogate: Option<u32>) -> Result<String, &'static str> {
        let Some(escape_char) = self.read() else {
            return Err("Expected escape sequence, got end of input");
        };

        // Ensure high surrogates are completed
        if high_surrogate.is_some() && !matches!(escape_char, 'u' | 'x' | 'U') {
            return Err("Expected low surrogate after high surrogate");
        }

        // Reverse solidus
        if escape_char == '\\' {
            return Ok("\\".to_string());
        }
        // Backspace
        else if escape_char == 'b' {
            return Ok("\x08".to_string()); // "\b"
        }
        // Form feed
        else if escape_char == 'f' {
            return Ok("\x0c".to_string()); // "\f"
        }
        // Newline
        else if escape_char == 'n' {
            return Ok("\n".to_string());
        }
        // Carriage return
        else if escape_char == 'r' {
            return Ok("\r".to_string());
        }
        // Tab
        else if escape_char == 't' {
            return Ok("\t".to_string());
        }
        // Vertical tab
        else if escape_char == 'v' {
            return Ok("\x0b".to_string()); // "\v"
        }
        // Null
        else if escape_char == '0' {
            return Ok("\0".to_string());
        }
        // Alert
        else if escape_char == 'a' {
            return Ok("\x07".to_string()); // "\a"
        }
        // Escape
        else if escape_char == 'e' {
            return Ok("\x1b".to_string()); // "\e"
        }
        // Unicode hex sequence
        else if escape_char == 'u' {
            return self.read_hex_escape_sequence::<4>(high_surrogate);
        }
        // Short unicode hex sequence
        else if escape_char == 'x' {
            return self.read_hex_escape_sequence::<2>(high_surrogate);
        }
        // Long unicode hex sequence
        else if escape_char == 'U' {
            return self.read_hex_escape_sequence::<8>(high_surrogate);
        }
        // Escaped newline
        else if Self::NEWLINE_CHARS.contains(&escape_char) {
            // Join CR LF
            if escape_char == '\r' {
                self.read_one('\n');
            }
            return Ok(String::new());
        }
        // Other
        else {
            return Ok(escape_char.to_string());
        }
    }
    fn read_hex_escape_sequence<const LENGTH: usize>(&mut self, high_surrogate: Option<u32>) -> Result<String, &'static str> {
        let code_point: u32 = match self.read_hex_sequence::<LENGTH>() {
            Ok(code_point) => code_point,
            Err(err) => return Err(err),
        };

        // Low surrogate
        if high_surrogate.is_some() {
            let combined: u32 = match Self::utf16_surrogates_to_code_point(high_surrogate.unwrap(), code_point) {
                Ok(combined) => combined,
                Err(err) => return Err(err),
            };
            return match char::from_u32(combined) {
                Some(combined_char) => Ok(combined_char.to_string()),
                None => Err("Invalid hex escape sequence"),
            };
        }
        else {
            // High surrogate followed by low surrogate
            if Self::is_utf16_high_surrogate(code_point) && self.read_one('\\') {
                return self.read_escape_sequence(Some(code_point));
            }
            // Standalone character
            else {
                return match char::from_u32(code_point) {
                    Some(code_point_char) => Ok(code_point_char.to_string()),
                    None => Err("Invalid hex escape sequence"),
                };
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
    fn read_any(&mut self, options: &[char]) -> Option<char> {
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
    const fn utf16_surrogates_to_code_point(high_surrogate: u32, low_surrogate: u32) -> Result<u32, &'static str> {
        if !Self::is_utf16_high_surrogate(high_surrogate) {
            return Err("High surrogate out of range");
        }
        if !Self::is_utf16_low_surrogate(low_surrogate) {
            return Err("Low surrogate out of range");
        }
        return Ok(0x10000 + (((high_surrogate - 0xD800) << 10) | (low_surrogate - 0xDC00)));
    }
    const fn is_utf16_high_surrogate(code_point: u32) -> bool {
        return code_point >= 0xD800 && code_point <= 0xDBFF;
    }
    const fn is_utf16_low_surrogate(code_point: u32) -> bool {
        return code_point >= 0xDC00 && code_point <= 0xDFFF;
    }
}