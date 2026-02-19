/// Methods for parsing JSONH numbers.
/// 
/// Unlike `JsonhReader::read_element()`, minimal validation is done here. Ensure the input is valid.
pub struct JsonhNumberParser {
}

impl JsonhNumberParser {
    /// Converts a JSONH number to a base-10 real.
    /// For example:
    /// 
    /// ```
    /// Input: +5.2e3.0
    /// Output: 5200
    /// ```
    pub fn parse(jsonh_number: &str) -> Result<f64, &'static str> {
        todo!();
    }

    /// Converts a fractional number with an exponent (e.g. `12.3e4.5`) from the given base (e.g. `01234567`) to a base-10 real.
    fn parse_fractional_number_with_exponent(digits: &str, base_digits: &str) -> Result<f64, &'static str> {
        todo!();
    }
    /// Converts a fractional number (e.g. `123.45`) from the given base (e.g. `01234567`) to a base-10 real.
    fn parse_fractional_number(digits: &str, base_digits: &str) -> Result<f64, &'static str> {
        todo!();
    }
    /// Converts a whole number (e.g. `12345`) from the given base (e.g. `01234567`) to a base-10 integer.
    fn parse_whole_number(digits: &str, base_digits: &str) -> Result<u64, &'static str> {
        todo!();
    }
}