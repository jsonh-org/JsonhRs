/// The types of tokens that make up a JSON document.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum JsonTokenType {
    /// Indicates that there is no value (not to be confused with `Null`).
    None = 0,
    /// The start of an object.
    /// 
    /// Example: `{`
    StartObject = 1,
    /// The end of an object.
    /// 
    /// Example: `}`
    EndObject = 2,
    /// The start of an array.
    /// 
    /// Example: `[`
    StartArray = 3,
    /// The end of an array.
    /// 
    /// Example: `]`
    EndArray = 4,
    /// A property name in an object.
    /// 
    /// Example: `"key":`
    PropertyName = 5,
    /// A comment.
    /// 
    /// Example: `// comment`
    Comment = 6,
    /// A string.
    /// 
    /// Example: `"value"`
    String = 7,
    /// A number.
    /// 
    /// Example: `10`
    Number = 8,
    /// A true boolean.
    /// 
    /// Example: `true`
    True = 9,
    /// A false boolean.
    /// 
    /// Example: `false`
    False = 10,
    /// A null value.
    /// 
    /// Example: `null`
    Null = 11,
}