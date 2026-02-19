use jsonh_rs::*;

#[test]
pub fn quoteless_string_starting_with_keyword_test() {
    let jsonh: &str = r#"
[nulla, null b, null, @null]
"#;
    let element: Vec<Option<String>> = serde_json::from_value(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap();

    assert_eq!(element, [Some("nulla".to_string()), Some("null b".to_string()), None, Some("null".to_string())]);
}

#[test]
pub fn braceless_object_with_invalid_value_test() {
    let jsonh: &str = r#"
a: {
"#;

    assert!(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).is_err());
}

#[test]
pub fn nested_braceless_object_test() {
    let jsonh: &str = r#"
[
    a: b
    c: d
]
"#;

    assert!(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).is_err());
}

#[test]
pub fn quoteless_strings_leading_trailing_whitespace_test() {
    let jsonh: &str = r#"
[
    a b  , 
]
"#;

    assert_eq!(
        serde_json::from_value::<Vec<String>>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        ["a b".to_string()]
    );
}

#[test]
pub fn space_in_quoteless_property_name_test() {
    let jsonh: &str = r#"
{
    a b: c d
}
"#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element.as_object().unwrap().len(), 1);
    assert_eq!(element.as_object().unwrap()["a b"], "c d");
}

#[test]
pub fn quoteless_strings_escape_test() {
    let jsonh: &str = r#"
a: \"5
b: \\z
c: 5 \\
"#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element.as_object().unwrap().len(), 3);
    assert_eq!(element.as_object().unwrap()["a"], "\"5");
    assert_eq!(element.as_object().unwrap()["b"], "\\z");
    assert_eq!(element.as_object().unwrap()["c"], "5 \\");
}

#[test]
pub fn multi_quoted_strings_no_last_newline_whitespace_test() {
    let jsonh: &str = r#"
"""
  hello world  """
"#;

    assert_eq!(
        serde_json::from_value::<String>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        "\n  hello world  "
    );
}

#[test]
pub fn multi_quoted_strings_no_first_whitespace_newline_test() {
    let jsonh: &str = r#"
"""  hello world
  """
"#;

    assert_eq!(
        serde_json::from_value::<String>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        "  hello world\n  "
    );
}

#[test]
pub fn quoteless_strings_escaped_leading_trailing_whitespace_test() {
    let jsonh: &str = r#"
\nZ\ \r
"#;

    assert_eq!(
        serde_json::from_value::<String>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        "Z"
    );
}

#[test]
pub fn hex_number_with_e_test() {
    let jsonh: &str = r#"
0x5e3
"#;

    assert_eq!(
        serde_json::from_value::<f64>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        0x5e3 as f64
    );

    let jsonh2: &str = r#"
0x5e+3
"#;

    assert_eq!(
        serde_json::from_value::<f64>(JsonhReader::parse_element_from_str(jsonh2, JsonhReaderOptions::new()).unwrap()).unwrap(),
        5000 as f64
    );
}

#[test]
pub fn number_with_repeated_underscores_test() {
    let jsonh: &str = r#"
100__000
"#;

    assert_eq!(
        serde_json::from_value::<f64>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        100__000 as f64
    );
}

#[test]
pub fn number_with_underscores_after_base_specifier_test() {
    let jsonh: &str = r#"
0b_100
"#;

    assert_eq!(
        serde_json::from_value::<f64>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        0b_100 as f64
    );
}

#[test]
pub fn negative_number_with_base_specifier_test() {
    let jsonh: &str = r#"
-0x5
"#;

    assert_eq!(
        serde_json::from_value::<f64>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        -0x5 as f64
    );
}

#[test]
pub fn number_dot_test() {
    let jsonh: &str = r#"
.
"#;

    assert!(matches!(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap(), Value::String(_)));
    assert_eq!(
        serde_json::from_value::<String>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        "."
    );

    let jsonh: &str = r#"
-.
"#;

    assert!(matches!(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap(), Value::String(_)));
    assert_eq!(
        serde_json::from_value::<String>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        "-."
    );
}

