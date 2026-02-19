use jsonh_rs::*;

#[test]
pub fn basic_object_test() {
    let jsonh = r#"
        {
            "a": "b"
        }
        "#;
    let mut reader: JsonhReader<'_> = JsonhReader::from_str(jsonh, JsonhReaderOptions::new());
    let tokens: Vec<Result<JsonhToken, &str>> = reader.read_element().collect();

    for token in &tokens {
        assert!(token.is_ok());
    }
    assert_eq!(tokens[0].as_ref().unwrap().json_type, JsonTokenType::StartObject);
    assert_eq!(tokens[1].as_ref().unwrap().json_type, JsonTokenType::PropertyName);
    assert_eq!(tokens[1].as_ref().unwrap().value, "a");
    assert_eq!(tokens[2].as_ref().unwrap().json_type, JsonTokenType::String);
    assert_eq!(tokens[2].as_ref().unwrap().value, "b");
    assert_eq!(tokens[3].as_ref().unwrap().json_type, JsonTokenType::EndObject);
}

#[test]
pub fn nestable_block_comment_test() {
    let jsonh = r#"
        /* */
        /=* *=/
        /==*/=**=/*==/
        /=*/==**==/*=/
        0
        "#;
    let mut reader: JsonhReader<'_> = JsonhReader::from_str(jsonh, JsonhReaderOptions::new());
    let tokens: Vec<Result<JsonhToken, &str>> = reader.read_element().collect();

    for token in &tokens {
        assert!(token.is_ok());
    }
    assert_eq!(tokens[0].as_ref().unwrap().json_type, JsonTokenType::Comment);
    assert_eq!(tokens[0].as_ref().unwrap().value, " ");
    assert_eq!(tokens[1].as_ref().unwrap().json_type, JsonTokenType::Comment);
    assert_eq!(tokens[1].as_ref().unwrap().value, " ");
    assert_eq!(tokens[2].as_ref().unwrap().json_type, JsonTokenType::Comment);
    assert_eq!(tokens[2].as_ref().unwrap().value, "/=**=/");
    assert_eq!(tokens[3].as_ref().unwrap().json_type, JsonTokenType::Comment);
    assert_eq!(tokens[3].as_ref().unwrap().value, "/==**==/");
    assert_eq!(tokens[4].as_ref().unwrap().json_type, JsonTokenType::Number);
    assert_eq!(tokens[4].as_ref().unwrap().value, "0");

    let mut reader2: JsonhReader<'_> = JsonhReader::from_str(jsonh, JsonhReaderOptions::new()
        .with_version(JsonhVersion::V1)
    );
    let tokens2: Vec<Result<JsonhToken, &str>> = reader2.read_element().collect();

    assert!(tokens2[1].as_ref().is_err());
}