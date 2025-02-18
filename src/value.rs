use std::ops;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Undefined,
    Nan,
    Infinity(bool), // +/- Infinity; true = -ve
    Null,
    Boolean(bool),
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Number(a), Value::String(b)) => Value::String(format!("{a}{b}")),
            (Value::Number(_), Value::Undefined) => Value::Nan,
            (Value::Number(_), Value::Nan) => Value::Nan,
            (Value::Number(_), Value::Infinity(neg)) => Value::Infinity(neg),
            (Value::Number(a), Value::Null) => Value::Number(a),
            (Value::Number(a), Value::Boolean(b)) => Value::Number(a + if b { 1.0 } else { 0.0 }),
 
            (Value::String(a), Value::Number(b)) => Value::String(format!("{a}{b}")),
            (Value::String(a), Value::String(b)) => Value::String(format!("{a}{b}")),
            (Value::String(a), Value::Undefined) => Value::String(format!("{a}undefined")),
            (Value::String(a), Value::Nan) => Value::String(format!("{a}NaN")),
            (Value::String(a), Value::Infinity(neg)) => Value::String(format!("{a}{}Infinity", if neg { "-" } else { "" })),
            (Value::String(a), Value::Null) => Value::String(format!("{a}null")),
            (Value::String(a), Value::Boolean(b)) => Value::String(format!("{a}{b}")),
 
            (Value::Undefined, Value::Number(_)) => Value::Nan,
            (Value::Undefined, Value::String(b)) => Value::String(format!("undefined{b}")),
            (Value::Undefined, Value::Undefined) => Value::Nan,
            (Value::Undefined, Value::Nan) => Value::Nan,
            (Value::Undefined, Value::Infinity(_)) => Value::Nan,
            (Value::Undefined, Value::Null) => Value::Nan,
            (Value::Undefined, Value::Boolean(_)) => Value::Nan,
 
            (Value::Nan, Value::Number(_)) => Value::Nan,
            (Value::Nan, Value::String(b)) => Value::String(format!("NaN{b}")),
            (Value::Nan, Value::Undefined) => Value::Nan,
            (Value::Nan, Value::Nan) => Value::Nan,
            (Value::Nan, Value::Infinity(_)) => Value::Nan,
            (Value::Nan, Value::Null) => Value::Nan,
            (Value::Nan, Value::Boolean(_)) => Value::Nan,

            (Value::Infinity(neg), Value::Number(_)) => Value::Infinity(neg),
            (Value::Infinity(neg), Value::String(b)) => Value::String(format!("{}Infinity{b}", if neg { "-" } else { "" })),
            (Value::Infinity(_), Value::Undefined) => Value::Nan,
            (Value::Infinity(_), Value::Nan) => Value::Nan,
            (Value::Infinity(a), Value::Infinity(b)) => if a == b { Value::Infinity(a) } else { Value::Nan },
            (Value::Infinity(neg), Value::Null) => Value::Infinity(neg),
            (Value::Infinity(neg), Value::Boolean(_)) => Value::Infinity(neg),

            (Value::Null, Value::Number(b)) => Value::Number(b),
            (Value::Null, Value::String(b)) => Value::String(format!("null{b}")),
            (Value::Null, Value::Undefined) => Value::Nan,
            (Value::Null, Value::Nan) => Value::Nan,
            (Value::Null, Value::Infinity(neg)) => Value::Infinity(neg),
            (Value::Null, Value::Null) => Value::Number(0.0),
            (Value::Null, Value::Boolean(b)) => Value::Number(0.0) + Value::Boolean(b),

            (Value::Boolean(a), Value::Number(b)) => Value::Number(b) + Value::Boolean(a),
            (Value::Boolean(a), Value::String(b)) => Value::String(format!("{a}{b}")),
            (Value::Boolean(_), Value::Undefined) => Value::Nan,
            (Value::Boolean(_), Value::Nan) => Value::Nan,
            (Value::Boolean(_), Value::Infinity(neg)) => Value::Infinity(neg),
            (Value::Boolean(a), Value::Null) => Value::Boolean(a) + Value::Number(0.0),
            (Value::Boolean(a), Value::Boolean(b)) => Value::Number(if a { 1.0 } else { 0.0 }) +  Value::Boolean(b),
        }
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            (Value::Number(a), Value::String(b)) => {
                if b.is_empty() {
                    return Value::Number(a); // 2 - '' = 2
                }
                match b.parse::<f64>() {
                    Err(_) => Value::Nan, // 9 - 'a' = NaN
                    Ok(parsed) => Value::Number(a - parsed), // 9 - '3' = 6
                }
            },
            (Value::Number(_), Value::Undefined) => Value::Nan,
            (Value::Number(_), Value::Nan) => Value::Nan,
            (Value::Number(_), Value::Infinity(neg)) => Value::Infinity(!neg),
            (Value::Number(a), Value::Null) => Value::Number(a),
            (Value::Number(a), Value::Boolean(b)) => Value::Number(a - if b { 1.0 } else { 0.0 }),
 
            (Value::String(a), Value::Number(b)) => {
                if a.is_empty() {
                    return Value::Number(-b); // '' - 2 = -2
                }
                match a.parse::<f64>() {
                    Err(_) => Value::Nan, // 'a' - 1 = NaN
                    Ok(parsed) => Value::Number(parsed - b), // '9' - 3 = 6
                }
            },
            (Value::String(a), Value::String(b)) => {
                let parse_a_result = a.parse::<f64>();
                let parse_b_result = b.parse::<f64>();
                if a.is_empty() && b.is_empty() {
                    Value::Number(0.0) // '' - '' = 0
                } else if a.is_empty() && parse_b_result.is_ok() {
                    let parse_b = parse_b_result.unwrap();
                    Value::Number(-parse_b) // '' - '2' = -2
                } else if b.is_empty() && parse_a_result.is_ok() {
                    let parse_a = parse_a_result.unwrap();
                    Value::Number(parse_a) // '2' - '' = 2
                } else if parse_a_result.is_ok() && parse_b_result.is_ok() {
                    let parse_a = parse_a_result.unwrap();
                    let parse_b = parse_b_result.unwrap();
                    Value::Number(parse_a - parse_b) // '12' - '3' = 9
                } else {
                    Value::Nan // 'a' - '1' = NaN
                }
            },
            (Value::String(_), Value::Undefined) => Value::Nan,
            (Value::String(_), Value::Nan) => Value::Nan,
            (Value::String(a), Value::Infinity(neg)) => {
                if a.is_empty() {
                    return Value::Infinity(!neg); // '' - Infinity = -Infinity;
                }
                match a.parse::<f64>() {
                    Err(_) => Value::Nan, // 'a' - Infinity; 'a' - -Infinity;
                    Ok(_) => Value::Infinity(!neg), // '1' - Infinity = -Infinity; '1' - -Infinity = Infinity;
                }
            },
            (Value::String(a), Value::Null) => Value::String(a) - Value::Number(0.0), // Considering null as 0
            (Value::String(a), Value::Boolean(b)) => Value::String(a) - Value::Number(if b { 1.0 } else { 0.0 }),
 
            (Value::Undefined, _) => Value::Nan,
            (Value::Nan, _) => Value::Nan,

            (Value::Infinity(neg), Value::Number(_)) => Value::Infinity(neg),
            (Value::Infinity(neg), Value::String(b)) => {
                if b.is_empty() {
                    return Value::Infinity(neg) // Infinity - '' = Infinity;
                }
                match b.parse::<f64>() {
                    Err(_) => Value::Nan, // Infinity - 'a' = NaN
                    Ok(_) => Value::Infinity(neg), // -Infinity - '1' = -Infinity
                }
            },
            (Value::Infinity(_), Value::Undefined) => Value::Nan,
            (Value::Infinity(_), Value::Nan) => Value::Nan,
            (Value::Infinity(a), Value::Infinity(b)) => {
                if a != b {
                    return Value::Infinity(a); // -Infinity - Infinity = -Infinity; Infinity - -Infinity = -Infinity;
                }
                Value::Nan // Infinity - Infinity = NaN; -Infinity - -Infinity = NaN;
            },
            (Value::Infinity(neg), Value::Null) => Value::Infinity(neg),
            (Value::Infinity(neg), Value::Boolean(_)) => Value::Infinity(neg),

            (Value::Null, Value::Number(b)) => Value::Number(-b),
            (Value::Null, Value::String(b)) => Value::Number(0.0) - Value::String(b),
            (Value::Null, Value::Undefined) => Value::Nan,
            (Value::Null, Value::Nan) => Value::Nan,
            (Value::Null, Value::Infinity(neg)) => Value::Infinity(!neg),
            (Value::Null, Value::Null) => Value::Number(0.0),
            (Value::Null, Value::Boolean(b)) => Value::Number(0.0) - Value::Boolean(b),

            (Value::Boolean(a), Value::Number(b)) => Value::Number(if a { 1.0 } else { 0.0 } - b),
            (Value::Boolean(a), Value::String(b)) => Value::Number(if a { 1.0 } else { 0.0 }) - Value::String(b),
            (Value::Boolean(_), Value::Undefined) => Value::Nan,
            (Value::Boolean(_), Value::Nan) => Value::Nan,
            (Value::Boolean(_), Value::Infinity(neg)) => Value::Infinity(!neg),
            (Value::Boolean(a), Value::Null) => Value::Boolean(a) - Value::Number(0.0),
            (Value::Boolean(a), Value::Boolean(b)) => Value::Number(if a { 1.0 } else { 0.0 }) -  Value::Boolean(b),
        }
    }
}