#[test]
pub fn duplicate_property_name_test() {
    let jsonh: &str = r#"
{
  a: 1,
  c: 2,
  a: 3,
}
"#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element.as_object().unwrap().len(), 2);
    assert_eq!(element.as_object().unwrap()["a"], 3 as f64);
    assert_eq!(element.as_object().unwrap()["c"], 2 as f64);
}

#[test]
pub fn empty_number_test() {
    let jsonh: &str = r#"
0e
"#;

    assert!(matches!(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap(), Value::String(_)));
    assert_eq!(
        serde_json::from_value::<String>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        "0e"
    );
}

#[test]
pub fn leading_zero_with_exponent_test() {
    let jsonh: &str = r#"
[0e4, 0xe, 0xEe+2]
"#;

    assert_eq!(
        serde_json::from_value::<Vec<f64>>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        [0e4 as f64, 0xe as f64, 1400 as f64]
    );

    let jsonh2: &str = r#"
[e+2, 0xe+2, 0oe+2, 0be+2]
"#;

    assert_eq!(
        serde_json::from_value::<Vec<String>>(JsonhReader::parse_element_from_str(jsonh2, JsonhReaderOptions::new()).unwrap()).unwrap(),
        ["e+2", "0xe+2", "0oe+2", "0be+2"]
    );

    let jsonh3: &str = r#"
[0x0e+, 0b0e+_1]
"#;

    assert_eq!(
        serde_json::from_value::<Vec<String>>(JsonhReader::parse_element_from_str(jsonh3, JsonhReaderOptions::new()).unwrap()).unwrap(),
        ["0x0e+", "0b0e+_1"]
    );
}

#[test]
pub fn error_in_braceless_property_name_test() {
    let jsonh: &str = r#"
a /
"#;

    assert!(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).is_err());
}

#[test]
pub fn first_property_name_in_braceless_object_test() {
    let jsonh: &str = r#"
a: b
"#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element.as_object().unwrap().len(), 1);
    assert_eq!(element.as_object().unwrap()["a"], "b");

    let jsonh2: &str = r#"
0: b
"#;
    let element2: Value = JsonhReader::parse_element_from_str(jsonh2, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element2.as_object().unwrap().len(), 1);
    assert_eq!(element2.as_object().unwrap()["0"], "b");

    let jsonh3: &str = r#"
true: b
"#;
    let element3: Value = JsonhReader::parse_element_from_str(jsonh3, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element3.as_object().unwrap().len(), 1);
    assert_eq!(element3.as_object().unwrap()["true"], "b");
}

#[test]
pub fn fraction_leading_zeroes_test() {
    let jsonh: &str = r#"
0.04
"#;

    assert_eq!(
        serde_json::from_value::<f64>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        0.04 as f64
    );
}

#[test]
pub fn underscore_after_leading_zero_test() {
    let jsonh: &str = r#"
0_0
"#;

    assert_eq!(
        serde_json::from_value::<f64>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        0_0 as f64
    );
}

#[test]
pub fn underscore_beside_dot_test() {
    let jsonh: &str = r#"
[0_.0, 0._0]
"#;

    assert_eq!(
        serde_json::from_value::<Vec<String>>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        ["0_.0", "0._0"]
    );
}

#[test]
pub fn multi_quoted_string_with_non_ascii_indents_test() {
    let jsonh: &str = r#"
    　
    """
    　　 a
    　　"""
"#;

    assert_eq!(
        serde_json::from_value::<String>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        " a"
    );
}

#[test]
pub fn join_cr_lf_in_multi_quoted_string_test() {
    let jsonh: &str = " ''' \\r\\nHello\r\n ''' ";

    assert_eq!(
        serde_json::from_value::<String>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        "Hello"
    );
}

#[test]
pub fn massive_numbers_test() {
    let jsonh: &str = r#"
[
    0x999_999_999_999_999_999_999_999,
    0x999_999_999_999_999_999_999_999.0,
]
"#;

    assert_eq!(
        serde_json::from_value::<Vec<f64>>(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap()).unwrap(),
        [
            47_536_897_508_558_602_556_126_370_201.0,
            47_536_897_508_558_602_556_126_370_201.0,
        ]
    );
}