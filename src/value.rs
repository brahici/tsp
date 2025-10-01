use chrono::{DateTime, Utc};
use std::str::FromStr;

use crate::parse::ParseResult;

#[derive(Debug, PartialEq)]
pub enum ValueError {
    NotAnInt,
    NotATS,
    Nothing,
}
impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueError::NotAnInt => write!(f, "the value is not an integer"),
            ValueError::NotATS => write!(f, "the value is not a timestamp"),
            ValueError::Nothing => write!(f, "can't interpret the value"),
        }
    }
}

pub fn ts_from_str(ts_str: String) -> Result<DateTime<Utc>, ValueError> {
    if let Ok(input) = ParseResult::from_str(&ts_str) {
        if let Ok(ts) = input.ts.parse() {
            let parse_fn = input.unit.get_parser();
            if let Some(dt) = parse_fn(ts) {
                Ok(dt)
            } else {
                Err(ValueError::NotATS)
            }
        } else {
            Err(ValueError::NotAnInt)
        }
    } else {
        Err(ValueError::Nothing)
    }
}

#[cfg(test)]
mod test {
    use crate::value::{ts_from_str, ValueError};
    use chrono::{DateTime, Utc};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_value_display() {
        let err_not_an_int = ValueError::NotAnInt;
        assert_eq!(format!("{err_not_an_int}"), "the value is not an integer");

        let err_not_a_ts = ValueError::NotATS;
        assert_eq!(format!("{err_not_a_ts}"), "the value is not a timestamp");

        let err_nothing = ValueError::Nothing;
        assert_eq!(format!("{err_nothing}"), "can't interpret the value");
    }

    #[test]
    fn test_ts_from_str_ok() {
        let now = SystemTime::now();
        let unow: DateTime<Utc> = now.into();
        let ns_ts = now.duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let ts_str = format!("n{}", ns_ts);

        let dt = ts_from_str(ts_str).unwrap();
        assert_eq!(dt, unow);
    }

    #[test]
    fn test_ts_from_str_error_nothing() {
        let error = ts_from_str("xxxx".to_string()).unwrap_err();
        assert_eq!(error, ValueError::Nothing);
    }

    #[test]
    fn test_ts_from_str_error_not_an_integer() {
        let error = ts_from_str("mxxxx".to_string()).unwrap_err();
        assert_eq!(error, ValueError::NotAnInt);
    }

    #[test]
    fn test_ts_from_str_error_not_a_timestamp() {
        let error = ts_from_str("s100000000000000000".to_string()).unwrap_err();
        assert_eq!(error, ValueError::NotATS);
    }
}
