use jsonh_rs::*;

#[test]
pub fn escape_sequence_test() {
    let jsonh: &str = r#"
        "\U0001F47D and \uD83D\uDC7D"
        "#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element, "👽 and 👽");
}

#[test]
pub fn quoteless_escape_sequence_test() {
    let jsonh: &str = r#"
        \U0001F47D and \uD83D\uDC7D
        "#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element, "👽 and 👽");
}

#[test]
pub fn multi_quoted_string_test() {
    let jsonh: &str = r#"
            """"
                  Hello! Here's a quote: ". Now a double quote: "". And a triple quote! """. Escape: \\\U0001F47D.
                 """"
        "#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element, " Hello! Here's a quote: \". Now a double quote: \"\". And a triple quote! \"\"\". Escape: \\👽.");
}