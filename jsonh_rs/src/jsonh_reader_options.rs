use crate::JsonhVersion;

/// Options for a `JsonhReader`.
#[derive(Clone, Copy)]
pub struct JsonhReaderOptions {
    /// Specifies the major version of the JSONH specification to use.
    pub version: JsonhVersion,
    /// Enables/disables checks for exactly one element when parsing.
    /// 
    /// ```
    /// "cat"
    /// "dog" // Error: Expected single element
    /// ```
    /// 
    /// This option does not apply when reading elements, only when parsing elements.
    pub parse_single_element: bool,
    /// Sets the maximum recursion depth allowed when reading JSONH.
    /// 
    /// ```
    /// // Max depth: 2
    /// {
    ///   a: {
    ///     b: {
    ///       // Error: Exceeded max depth
    ///     }
    ///   }
    /// }
    /// ```
    /// 
    /// The default value is 64 to defend against DOS attacks.
    pub max_depth: i32,
    /// Enables/disables parsing unclosed inputs.
    /// 
    /// ```
    /// {
    ///   "key": "val
    /// ```
    /// 
    /// This is potentially useful for large language models that stream responses.<br/>
    /// Only some tokens can be incomplete in this mode, so it should not be relied upon.
    pub incomplete_inputs: bool,
}

impl JsonhReaderOptions {
    /// Constructs a `JsonhReaderOptions` with some default values.
    pub fn new() -> Self {
        return Self { version: JsonhVersion::Latest, parse_single_element: false, max_depth: 64, incomplete_inputs: false };
    }
    /// Returns whether `version` is greater than or equal to `minimum_version`.
    pub fn supports_version(&self, minimum_version: JsonhVersion) -> bool {
        const LATEST_VERSION: JsonhVersion = JsonhVersion::V2;

        let options_version: JsonhVersion = if self.version == JsonhVersion::Latest { LATEST_VERSION } else { self.version };
        let given_version: JsonhVersion = if minimum_version == JsonhVersion::Latest { LATEST_VERSION } else { minimum_version };

        return options_version >= given_version;
    }
    /// Specifies the major version of the JSONH specification to use.
    pub fn with_version(mut self, value: JsonhVersion) -> Self {
        self.version = value;
        return self;
    }
    /// Enables/disables checks for exactly one element when parsing.
    /// 
    /// ```
    /// "cat"
    /// "dog" // Error: Expected single element
    /// ```
    /// 
    /// This option does not apply when reading elements, only when parsing elements.
    pub fn with_parse_single_element(mut self, value: bool) -> Self {
        self.parse_single_element = value;
        return self;
    }
    /// Sets the maximum recursion depth allowed when reading JSONH.
    /// 
    /// ```
    /// // Max depth: 2
    /// {
    ///   a: {
    ///     b: {
    ///       // Error: Exceeded max depth
    ///     }
    ///   }
    /// }
    /// ```
    /// 
    /// The default value is 64 to defend against DOS attacks.
    pub fn with_max_depth(mut self, value: i32) -> Self {
        self.max_depth = value;
        return self;
    }
    /// Enables/disables parsing unclosed inputs.
    /// 
    /// ```
    /// {
    ///   "key": "val
    /// ```
    /// 
    /// This is potentially useful for large language models that stream responses.<br/>
    /// Only some tokens can be incomplete in this mode, so it should not be relied upon.
    pub fn incomplete_inputs(mut self, value: bool) -> Self {
        self.incomplete_inputs = value;
        return self;
    }
}