use crate::unit::Unit;

use std::str::FromStr;

#[derive(Debug)]
pub struct InputParseError;

#[derive(Debug, PartialEq)]
pub struct ParseResult {
    pub unit: Unit,
    pub ts: String,
}
impl FromStr for ParseResult {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prefix = &s.to_string()[0..1];
        match prefix {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => Ok(ParseResult {
                unit: Unit::Secs,
                ts: s.into(),
            }),
            _ => {
                if let Ok(unit) = Unit::from_str(prefix) {
                    Ok(ParseResult {
                        unit,
                        ts: s.to_string()[1..].into(),
                    })
                } else {
                    Err(InputParseError)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parse::ParseResult;
    use crate::unit::Unit;
    use std::str::FromStr;

    #[test]
    fn test_from_str_no_prefix() {
        let s = "1000";
        let res = ParseResult::from_str(s).unwrap();

        assert_eq!(res.unit, Unit::Secs);
        assert_eq!(res.ts, "1000".to_string());
    }

    #[test]
    fn test_from_str_prefix_seconds() {
        let s = "s1000";
        let res = ParseResult::from_str(s).unwrap();

        assert_eq!(
            res,
            ParseResult {
                unit: Unit::Secs,
                ts: "1000".to_string(),
            }
        );
    }
}
