use std::ops;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Undefined,
    Nan,
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::Number(a), Value::String(b)) => Value::String(format!("{a}{b}")),
            (Value::Number(_), Value::Undefined) => Value::Nan,
            (Value::Number(_), Value::Nan) => Value::Nan,
 
            (Value::String(a), Value::Number(b)) => Value::String(format!("{a}{b}")),
            (Value::String(a), Value::String(b)) => Value::String(format!("{a}{b}")),
            (Value::String(a), Value::Undefined) => Value::String(format!("{a}undefined")),
            (Value::String(a), Value::Nan) => Value::String(format!("{a}NaN")),
 
            (Value::Undefined, Value::Number(_)) => Value::Nan,
            (Value::Undefined, Value::String(b)) => Value::String(format!("undefined{b}")),
            (Value::Undefined, Value::Undefined) => Value::Nan,
            (Value::Undefined, Value::Nan) => Value::Nan,
 
            (Value::Nan, Value::Number(_)) => Value::Nan,
            (Value::Nan, Value::String(b)) => Value::String(format!("NaN{b}")),
            (Value::Nan, Value::Undefined) => Value::Nan,
            (Value::Nan, Value::Nan) => Value::Nan,
        }
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            (Value::Number(a), Value::String(b)) => {
                match b.parse::<f64>() {
                    Err(_) => Value::Nan, // 9 - 'a' = NaN
                    Ok(parsed) => Value::Number(a - parsed), // 9 - '3' = 6
                }
            },
            (Value::Number(_), Value::Undefined) => Value::Nan,
            (Value::Number(_), Value::Nan) => Value::Nan,
 
            (Value::String(a), Value::Number(b)) => {
                match a.parse::<f64>() {
                    Err(_) => Value::Nan, // 'a' - 1 = NaN
                    Ok(parsed) => Value::Number(parsed - b), // '9' - 3 = 6
                }
            },
            (Value::String(a), Value::String(b)) => {
                match (a.parse::<f64>(), b.parse::<f64>()) {
                    (Ok(parsed_a), Ok(parsed_b)) => Value::Number(parsed_a - parsed_b), // '12' - '3' = 9
                    _ => Value::Nan, // 'a' - '1' = NaN
                }
            },
            (Value::String(_), Value::Undefined) => Value::Nan,
            (Value::String(_), Value::Nan) => Value::Nan,
 
            (Value::Undefined, _) => Value::Nan,
            (Value::Nan, _) => Value::Nan,
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

        assert_eq!(Value::String("a".to_string()) + Value::Number(1.0), Value::String("a1".to_string()));
        assert_eq!(Value::String("a".to_string()) + Value::String("b".to_string()), Value::String("ab".to_string()));
        assert_eq!(Value::String("a".to_string()) + Value::Undefined, Value::String("aundefined".to_string()));
        assert_eq!(Value::String("a".to_string()) + Value::Nan, Value::String("aNaN".to_string()));

        assert_eq!(Value::Undefined + Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Undefined + Value::String("a".to_string()), Value::String("undefineda".to_string()));
        assert_eq!(Value::Undefined + Value::Undefined, Value::Nan);
        assert_eq!(Value::Undefined + Value::Nan, Value::Nan);

        assert_eq!(Value::Nan + Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Nan + Value::String("a".to_string()), Value::String("NaNa".to_string()));
        assert_eq!(Value::Nan + Value::Undefined, Value::Nan);
        assert_eq!(Value::Nan + Value::Nan, Value::Nan);
    }

    #[test]
    fn test_sub_values() {
        assert_eq!(Value::Number(1.0) - Value::Number(2.0), Value::Number(-1.0));
        assert_eq!(Value::Number(1.0) - Value::String("a".to_string()), Value::Nan);
        assert_eq!(Value::Number(1.0) - Value::String("2".to_string()), Value::Number(-1.0));
        assert_eq!(Value::Number(1.0) - Value::Undefined, Value::Nan);
        assert_eq!(Value::Number(1.0) - Value::Nan, Value::Nan);

        assert_eq!(Value::String("a".to_string()) - Value::Number(1.0), Value::Nan);
        assert_eq!(Value::String("2".to_string()) - Value::Number(1.0), Value::Number(1.0));
        assert_eq!(Value::String("a".to_string()) - Value::String("2".to_string()), Value::Nan);
        assert_eq!(Value::String("1".to_string()) - Value::String("2".to_string()), Value::Number(-1.0));
        assert_eq!(Value::String("a".to_string()) - Value::Undefined, Value::Nan);
        assert_eq!(Value::String("a".to_string()) - Value::Nan, Value::Nan);

        assert_eq!(Value::Undefined - Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Undefined - Value::String("1".to_string()), Value::Nan);
        assert_eq!(Value::Undefined - Value::Undefined, Value::Nan);
        assert_eq!(Value::Undefined - Value::Nan, Value::Nan);

        assert_eq!(Value::Nan - Value::Number(2.0), Value::Nan);
        assert_eq!(Value::Nan - Value::String("1".to_string()), Value::Nan);
        assert_eq!(Value::Nan - Value::Undefined, Value::Nan);
        assert_eq!(Value::Nan - Value::Nan, Value::Nan);
    }
}
