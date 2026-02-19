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
    pub fn parse(mut jsonh_number: String) -> Result<f64, &'static str> {
        // Remove underscores
        jsonh_number = jsonh_number.replace('_', "");
        let mut digits: &str = jsonh_number.as_str();

        // Get sign
        let mut sign: i8 = 1;
        if digits.starts_with('-') {
            sign = -1;
            digits = &digits[1..];
        }
        else if digits.starts_with('+') {
            sign = 1;
            digits = &digits[1..];
        }

        // Decimal
        let mut base_digits: &str = "0123456789";
        // Hexadecimal
        if digits.starts_with("0x") {
            base_digits = "0123456789abcdef";
            digits = &digits[2..];
        }
        // Binary
        else if digits.starts_with("0b") {
            base_digits = "01";
            digits = &digits[2..];
        }
        // Octal
        else if digits.starts_with("0o") {
            base_digits = "01234567";
            digits = &digits[2..];
        }

        // Parse number with base digits
        let mut number: f64 = match Self::parse_fractional_number_with_exponent(digits, base_digits) {
            Ok(number) => number,
            Err(number_error) => return Err(number_error),
        };

        // Apply sign
        if sign != 1 {
            number *= sign as f64;
        }
        return Ok(number);
    }

    /// Converts a fractional number with an exponent (e.g. `12.3e4.5`) from the given base (e.g. `01234567`) to a base-10 real.
    fn parse_fractional_number_with_exponent(digits: &str, base_digits: &str) -> Result<f64, &'static str> {
        // Find exponent
        let mut exponent_index: Option<usize> = None;
        // Hexadecimal exponent
        if base_digits.contains('e') {
            for (index, digit) in digits.char_indices() {
                if !matches!(digit, 'e' | 'E') {
                    continue;
                }
                let next_index: usize = index + digit.len_utf8();
                if next_index >= digits.len() || !(digits[next_index..].starts_with(['+', '-'])) {
                    continue;
                }
                exponent_index = Some(index);
                break;
            }
        }
        // Exponent
        else {
            exponent_index = digits.find(['e', 'E']);
        }

        // If no exponent then parse real
        if exponent_index.is_none() {
            return Self::parse_fractional_number(digits, base_digits);
        }

        // Get mantissa and exponent
        let mantissa_part: &str = &digits[..exponent_index.unwrap()];
        let exponent_part: &str = &digits[(exponent_index.unwrap() + 1)..];

        // Parse mantissa and exponent
        let mantissa: f64 = match Self::parse_fractional_number(mantissa_part, base_digits) {
            Ok(mantissa) => mantissa,
            Err(mantissa_error) => return Err(mantissa_error),
        };
        let exponent: f64 = match Self::parse_fractional_number(exponent_part, base_digits) {
            Ok(exponent) => exponent,
            Err(exponent_error) => return Err(exponent_error),
        };

        // Multiply mantissa by 10 ^ exponent
        return Ok(mantissa * (10 as f64).powf(exponent));
    }
    /// Converts a fractional number (e.g. `123.45`) from the given base (e.g. `01234567`) to a base-10 real.
    fn parse_fractional_number(digits: &str, base_digits: &str) -> Result<f64, &'static str> {
        // Optimization for base-10 digits
        if base_digits == "0123456789" {
            return match digits.parse() {
                Ok(number) => Ok(number),
                Err(_) => Err("Error parsing number from string"),
            };
        }

        // Find dot
        let dot_index: Option<usize> = digits.find('.');
        // If no dot then parse integer
        if dot_index.is_none() {
            return Self::parse_whole_number(digits, base_digits);
        }

        // Get parts of number
        let whole_part: &str = &digits[..dot_index.unwrap()];
        let fraction_part: &str = &digits[(dot_index.unwrap() + 1)..];

        // Parse parts of number
        let whole: f64 = match Self::parse_whole_number(whole_part, base_digits) {
            Ok(whole) => whole,
            Err(whole_error) => return Err(whole_error),
        };

        // Add each column of fraction digits
        let mut fraction: f64 = 0 as f64;
        for digit_char in fraction_part.chars().rev() {
            // Get current digit
            let digit_int: Option<usize> = base_digits.find(digit_char.to_ascii_lowercase());

            // Ensure digit is valid
            if digit_int.is_none() {
                return Err("Invalid digit");
            }

            // Add value of column
            fraction = (fraction + (digit_int.unwrap() as f64)) / (base_digits.len() as f64);
        }

        // Combine whole and fraction
        return Ok(whole + fraction);
    }
    /// Converts a whole number (e.g. `12345`) from the given base (e.g. `01234567`) to a base-10 integer.
    fn parse_whole_number(mut digits: &str, base_digits: &str) -> Result<f64, &'static str> {
        // Optimization for base-10 digits
        if base_digits == "0123456789" {
            return match digits.parse() {
                Ok(number) => Ok(number),
                Err(_) => Err("Error parsing number from string"),
            };
        }

        // Get sign
        let mut sign: i8 = 1;
        if digits.starts_with('-') {
            sign = -1;
            digits = &digits[1..];
        }
        else if digits.starts_with('+') {
            sign = 1;
            digits = &digits[1..];
        }

        // Add each column of digits
        let mut integer: f64 = 0 as f64;
        for digit_char in digits.chars() {
            // Get current digit
            let digit_int: Option<usize> = base_digits.find(digit_char.to_ascii_lowercase());

            // Ensure digit is valid
            if digit_int.is_none() {
                return Err("Invalid digit");
            }

            // Add value of column
            integer = (integer * (base_digits.len() as f64)) + (digit_int.unwrap() as f64);
        }

        // Apply sign
        if sign != 1 {
            integer *= sign as f64;
        }
        return Ok(integer);
    }
}