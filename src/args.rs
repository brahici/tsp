#[derive(Debug, PartialEq)]
pub enum ArgsError {
    NotEnough(String),
}
impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsError::NotEnough(msg) => write!(f, "{msg}"),
        }
    }
}

pub fn get_ts_strings(mut args: Vec<String>) -> Result<Vec<String>, ArgsError> {
    let count = args.len() - 1;
    match count {
        0 => Err(ArgsError::NotEnough(
            "tsp expects at least one argument".into(),
        )),
        _ => Ok(args.split_off(1)),
    }
}

#[cfg(test)]
mod test {
    use crate::args::{ArgsError, get_ts_strings};

    #[test]
    fn test_no_args() {
        let no_args: Vec<String> = vec!["tsp".to_string()];
        let error = get_ts_strings(no_args).unwrap_err();
        assert_eq!(
            error,
            ArgsError::NotEnough("tsp expects at least one argument".into())
        );
    }

    #[test]
    fn test_some_args() {
        let some_args: Vec<String> =
            vec!["tsp".to_string(), "argA".to_string(), "argB".to_string()];
        let remaining = get_ts_strings(some_args).unwrap();
        assert_eq!(remaining, vec!["argA".to_string(), "argB".to_string()]);
    }
}
