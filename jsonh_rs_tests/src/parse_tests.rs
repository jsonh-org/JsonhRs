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

#[test]
pub fn array_test() {
    let jsonh: &str = r#"
        [
            1, 2,
            3
            4 5,6
        ]
        "#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element.as_array().unwrap().len(), 5);
    assert_eq!(element.as_array().unwrap()[0].as_f64().unwrap(), 1.0);
    assert_eq!(element.as_array().unwrap()[1].as_f64().unwrap(), 2.0);
    assert_eq!(element.as_array().unwrap()[2].as_f64().unwrap(), 3.0);
    assert_eq!(element.as_array().unwrap()[3].as_str().unwrap(), "4 5");
    assert_eq!(element.as_array().unwrap()[4].as_f64().unwrap(), 6.0);
}

#[test]
pub fn number_parser_test() {
    assert_eq!(JsonhNumberParser::parse("1.2e3.4".to_string()).unwrap().trunc(), 3014 as f64);
}

#[test]
pub fn braceless_object_test() {
    let jsonh: &str = r#"
        a: b
        c: d
        "#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element.as_object().unwrap().len(), 2);
    assert_eq!(element.as_object().unwrap()["a"].as_str().unwrap(), "b");
    assert_eq!(element.as_object().unwrap()["c"].as_str().unwrap(), "d");
}

#[test]
pub fn comment_test() {
    let jsonh: &str = r#"
        [
            1 # hash comment
            2 // line comment
            3 /* block comment */,4
        ]
        "#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element.as_array().unwrap(), &[Value::from(1.0), Value::from(2.0), Value::from(3.0), Value::from(4.0)]);
}

#[test]
pub fn verbatim_string_test() {
    let jsonh: &str = r#"
        {
            a\\: b\\
            @c\\: @d\\
            @e\\: f\\
        }
        "#;
    let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element.as_object().unwrap().len(), 3);
    assert_eq!(element.as_object().unwrap()["a\\"].as_str().unwrap(), "b\\");
    assert_eq!(element.as_object().unwrap()["c\\\\"].as_str().unwrap(), "d\\\\");
    assert_eq!(element.as_object().unwrap()["e\\\\"].as_str().unwrap(), "f\\");

    let element2: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()
        .with_version(JsonhVersion::V1)
    ).unwrap();
    assert_eq!(element2.as_object().unwrap().len(), 3);
    assert_eq!(element2.as_object().unwrap()["a\\"].as_str().unwrap(), "b\\");
    assert_eq!(element2.as_object().unwrap()["@c\\"].as_str().unwrap(), "@d\\");
    assert_eq!(element2.as_object().unwrap()["@e\\"].as_str().unwrap(), "f\\");

    let jsonh2: &str = r#"
        @"a\\": @'''b\\'''
        "#;
    let element3: Value = JsonhReader::parse_element_from_str(jsonh2, JsonhReaderOptions::new()).unwrap();

    assert_eq!(element3.as_object().unwrap().len(), 1);
    assert_eq!(element3.as_object().unwrap()["a\\\\"].as_str().unwrap(), "b\\\\");
}

#[test]
pub fn parse_single_element_test() {
    let jsonh: &str = r#"
        1
        2
        "#;
    let element: f64 = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap().as_f64().unwrap();

    assert_eq!(element, 1.0);

    assert_eq!(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()
        .with_parse_single_element(true)
    ).is_err(), true);

    let jsonh2: &str = r#"
        1


        "#;

    assert_eq!(JsonhReader::parse_element_from_str(jsonh2, JsonhReaderOptions::new()
        .with_parse_single_element(true)
    ).is_err(), false);
}

#[test]
pub fn big_numbers_test() {
    // serde_json::Value does not support 1e99999 (infinity)

    let jsonh: &str = r#"
        [
            3.5,
            1e99999,
            999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999
        ]
        "#;

    // Rust's serde_json::Value does not support infinity
    assert_eq!(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).is_err(), true);
}

#[test]
pub fn max_depth_test() {
    let jsonh: &str = r#"
        {
            a: {
            b: {
                c: ""
            }
            d: {
            }
            }
        }
        "#;

    assert_eq!(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()
        .with_max_depth(2)
    ).is_err(), true);

    assert_eq!(JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()
        .with_max_depth(3)
    ).is_err(), false);
}