impl ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (Value::Number(a), Value::String(b)) => {
                if b.is_empty() {
                    return Value::Number(0.0); // '' * 2 = 0
                }
                match b.parse::<f64>() {
                    Err(_) => Value::Nan, // 2 * 'a' = NaN
                    Ok(parsed) => Value::Number(a * parsed), // 2 * '5' = 10
                }
            },
            (Value::Number(_), Value::Undefined) => Value::Nan,
            (Value::Number(_), Value::Nan) => Value::Nan,
            (Value::Number(a), Value::Infinity(neg)) => {
                if a == 0.0 {
                    return Value::Nan; // 0 * Infinity = NaN; 0 * -Infinity = NaN;
                } else if a < 0.0 {
                    return Value::Infinity(!neg); // -2 * Infinity = -Infinity; -2 * -Infinity = Infinity;
                }
                Value::Infinity(neg) // 2 * Infinity = Infinity; 2 * -Infinity = -Infinity;
            },
            (Value::Number(_), Value::Null) => Value::Number(0.0),
            (Value::Number(a), Value::Boolean(b)) => Value::Number(a * if b { 1.0 } else { 0.0 }),

            (Value::String(a), Value::Number(b)) => {
                if a.is_empty() {
                    return Value::Number(0.0); // 2 * '' = 0
                }
                match a.parse::<f64>() {
                    Err(_) => Value::Nan, // 'a' * 2 = NaN
                    Ok(parsed) => Value::Number(parsed * b), // '5' * 2 = 10
                }
            },
            (Value::String(a), Value::String(b)) => {
                let parse_a_result = a.parse::<f64>();
                let parse_b_result = b.parse::<f64>();
                if (a.is_empty() && b.is_empty()) || (a.is_empty() && parse_b_result.is_ok()) || (b.is_empty() && parse_a_result.is_ok()) {
                    // '' * '' = 0
                    // '' * '2' = 0
                    // '2' * '' = 0
                    Value::Number(0.0) 
                } else if parse_a_result.is_ok() && parse_b_result.is_ok() {
                    let parse_a = parse_a_result.unwrap();
                    let parse_b = parse_b_result.unwrap();
                    Value::Number(parse_a * parse_b) // '2' * '3' = 6
                } else {
                    Value::Nan // 'a' * '1' = NaN
                }
            },
            (Value::String(_), Value::Undefined) => Value::Nan,
            (Value::String(_), Value::Nan) => Value::Nan,
            (Value::String(a), Value::Infinity(neg)) => {
                if a.is_empty() {
                    return Value::Nan;
                }
                match a.parse::<f64>() {
                    Err(_) => Value::Nan, // 'a' * Infinity = NaN; 'a' * -Infinity = NaN;
                    Ok(parsed) => Value::Number(parsed) * Value::Infinity(neg), // Considering parsed value as number!
                }
            },
            (Value::String(a), Value::Null) => Value::String(a) * Value::Number(0.0), // Considering 'null' as 0
            (Value::String(a), Value::Boolean(b)) => Value::String(a) * Value::Number(if b { 1.0 } else { 0.0 }),
 
            (Value::Undefined, _) => Value::Nan,
            (Value::Nan, _) => Value::Nan,

            (Value::Infinity(neg), Value::Number(b)) => Value::Number(b) * Value::Infinity(neg),
            (Value::Infinity(neg), Value::String(b)) => Value::String(b) * Value::Infinity(neg),
            (Value::Infinity(_), Value::Undefined) => Value::Nan,
            (Value::Infinity(_), Value:: Nan) => Value::Nan,
            (Value::Infinity(a), Value::Infinity(b)) => Value::Infinity(a ^ b),
            (Value::Infinity(_), Value::Null) => Value::Nan,
            (Value::Infinity(neg), Value::Boolean(b)) => Value::Infinity(neg) * Value::Number(if b { 1.0 } else { 0.0 }),

            (Value::Null, Value::Number(_)) => Value::Number(0.0),
            (Value::Null, Value::String(b)) => Value::String(b) * Value::Null,
            (Value::Null, Value::Undefined) => Value::Nan,
            (Value::Null, Value::Nan) => Value::Nan,
            (Value::Null, Value::Infinity(_)) => Value::Nan,
            (Value::Null, Value::Null) => Value::Number(0.0),
            (Value::Null, Value::Boolean(_)) => Value::Number(0.0),

            (Value::Boolean(a), Value::Number(b)) => Value::Number(b) * Value::Boolean(a),
            (Value::Boolean(a), Value::String(b)) => Value::String(b) * Value::Boolean(a),
            (Value::Boolean(_), Value::Undefined) => Value::Nan,
            (Value::Boolean(_), Value::Nan) => Value::Nan,
            (Value::Boolean(a), Value::Infinity(b)) => Value::Infinity(b) * Value::Boolean(a),
            (Value::Boolean(_), Value::Null) => Value::Number(0.0),
            (Value::Boolean(a), Value::Boolean(b)) => Value::Number(if a && b { 1.0 } else { 0.0 }),
        }
    }
}

