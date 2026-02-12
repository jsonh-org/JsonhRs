use jsonh_rs::*;

#[test]
pub fn example_test() {
    let res = JsonhReader::from_str("input").parse_element();

    assert_eq!(res, Ok(Value::String("example".to_string())));
}