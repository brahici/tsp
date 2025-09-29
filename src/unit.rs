use chrono::DateTime;
use chrono::Utc;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct UnitParseError;

type ParseFn = fn(i64) -> Option<DateTime<Utc>>;

#[derive(Debug, PartialEq)]
pub enum Unit {
    Secs,
    Millis,
    Micros,
    Nanos,
}
impl FromStr for Unit {
    type Err = UnitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "s" | "" => Ok(Unit::Secs),
            "m" => Ok(Unit::Millis),
            "u" => Ok(Unit::Micros),
            "n" => Ok(Unit::Nanos),
            _ => Err(UnitParseError),
        }
    }
}
impl Unit {
    pub fn get_parser(&self) -> ParseFn {
        match self {
            Unit::Millis => DateTime::from_timestamp_millis,
            Unit::Micros => DateTime::from_timestamp_micros,
            Unit::Nanos => |n| Some(DateTime::from_timestamp_nanos(n)),
            _ => DateTime::from_timestamp_secs,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::unit::{Unit, UnitParseError};
    use std::str::FromStr;

    #[test]
    fn test_from_str_empty() {
        let u = Unit::from_str("").unwrap();
        assert_eq!(u, Unit::Secs);
    }

    #[test]
    fn test_from_str_seconds() {
        let u = Unit::from_str("s").unwrap();
        assert_eq!(u, Unit::Secs);
    }

    #[test]
    fn test_from_str_millis() {
        let u = Unit::from_str("m").unwrap();
        assert_eq!(u, Unit::Millis);
    }

    #[test]
    fn test_from_str_micros() {
        let u = Unit::from_str("u").unwrap();
        assert_eq!(u, Unit::Micros);
    }

    #[test]
    fn test_from_str_nanos() {
        let u = Unit::from_str("n").unwrap();
        assert_eq!(u, Unit::Nanos);
    }

    #[test]
    fn test_from_str_err() {
        let error = Unit::from_str("x").unwrap_err();
        assert_eq!(error, UnitParseError)
    }
}