impl ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => {
                if a == 0.0 && b == 0.0 {
                    return Value::Nan; // 0 / 0 = NaN
                } else if b == 0.0 {
                    return Value::Infinity(a < 0.0); // 1 / 0 = Infinity; -1 / 0 = -Infinity;
                }
                Value::Number(a / b)
            },
            (Value::Number(a), Value::String(b)) => {
                if b.is_empty() && a != 0.0 {
                    return Value::Infinity(a < 0.0); // 1 / '' = Infinity; -1 / '' = -Infinity;
                } else if b.is_empty() {
                    return Value::Nan; // 0 / '' = NaN
                }
                match b.parse::<f64>() {
                    Err(_) => Value::Nan, // 9 / 'a' = NaN
                    Ok(parsed) => Value::Number(a) / Value::Number(parsed), // 9 / '3' = 3
                }
            },
            (Value::Number(_), Value::Undefined) => Value::Nan,
            (Value::Number(_), Value::Nan) => Value::Nan,
            (Value::Number(_), Value::Infinity(_)) => Value::Number(0.0),
            (Value::Number(a), Value::Null) => Value::Number(a) / Value::Number(0.0),
            (Value::Number(a), Value::Boolean(b)) => Value::Number(a) / Value::Number(if b { 1.0 } else { 0.0 }),
 
            (Value::String(a), Value::Number(b)) => {
                let parse_a_result = a.parse::<f64>();
                if a.is_empty() && b != 0.0 {
                    return Value::Number(0.0); // '' / 2 = 0
                } else if parse_a_result.is_err() {
                    return Value::Nan; // '' / 0; 'a' / 2;
                }
                let parse_a = parse_a_result.unwrap();
                Value::Number(parse_a) / Value::Number(b) // Considering parsed value as number!
            },
            (Value::String(a), Value::String(b)) => {
                let parse_a_result = a.parse::<f64>();
                let parse_b_result = b.parse::<f64>();
                if a.is_empty() && b.is_empty() {
                    Value::Nan // '' / '' = NaN
                } else if parse_a_result.is_ok() && parse_b_result.is_ok() {
                    let parse_a = parse_a_result.unwrap();
                    let parse_b = parse_b_result.unwrap();
                    Value::Number(parse_a) / Value::Number(parse_b) // Considering parsed value as number!
                } else if parse_b_result.is_ok() {
                    let parse_b = parse_b_result.unwrap();
                    Value::String(a) / Value::Number(parse_b) // Considering parsed value as number!
                } else if parse_a_result.is_ok() {
                    let parse_a = parse_a_result.unwrap();
                    Value::Number(parse_a) / Value::String(b) // Considering parsed value as number!
                } else {
                    Value::Nan // 'a' / 'b' = NaN
                }
            },
            (Value::String(_), Value::Undefined) => Value::Nan,
            (Value::String(_), Value::Nan) => Value::Nan,
            (Value::String(a), Value::Infinity(_)) => {
                if a.is_empty() {
                    return Value::Number(0.0); // '' / Infinity = 0; '' / -Infinity = 0;
                }
                match a.parse::<f64>() {
                    Err(_) => Value::Nan, // 'a' / Infinity = NaN
                    Ok(_) => Value::Number(0.0), // '123' / Infinity = 0
                }
            },
            (Value::String(a), Value::Null) => Value::String(a) / Value::Number(0.0),
            (Value::String(a), Value::Boolean(b)) => Value::String(a) / Value::Number(if b { 1.0 } else { 0.0 }),
 
            (Value::Undefined, _) => Value::Nan,
            (Value::Nan, _) => Value::Nan,

            (Value::Infinity(neg), Value::Number(b)) => Value::Infinity(neg ^ (b < 0.0)),
            (Value::Infinity(neg), Value::String(b)) => {
                if b.is_empty() {
                    return Value::Infinity(neg); // Infinity / '' = Infinity;
                }
                match b.parse::<f64>() {
                    Err(_) => Value::Nan,
                    Ok(parsed) => Value::Infinity(neg) / Value::Number(parsed), // Considering parsed value as number!
                }
            },
            (Value::Infinity(_), Value::Undefined) => Value::Nan,
            (Value::Infinity(_), Value::Nan) => Value::Nan,
            (Value::Infinity(_), Value::Infinity(_)) => Value::Nan,
            (Value::Infinity(neg), Value::Null) => Value::Infinity(neg),
            (Value::Infinity(neg), Value::Boolean(_)) => Value::Infinity(neg),

            (Value::Null, Value::Number(b)) => Value::Number(0.0) / Value::Number(b),
            (Value::Null, Value::String(b)) => Value::Number(0.0) / Value::String(b),
            (Value::Null, Value::Undefined) => Value::Nan,
            (Value::Null, Value::Nan) => Value::Nan,
            (Value::Null, Value::Infinity(_)) => Value::Number(0.0),
            (Value::Null, Value::Null) => Value::Nan,
            (Value::Null, Value::Boolean(b)) => Value::Number(0.0) / Value::Boolean(b),

            (Value::Boolean(a), Value::Number(b)) => Value::Number(if a { 1.0 } else { 0.0 }) / Value::Number(b),
            (Value::Boolean(a), Value::String(b)) => Value::Number(if a { 1.0 } else { 0.0 }) / Value::String(b),
            (Value::Boolean(_), Value::Undefined) => Value::Nan,
            (Value::Boolean(_), Value::Nan) => Value::Nan,
            (Value::Boolean(_), Value::Infinity(_)) => Value::Number(0.0),
            (Value::Boolean(a), Value::Null) => Value::Boolean(a) / Value::Number(0.0),
            (Value::Boolean(a), Value::Boolean(b)) => Value::Number(if a { 1.0 } else { 0.0 }) / Value::Boolean(b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_values() {
        assert_eq!(Value::Number(1.0) + Value::Number(2.0), Value::Number(3.0));
        assert_eq!(Value::Number(1.0) + Value::String("a".to_string()), Value::String("1a".to_string()));
        assert_eq!(Value::Number(1.0) + Value::Undefined, Value::Nan);
        assert_eq!(Value::Number(1.0) + Value::Nan, Value::Nan);
        assert_eq!(Value::Number(1.0) + Value::Infinity(false), Value::Infinity(false));
        assert_eq!(Value::Number(1.0) + Value::Infinity(true), Value::Infinity(true));
        assert_eq!(Value::Number(1.0) + Value::Null, Value::Number(1.0));
        assert_eq!(Value::Number(3.0) + Value::Boolean(true), Value::Number(4.0));
        assert_eq!(Value::Number(2.0) + Value::Boolean(false), Value::Number(2.0));

        assert_eq!(Value::String("a".to_string()) + Value::Number(1.0), Value::String("a1".to_string()));
        assert_eq!(Value::String("a".to_string()) + Value::String("b".to_string()), Value::String("ab".to_string()));
        assert_eq!(Value::String("a".to_string()) + Value::Undefined, Value::String("aundefined".to_string()));
        assert_eq!(Value::String("a".to_string()) + Value::Nan, Value::String("aNaN".to_string()));
        assert_eq!(Value::String("a".to_string()) + Value::Infinity(false), Value::String("aInfinity".to_string()));
        assert_eq!(Value::String("a".to_string()) + Value::Infinity(true), Value::String("a-Infinity".to_string()));
        assert_eq!(Value::String("a".to_string()) + Value::Null, Value::String("anull".to_string()));
        assert_eq!(Value::String("1".to_string()) + Value::Boolean(true), Value::String("1true".to_string()));
        assert_eq!(Value::String("!".to_string()) + Value::Boolean(false), Value::String("!false".to_string()));

        assert_eq!(Value::Undefined + Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Undefined + Value::String("a".to_string()), Value::String("undefineda".to_string()));
        assert_eq!(Value::Undefined + Value::Undefined, Value::Nan);
        assert_eq!(Value::Undefined + Value::Nan, Value::Nan);
        assert_eq!(Value::Undefined + Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Undefined + Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Undefined + Value::Null, Value::Nan);
        assert_eq!(Value::Undefined + Value::Boolean(true), Value::Nan);
        assert_eq!(Value::Undefined + Value::Boolean(false), Value::Nan);

        assert_eq!(Value::Nan + Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Nan + Value::String("a".to_string()), Value::String("NaNa".to_string()));
        assert_eq!(Value::Nan + Value::Undefined, Value::Nan);
        assert_eq!(Value::Nan + Value::Nan, Value::Nan);
        assert_eq!(Value::Nan + Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Nan + Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Nan + Value::Null, Value::Nan);
        assert_eq!(Value::Nan + Value::Boolean(true), Value::Nan);
        assert_eq!(Value::Nan + Value::Boolean(false), Value::Nan);

        assert_eq!(Value::Infinity(false) + Value::Number(2.0), Value::Infinity(false));
        assert_eq!(Value::Infinity(true) + Value::Number(2.0), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) + Value::String("a".to_string()), Value::String("Infinitya".to_string()));
        assert_eq!(Value::Infinity(true) + Value::String("a".to_string()), Value::String("-Infinitya".to_string()));
        assert_eq!(Value::Infinity(false) + Value::Undefined, Value::Nan);
        assert_eq!(Value::Infinity(true) + Value::Undefined, Value::Nan);
        assert_eq!(Value::Infinity(false) + Value::Nan, Value::Nan);
        assert_eq!(Value::Infinity(true) + Value::Nan, Value::Nan);
        assert_eq!(Value::Infinity(false) + Value::Infinity(false), Value::Infinity(false));
        assert_eq!(Value::Infinity(true) + Value::Infinity(true), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) + Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Infinity(false) + Value::Null, Value::Infinity(false));
        assert_eq!(Value::Infinity(false) + Value::Boolean(true), Value::Infinity(false));
        assert_eq!(Value::Infinity(true) + Value::Boolean(false), Value::Infinity(true));

        assert_eq!(Value::Null + Value::Number(-2.0), Value::Number(-2.0));
        assert_eq!(Value::Null + Value::String("!".to_string()), Value::String("null!".to_string()));
        assert_eq!(Value::Null + Value::Undefined, Value::Nan);
        assert_eq!(Value::Null + Value::Nan, Value::Nan);
        assert_eq!(Value::Null + Value::Infinity(true), Value::Infinity(true));
        assert_eq!(Value::Null + Value::Null, Value::Number(0.0));
        assert_eq!(Value::Null + Value::Boolean(true), Value::Number(1.0));
        assert_eq!(Value::Null + Value::Boolean(false), Value::Number(0.0));

        assert_eq!(Value::Boolean(true) + Value::Number(2.0), Value::Number(3.0));
        assert_eq!(Value::Boolean(false) + Value::Number(2.0), Value::Number(2.0));
        assert_eq!(Value::Boolean(true) + Value::String("!".to_string()), Value::String("true!".to_string()));
        assert_eq!(Value::Boolean(false) + Value::String("2".to_string()), Value::String("false2".to_string()));
        assert_eq!(Value::Boolean(false) + Value::Undefined, Value::Nan);
        assert_eq!(Value::Boolean(true) + Value::Nan, Value::Nan);
        assert_eq!(Value::Boolean(true) + Value::Infinity(true), Value::Infinity(true));
        assert_eq!(Value::Boolean(false) + Value::Null, Value::Number(0.0));
        assert_eq!(Value::Boolean(true) + Value::Null, Value::Number(1.0));
        assert_eq!(Value::Boolean(false) + Value::Boolean(false), Value::Number(0.0));
        assert_eq!(Value::Boolean(true) + Value::Boolean(false), Value::Number(1.0));
        assert_eq!(Value::Boolean(true) + Value::Boolean(true), Value::Number(2.0));
    }

    #[test]
    fn test_sub_values() {
        assert_eq!(Value::Number(1.0) - Value::Number(2.0), Value::Number(-1.0));
        assert_eq!(Value::Number(1.0) - Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Number(1.0) - Value::String("2".to_string()), Value::Number(-1.0));
        assert_eq!(Value::Number(1.0) - Value::String("".to_string()), Value::Number(1.0));
        assert_eq!(Value::Number(1.0) - Value::Undefined, Value::Nan);
        assert_eq!(Value::Number(1.0) - Value::Nan, Value::Nan);
        assert_eq!(Value::Number(1.0) - Value::Infinity(false), Value::Infinity(true));
        assert_eq!(Value::Number(1.0) - Value::Infinity(true), Value::Infinity(false));
        assert_eq!(Value::Number(2.0) - Value::Null, Value::Number(2.0));
        assert_eq!(Value::Number(3.0) - Value::Boolean(true), Value::Number(2.0));
        assert_eq!(Value::Number(3.0) - Value::Boolean(false), Value::Number(3.0));

        assert_eq!(Value::String("a".to_string()) - Value::Number(1.0), Value::Nan);
        assert_eq!(Value::String("2".to_string()) - Value::Number(1.0), Value::Number(1.0));
        assert_eq!(Value::String("".to_string()) - Value::Number(2.0), Value::Number(-2.0));
        assert_eq!(Value::String("a".to_string()) - Value::String("2".to_string()), Value::Nan);
        assert_eq!(Value::String("1".to_string()) - Value::String("2".to_string()), Value::Number(-1.0));
        assert_eq!(Value::String("".to_string()) - Value::String("".to_string()), Value::Number(0.0));
        assert_eq!(Value::String("a".to_string()) - Value::Undefined, Value::Nan);
        assert_eq!(Value::String("a".to_string()) - Value::Nan, Value::Nan);
        assert_eq!(Value::String("".to_string()) - Value::Infinity(false), Value::Infinity(true));
        assert_eq!(Value::String("2".to_string()) - Value::Infinity(true), Value::Infinity(false));
        assert_eq!(Value::String("a".to_string()) - Value::Infinity(false), Value::Nan);
        assert_eq!(Value::String("a".to_string()) - Value::Null, Value::Nan);
        assert_eq!(Value::String("".to_string()) - Value::Null, Value::Number(0.0));
        assert_eq!(Value::String("3".to_string()) - Value::Null, Value::Number(3.0));
        assert_eq!(Value::String("a".to_string()) - Value::Boolean(true), Value::Nan);
        assert_eq!(Value::String("2".to_string()) - Value::Boolean(false), Value::Number(2.0));
        assert_eq!(Value::String("".to_string()) - Value::Boolean(true), Value::Number(-1.0));

        assert_eq!(Value::Undefined - Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Undefined - Value::String("1".to_string()), Value::Nan);
        assert_eq!(Value::Undefined - Value::Undefined, Value::Nan);
        assert_eq!(Value::Undefined - Value::Nan, Value::Nan);
        assert_eq!(Value::Undefined - Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Undefined - Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Undefined - Value::Null, Value::Nan);
        assert_eq!(Value::Undefined - Value::Boolean(false), Value::Nan);
        assert_eq!(Value::Undefined - Value::Boolean(true), Value::Nan);

        assert_eq!(Value::Nan - Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Nan - Value::String("1".to_string()), Value::Nan);
        assert_eq!(Value::Nan - Value::Undefined, Value::Nan);
        assert_eq!(Value::Nan - Value::Nan, Value::Nan);
        assert_eq!(Value::Nan - Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Nan - Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Nan - Value::Null, Value::Nan);
        assert_eq!(Value::Nan - Value::Boolean(true), Value::Nan);
        assert_eq!(Value::Nan - Value::Boolean(false), Value::Nan);

        assert_eq!(Value::Infinity(false) - Value::Number(1.0), Value::Infinity(false));
        assert_eq!(Value::Infinity(true) - Value::Number(1.0), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) - Value::String("".to_string()), Value::Infinity(false));
        assert_eq!(Value::Infinity(true) - Value::String("".to_string()), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) - Value::String("2".to_string()), Value::Infinity(false));
        assert_eq!(Value::Infinity(true) - Value::String("2".to_string()), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) - Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Infinity(true) - Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Infinity(false) - Value::Undefined, Value::Nan);
        assert_eq!(Value::Infinity(true) - Value::Undefined, Value::Nan);
        assert_eq!(Value::Infinity(false) - Value::Nan, Value::Nan);
        assert_eq!(Value::Infinity(true) - Value::Nan, Value::Nan);
        assert_eq!(Value::Infinity(false) - Value::Infinity(true), Value::Infinity(false));
        assert_eq!(Value::Infinity(true) - Value::Infinity(false), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) - Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Infinity(true) - Value::Null, Value::Infinity(true));
        assert_eq!(Value::Infinity(false) - Value::Null, Value::Infinity(false));
        assert_eq!(Value::Infinity(true) - Value::Boolean(false), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) - Value::Boolean(true), Value::Infinity(false));

        assert_eq!(Value::Null - Value::Number(2.0), Value::Number(-2.0));
        assert_eq!(Value::Null - Value::String("".to_string()), Value::Number(0.0));
        assert_eq!(Value::Null - Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Null - Value::String("2".to_string()), Value::Number(-2.0));
        assert_eq!(Value::Null - Value::Undefined, Value::Nan);
        assert_eq!(Value::Null - Value::Nan, Value::Nan);
        assert_eq!(Value::Null - Value::Infinity(false), Value::Infinity(true));
        assert_eq!(Value::Null - Value::Null, Value::Number(0.0));
        assert_eq!(Value::Null - Value::Boolean(true), Value::Number(-1.0));

        assert_eq!(Value::Boolean(true) - Value::Number(2.0), Value::Number(-1.0));
        assert_eq!(Value::Boolean(false) - Value::Number(2.0), Value::Number(-2.0));
        assert_eq!(Value::Boolean(true) - Value::String("!".to_string()), Value::Nan);
        assert_eq!(Value::Boolean(false) - Value::String("2".to_string()), Value::Number(-2.0));
        assert_eq!(Value::Boolean(false) - Value::Undefined, Value::Nan);
        assert_eq!(Value::Boolean(true) - Value::Nan, Value::Nan);
        assert_eq!(Value::Boolean(true) - Value::Infinity(true), Value::Infinity(false));
        assert_eq!(Value::Boolean(false) - Value::Null, Value::Number(0.0));
        assert_eq!(Value::Boolean(true) - Value::Null, Value::Number(1.0));
        assert_eq!(Value::Boolean(false) - Value::Boolean(false), Value::Number(0.0));
        assert_eq!(Value::Boolean(true) - Value::Boolean(false), Value::Number(1.0));
        assert_eq!(Value::Boolean(true) - Value::Boolean(true), Value::Number(0.0));
    }

    #[test]
    fn test_mul_values() {
        assert_eq!(Value::Number(3.0) * Value::Number(2.0), Value::Number(6.0));
        assert_eq!(Value::Number(3.0) * Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Number(3.0) * Value::String("-2".to_string()), Value::Number(-6.0));
        assert_eq!(Value::Number(1.0) * Value::String("".to_string()), Value::Number(0.0));
        assert_eq!(Value::Number(1.0) * Value::Undefined, Value::Nan);
        assert_eq!(Value::Number(1.0) * Value::Nan, Value::Nan);
        assert_eq!(Value::Number(0.0) * Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Number(0.0) * Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Number(-2.0) * Value::Infinity(false), Value::Infinity(true));
        assert_eq!(Value::Number(2.0) * Value::Infinity(true), Value::Infinity(true));
        assert_eq!(Value::Number(2.0) * Value::Null, Value::Number(0.0));
        assert_eq!(Value::Number(2.0) * Value::Boolean(true), Value::Number(2.0));
        assert_eq!(Value::Number(2.0) * Value::Boolean(false), Value::Number(0.0));

        assert_eq!(Value::String("a".to_string()) * Value::Number(1.0), Value::Nan);
        assert_eq!(Value::String("2".to_string()) * Value::Number(2.0), Value::Number(4.0));
        assert_eq!(Value::String("".to_string()) * Value::Number(2.0), Value::Number(0.0));
        assert_eq!(Value::String("a".to_string()) * Value::String("2".to_string()), Value::Nan);
        assert_eq!(Value::String("-2".to_string()) * Value::String("2".to_string()), Value::Number(-4.0));
        assert_eq!(Value::String("".to_string()) * Value::String("".to_string()), Value::Number(0.0));
        assert_eq!(Value::String("a".to_string()) * Value::Undefined, Value::Nan);
        assert_eq!(Value::String("a".to_string()) * Value::Nan, Value::Nan);
        assert_eq!(Value::String("a".to_string()) * Value::Infinity(false), Value::Nan);
        assert_eq!(Value::String("a".to_string()) * Value::Infinity(true), Value::Nan);
        assert_eq!(Value::String("".to_string()) * Value::Infinity(false), Value::Nan);
        assert_eq!(Value::String("".to_string()) * Value::Infinity(true), Value::Nan);
        assert_eq!(Value::String("0".to_string()) * Value::Infinity(false), Value::Nan);
        assert_eq!(Value::String("0".to_string()) * Value::Infinity(true), Value::Nan);
        assert_eq!(Value::String("2".to_string()) * Value::Infinity(false), Value::Infinity(false));
        assert_eq!(Value::String("-2".to_string()) * Value::Infinity(false), Value::Infinity(true));
        assert_eq!(Value::String("a".to_string()) * Value::Null, Value::Nan);
        assert_eq!(Value::String("2".to_string()) * Value::Null, Value::Number(0.0));
        assert_eq!(Value::String("".to_string()) * Value::Null, Value::Number(0.0));
        assert_eq!(Value::String("a".to_string()) * Value::Boolean(false), Value::Nan);
        assert_eq!(Value::String("2".to_string()) * Value::Boolean(true), Value::Number(2.0));
        assert_eq!(Value::String("".to_string()) * Value::Boolean(true), Value::Number(0.0));

        assert_eq!(Value::Undefined * Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Undefined * Value::String("1".to_string()), Value::Nan);
        assert_eq!(Value::Undefined * Value::Undefined, Value::Nan);
        assert_eq!(Value::Undefined * Value::Nan, Value::Nan);
        assert_eq!(Value::Undefined * Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Undefined * Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Undefined * Value::Null, Value::Nan);
        assert_eq!(Value::Undefined * Value::Boolean(true), Value::Nan);
        assert_eq!(Value::Undefined * Value::Boolean(false), Value::Nan);

        assert_eq!(Value::Nan * Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Nan * Value::String("1".to_string()), Value::Nan);
        assert_eq!(Value::Nan * Value::Undefined, Value::Nan);
        assert_eq!(Value::Nan * Value::Nan, Value::Nan);
        assert_eq!(Value::Nan * Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Nan * Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Nan * Value::Null, Value::Nan);
        assert_eq!(Value::Nan * Value::Boolean(true), Value::Nan);
        assert_eq!(Value::Nan * Value::Boolean(false), Value::Nan);

        assert_eq!(Value::Infinity(false) * Value::Number(0.0), Value::Nan);
        assert_eq!(Value::Infinity(true) * Value::Number(0.0), Value::Nan);
        assert_eq!(Value::Infinity(false) * Value::Number(2.0), Value::Infinity(false));
        assert_eq!(Value::Infinity(false) * Value::Number(-2.0), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) * Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Infinity(true) * Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Infinity(false) * Value::String("".to_string()), Value::Nan);
        assert_eq!(Value::Infinity(true) * Value::String("".to_string()), Value::Nan);
        assert_eq!(Value::Infinity(false) * Value::String("0".to_string()), Value::Nan);
        assert_eq!(Value::Infinity(true) * Value::String("0".to_string()), Value::Nan);
        assert_eq!(Value::Infinity(false) * Value::String("2".to_string()), Value::Infinity(false));
        assert_eq!(Value::Infinity(false) * Value::String("-2".to_string()), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) * Value::Undefined, Value::Nan);
        assert_eq!(Value::Infinity(true) * Value::Undefined, Value::Nan);
        assert_eq!(Value::Infinity(false) * Value::Nan, Value::Nan);
        assert_eq!(Value::Infinity(true) * Value::Nan, Value::Nan);
        assert_eq!(Value::Infinity(false) * Value::Infinity(false), Value::Infinity(false));
        assert_eq!(Value::Infinity(true) * Value::Infinity(false), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) * Value::Infinity(true), Value::Infinity(true));
        assert_eq!(Value::Infinity(true) * Value::Infinity(true), Value::Infinity(false));
        assert_eq!(Value::Infinity(false) * Value::Boolean(true), Value::Infinity(false));
        assert_eq!(Value::Infinity(true) * Value::Boolean(false), Value::Nan);

        assert_eq!(Value::Null * Value::Number(2.0), Value::Number(0.0));
        assert_eq!(Value::Null * Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Null * Value::String("2".to_string()), Value::Number(0.0));
        assert_eq!(Value::Null * Value::String("".to_string()), Value::Number(0.0));
        assert_eq!(Value::Null * Value::Undefined, Value::Nan);
        assert_eq!(Value::Null * Value::Nan, Value::Nan);
        assert_eq!(Value::Null * Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Null * Value::Null, Value::Number(0.0));
        assert_eq!(Value::Null * Value::Boolean(true), Value::Number(0.0));
        assert_eq!(Value::Null * Value::Boolean(false), Value::Number(0.0));

        assert_eq!(Value::Boolean(true) * Value::Number(2.0), Value::Number(2.0));
        assert_eq!(Value::Boolean(false) * Value::Number(2.0), Value::Number(0.0));
        assert_eq!(Value::Boolean(true) * Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Boolean(false) * Value::String("2".to_string()), Value::Number(0.0));
        assert_eq!(Value::Boolean(true) * Value::String("".to_string()), Value::Number(0.0));
        assert_eq!(Value::Boolean(false) * Value::Undefined, Value::Nan);
        assert_eq!(Value::Boolean(true) * Value::Undefined, Value::Nan);
        assert_eq!(Value::Boolean(false) * Value::Nan, Value::Nan);
        assert_eq!(Value::Boolean(true) * Value::Nan, Value::Nan);
        assert_eq!(Value::Boolean(false) * Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Boolean(true) * Value::Infinity(true), Value::Infinity(true));
        assert_eq!(Value::Boolean(false) * Value::Null, Value::Number(0.0));
        assert_eq!(Value::Boolean(true) * Value::Null, Value::Number(0.0));
        assert_eq!(Value::Boolean(false) * Value::Boolean(false), Value::Number(0.0));
        assert_eq!(Value::Boolean(true) * Value::Boolean(false), Value::Number(0.0));
        assert_eq!(Value::Boolean(true) * Value::Boolean(true), Value::Number(1.0));
    }

    #[test]
    fn test_div_values() {
        assert_eq!(Value::Number(3.0) / Value::Number(4.0), Value::Number(0.75));
        assert_eq!(Value::Number(0.0) / Value::Number(0.0), Value::Nan);
        assert_eq!(Value::Number(-3.0) / Value::Number(0.0), Value::Infinity(true));
        assert_eq!(Value::Number(0.0) / Value::String("".to_string()), Value::Nan);
        assert_eq!(Value::Number(3.0) / Value::String("".to_string()), Value::Infinity(false));
        assert_eq!(Value::Number(9.0) / Value::String("-3.0".to_string()), Value::Number(-3.0));
        assert_eq!(Value::Number(3.0) / Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Number(3.0) / Value::Undefined, Value::Nan);
        assert_eq!(Value::Number(3.0) / Value::Nan, Value::Nan);
        assert_eq!(Value::Number(3.0) / Value::Infinity(false), Value::Number(0.0));
        assert_eq!(Value::Number(3.0) / Value::Infinity(true), Value::Number(0.0));
        assert_eq!(Value::Number(3.0) / Value::Null, Value::Infinity(false));
        assert_eq!(Value::Number(0.0) / Value::Null, Value::Nan);
        assert_eq!(Value::Number(2.0) / Value::Boolean(true), Value::Number(2.0));
        assert_eq!(Value::Number(0.0) / Value::Boolean(false), Value::Nan);

        assert_eq!(Value::String("".to_string()) / Value::Number(2.0), Value::Number(0.0));
        assert_eq!(Value::String("".to_string()) / Value::Number(0.0), Value::Nan);
        assert_eq!(Value::String("a".to_string()) / Value::Number(2.0), Value::Nan);
        assert_eq!(Value::String("-2".to_string()) / Value::Number(2.0), Value::Number(-1.0));
        assert_eq!(Value::String("".to_string()) / Value::String("".to_string()), Value::Nan);
        assert_eq!(Value::String("".to_string()) / Value::String("2.0".to_string()), Value::Number(0.0));
        assert_eq!(Value::String("".to_string()) / Value::String("0.0".to_string()), Value::Nan);
        assert_eq!(Value::String("a".to_string()) / Value::String("2.0".to_string()), Value::Nan);
        assert_eq!(Value::String("-2".to_string()) / Value::String("2.0".to_string()), Value::Number(-1.0));
        assert_eq!(Value::String("3.0".to_string()) / Value::String("".to_string()), Value::Infinity(false));
        assert_eq!(Value::String("9.0".to_string()) / Value::String("-3.0".to_string()), Value::Number(-3.0));
        assert_eq!(Value::String("3.0".to_string()) / Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::String("b".to_string()) / Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::String("3".to_string()) / Value::Undefined, Value::Nan);
        assert_eq!(Value::String("3".to_string()) / Value::Nan, Value::Nan);
        assert_eq!(Value::String("".to_string()) / Value::Infinity(false), Value::Number(0.0));
        assert_eq!(Value::String("".to_string()) / Value::Infinity(true), Value::Number(0.0));
        assert_eq!(Value::String("a".to_string()) / Value::Infinity(false), Value::Nan);
        assert_eq!(Value::String("3.0".to_string()) / Value::Infinity(true), Value::Number(0.0));
        assert_eq!(Value::String("".to_string()) / Value::Null, Value::Nan);
        assert_eq!(Value::String("a".to_string()) / Value::Null, Value::Nan);
        assert_eq!(Value::String("-2".to_string()) / Value::Null, Value::Infinity(true));
        assert_eq!(Value::String("".to_string()) / Value::Boolean(false), Value::Nan);
        assert_eq!(Value::String("a".to_string()) / Value::Boolean(true), Value::Nan);
        assert_eq!(Value::String("-2".to_string()) / Value::Boolean(false), Value::Infinity(true));

        assert_eq!(Value::Undefined / Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Undefined / Value::String("".to_string()), Value::Nan);
        assert_eq!(Value::Undefined / Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Undefined / Value::String("2".to_string()), Value::Nan);
        assert_eq!(Value::Undefined / Value::Undefined, Value::Nan);
        assert_eq!(Value::Undefined / Value::Nan, Value::Nan);
        assert_eq!(Value::Undefined / Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Undefined / Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Undefined / Value::Null, Value::Nan);
        assert_eq!(Value::Undefined / Value::Boolean(true), Value::Nan);

        assert_eq!(Value::Nan / Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Nan / Value::String("".to_string()), Value::Nan);
        assert_eq!(Value::Nan / Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Nan / Value::String("2".to_string()), Value::Nan);
        assert_eq!(Value::Nan / Value::Undefined, Value::Nan);
        assert_eq!(Value::Nan / Value::Nan, Value::Nan);
        assert_eq!(Value::Nan / Value::Infinity(false), Value::Nan);
        assert_eq!(Value::Nan / Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Nan / Value::Null, Value::Nan);
        assert_eq!(Value::Nan / Value::Boolean(true), Value::Nan);

        assert_eq!(Value::Infinity(false) / Value::Number(2.0), Value::Infinity(false));
        assert_eq!(Value::Infinity(false) / Value::Number(-2.0), Value::Infinity(true));
        assert_eq!(Value::Infinity(true) / Value::Number(2.0), Value::Infinity(true));
        assert_eq!(Value::Infinity(true) / Value::Number(-2.0), Value::Infinity(false));
        assert_eq!(Value::Infinity(false) / Value::String("".to_string()), Value::Infinity(false));
        assert_eq!(Value::Infinity(true) / Value::String("".to_string()), Value::Infinity(true));
        assert_eq!(Value::Infinity(true) / Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Infinity(false) / Value::String("2.0".to_string()), Value::Infinity(false));
        assert_eq!(Value::Infinity(false) / Value::String("-2.0".to_string()), Value::Infinity(true));
        assert_eq!(Value::Infinity(true) / Value::String("2.0".to_string()), Value::Infinity(true));
        assert_eq!(Value::Infinity(true) / Value::String("-2.0".to_string()), Value::Infinity(false));
        assert_eq!(Value::Infinity(false) / Value::Undefined, Value::Nan);
        assert_eq!(Value::Infinity(true) / Value::Nan, Value::Nan);
        assert_eq!(Value::Infinity(false) / Value::Infinity(true), Value::Nan);
        assert_eq!(Value::Infinity(true) / Value::Null, Value::Infinity(true));
        assert_eq!(Value::Infinity(false) / Value::Null, Value::Infinity(false));
        assert_eq!(Value::Infinity(true) / Value::Boolean(false), Value::Infinity(true));
        assert_eq!(Value::Infinity(false) / Value::Boolean(true), Value::Infinity(false));

        assert_eq!(Value::Null / Value::Number(2.0), Value::Number(0.0));
        assert_eq!(Value::Null / Value::Number(0.0), Value::Nan);
        assert_eq!(Value::Null / Value::String("".to_string()), Value::Nan);
        assert_eq!(Value::Null / Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Null / Value::String("2".to_string()), Value::Number(0.0));
        assert_eq!(Value::Null / Value::Undefined, Value::Nan);
        assert_eq!(Value::Null / Value::Nan, Value::Nan);
        assert_eq!(Value::Null / Value::Infinity(false), Value::Number(0.0));
        assert_eq!(Value::Null / Value::Infinity(true), Value::Number(0.0));
        assert_eq!(Value::Null / Value::Null, Value::Nan);
        assert_eq!(Value::Null / Value::Boolean(true), Value::Number(0.0));
        assert_eq!(Value::Null / Value::Boolean(false), Value::Nan);

        assert_eq!(Value::Boolean(true) / Value::Number(2.0), Value::Number(0.5));
        assert_eq!(Value::Boolean(false) / Value::Number(2.0), Value::Number(0.0));
        assert_eq!(Value::Boolean(true) / Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Boolean(false) / Value::String("2".to_string()), Value::Number(0.0));
        assert_eq!(Value::Boolean(true) / Value::String("".to_string()), Value::Infinity(false));
        assert_eq!(Value::Boolean(false) / Value::Undefined, Value::Nan);
        assert_eq!(Value::Boolean(true) / Value::Undefined, Value::Nan);
        assert_eq!(Value::Boolean(false) / Value::Nan, Value::Nan);
        assert_eq!(Value::Boolean(true) / Value::Nan, Value::Nan);
        assert_eq!(Value::Boolean(false) / Value::Infinity(true), Value::Number(0.0));
        assert_eq!(Value::Boolean(true) / Value::Infinity(true), Value::Number(0.0));
        assert_eq!(Value::Boolean(false) / Value::Null, Value::Nan);
        assert_eq!(Value::Boolean(true) / Value::Null, Value::Infinity(false));
        assert_eq!(Value::Boolean(false) / Value::Boolean(false), Value::Nan);
        assert_eq!(Value::Boolean(true) / Value::Boolean(false), Value::Infinity(false));
        assert_eq!(Value::Boolean(false) / Value::Boolean(true), Value::Number(0.0));
        assert_eq!(Value::Boolean(true) / Value::Boolean(true), Value::Number(1.0));
    }
}
