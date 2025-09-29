use chrono::{DateTime, Utc};
use std::str::FromStr;

use crate::parse::ParseResult;

#[derive(Debug, PartialEq)]
pub enum ProcessError {
    NotAnInt,
    NotATS,
    Nothing,
}
impl std::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessError::NotAnInt => write!(f, "the value is not an integer"),
            ProcessError::NotATS => write!(f, "the value is not a timestamp"),
            ProcessError::Nothing => write!(f, "can't interpret the value"),
        }
    }
}

pub fn ts_from_str(ts_str: String) -> Result<DateTime<Utc>, ProcessError> {
    if let Ok(input) = ParseResult::from_str(&ts_str) {
        if let Ok(ts) = input.ts.parse() {
            let parse_fn = input.unit.get_parser();
            if let Some(dt) = parse_fn(ts) {
                Ok(dt)
            } else {
                Err(ProcessError::NotATS)
            }
        } else {
            Err(ProcessError::NotAnInt)
        }
    } else {
        Err(ProcessError::Nothing)
    }
}

#[cfg(test)]
mod test {
    use crate::process::{ProcessError, ts_from_str};
    use chrono::{DateTime, Utc};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_ok() {
        let now = SystemTime::now();
        let unow: DateTime<Utc> = now.into();
        let ns_ts = now.duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let ts_str = format!("n{}", ns_ts);

        let dt = ts_from_str(ts_str).unwrap();
        assert_eq!(dt, unow);
    }

    #[test]
    fn test_error_nothing() {
        let error = ts_from_str("xxxx".to_string()).unwrap_err();
        assert_eq!(error, ProcessError::Nothing);
    }

    #[test]
    fn test_error_not_an_integer() {
        let error = ts_from_str("mxxxx".to_string()).unwrap_err();
        assert_eq!(error, ProcessError::NotAnInt);
    }

    #[test]
    fn test_error_not_a_timestamp() {
        let error = ts_from_str("s100000000000000000".to_string()).unwrap_err();
        assert_eq!(error, ProcessError::NotATS);
    }
}
