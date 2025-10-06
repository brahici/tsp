use std::panic;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use chrono_tz::Tz;

use crate::dump::{get_fn, DumpOutcomeFn};

const JSON_FLAGS: &[&str] = &["-j", "--json"];
const DEFAULT_DATE_FORMAT: &str = "%a, %d %b %Y %H:%M:%S %z";
const DEFAULT_TZ: &str = "UTC";

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
            "tsp expects at least one value to process".into(),
        )),
        _ => Ok(args.split_off(1)),
    }
}

pub fn get_dump_fn(cli_args: &mut Vec<String>) -> DumpOutcomeFn {
    let mut json = false;
    let original_len = cli_args.len();

    // check if json output is required
    cli_args.retain(|x| !JSON_FLAGS.contains(&x.as_str()));
    if original_len > cli_args.len() {
        json = true;
    }
    get_fn(json)
}

pub fn get_fmt_str(cli_args: &mut Vec<String>) -> String {
    validate_fmt(extract_fmt(cli_args))
}

fn extract_fmt(cli_args: &mut Vec<String>) -> String {
    let mut cli_args_iter = cli_args.clone().into_iter();
    match cli_args_iter.position(|x| x == "-F") {
        None => DEFAULT_DATE_FORMAT.to_string(),
        Some(idx) => {
            cli_args.remove(idx);
            if let Some(fmt) = cli_args_iter.next() {
                cli_args.remove(idx);
                return fmt.to_owned();
            }
            DEFAULT_DATE_FORMAT.to_string()
        }
    }
}

fn validate_fmt(fmt: String) -> String {
    let dt_now: DateTime<Utc> = SystemTime::now().into();
    let fmt_ts = dt_now.format(fmt.as_str());
    let result = panic::catch_unwind(|| format!("{fmt_ts}"));
    match result {
        Ok(_) => fmt,
        Err(_) => {
            eprintln!("! invalid format string: '{fmt}', using default.");
            DEFAULT_DATE_FORMAT.to_string()
        }
    }
}

pub fn get_tz(cli_args: &mut Vec<String>) -> chrono_tz::Tz {
    validate_tz(extract_tz(cli_args))
}

fn extract_tz(cli_args: &mut Vec<String>) -> String {
    let mut cli_args_iter = cli_args.clone().into_iter();
    match cli_args_iter.position(|x| x == "-T") {
        None => DEFAULT_TZ.to_string(),
        Some(idx) => {
            cli_args.remove(idx);
            if let Some(tz) = cli_args_iter.next() {
                cli_args.remove(idx);
                return tz.to_owned();
            }
            DEFAULT_TZ.to_string()
        }
    }
}

fn validate_tz(tz_str: String) -> Tz {
    match tz_str.parse::<Tz>() {
        Ok(tz) => tz,
        Err(_) => DEFAULT_TZ.parse::<Tz>().unwrap(),
    }
}

pub fn cleanup(cli_args: &mut Vec<String>) -> bool {
    let original_len = cli_args.len();
    let mut cleaned = false;
    cli_args.retain(|x| !x.starts_with("-"));
    if original_len > cli_args.len() {
        cleaned = true;
    }
    cleaned
}

#[cfg(test)]
mod test {
    use chrono_tz::UTC;

    use crate::args::ArgsError;
    use crate::args::DEFAULT_DATE_FORMAT;
    use crate::args::{cleanup, get_dump_fn};
    use crate::args::{get_fmt_str, get_ts_strings, validate_fmt};
    use crate::args::{get_tz, validate_tz};

    #[test]
    fn test_no_args() {
        let no_args: Vec<String> = vec!["tsp".to_string()];
        let error = get_ts_strings(no_args).unwrap_err();
        assert_eq!(
            error,
            ArgsError::NotEnough("tsp expects at least one value to process".into())
        );
    }

    #[test]
    fn test_some_args() {
        let some_args: Vec<String> =
            vec!["tsp".to_string(), "argA".to_string(), "argB".to_string()];
        let remaining = get_ts_strings(some_args).unwrap();
        assert_eq!(remaining, vec!["argA".to_string(), "argB".to_string()]);
    }

    #[test]
    fn test_get_dump_fn_text() {
        // can't test the value returned by `get_dump_fn`, as it will be a private function
        // we can test other effect of the function on the arguments passed to it.
        let mut some_args: Vec<String> = vec!["tsp".to_string(), "argA".to_string()];
        let _ = get_dump_fn(&mut some_args);
        assert_eq!(some_args, vec!["tsp".to_string(), "argA".to_string()]);
    }

    #[test]
    fn test_get_dump_fn_json() {
        // same as above
        let mut some_args: Vec<String> =
            vec!["tsp".to_string(), "--json".to_string(), "argA".to_string()];
        let _ = get_dump_fn(&mut some_args);
        assert_eq!(some_args, vec!["tsp".to_string(), "argA".to_string()]);
    }

    #[test]
    fn test_get_dump_fn_json_short() {
        // same as above
        let mut some_args: Vec<String> =
            vec!["tsp".to_string(), "-j".to_string(), "argA".to_string()];
        let _ = get_dump_fn(&mut some_args);
        assert_eq!(some_args, vec!["tsp".to_string(), "argA".to_string()]);
    }

    #[test]
    fn test_get_fmt_str() {
        let mut some_args: Vec<String> = vec![
            "tsp".to_string(),
            "-F".to_string(),
            "a-format".to_string(),
            "argA".to_string(),
        ];
        let fmt_str = get_fmt_str(&mut some_args);
        assert_eq!(some_args, vec!["tsp".to_string(), "argA".to_string()]);
        assert_eq!(fmt_str, "a-format".to_string());
    }

    #[test]
    fn test_get_fmt_str_default() {
        let mut some_args: Vec<String> =
            vec!["tsp".to_string(), "argA".to_string(), "-F".to_string()];
        let fmt_str = get_fmt_str(&mut some_args);
        assert_eq!(fmt_str, DEFAULT_DATE_FORMAT.to_string());
        assert_eq!(some_args, vec!["tsp".to_string(), "argA".to_string(),]);
    }

    #[test]
    fn test_validate_fmt_ok() {
        let fmt = "%Y%m%d".to_string();
        let got = validate_fmt(fmt.to_owned());
        assert_eq!(got, fmt);
    }

    #[test]
    fn test_validate_fmt_err() {
        let fmt = "%N".to_string();
        let got = validate_fmt(fmt.to_owned());
        assert_eq!(got, DEFAULT_DATE_FORMAT);
    }

    #[test]
    fn test_get_tz() {
        let mut some_args: Vec<String> = vec![
            "tsp".to_string(),
            "-T".to_string(),
            "Europe/Paris".to_string(),
            "argA".to_string(),
        ];
        let tz = get_tz(&mut some_args);
        assert_eq!(some_args, vec!["tsp".to_string(), "argA".to_string()]);
        assert_eq!(tz, chrono_tz::Europe::Paris);
    }

    #[test]
    fn test_get_fmt_tz_default() {
        let mut some_args: Vec<String> = vec!["tsp".to_string(), "argA".to_string()];
        let tz = get_tz(&mut some_args);
        assert_eq!(tz, UTC);
        assert_eq!(some_args, vec!["tsp".to_string(), "argA".to_string(),]);
    }

    #[test]
    fn test_get_fmt_tz_missing_value() {
        let mut some_args: Vec<String> =
            vec!["tsp".to_string(), "argA".to_string(), "-T".to_string()];
        let tz = get_tz(&mut some_args);
        assert_eq!(tz, UTC);
        assert_eq!(some_args, vec!["tsp".to_string(), "argA".to_string(),]);
    }

    #[test]
    fn test_validate_tz_ok() {
        let tz_str = "Europe/Paris".to_string();
        let tz = validate_tz(tz_str.to_owned());
        assert_eq!(tz, chrono_tz::Europe::Paris);
    }

    #[test]
    fn test_validate_tz_err() {
        let tz_str = "Atlantis/Atlantis_City".to_string();
        let tz = validate_tz(tz_str.to_owned());
        assert_eq!(tz, chrono_tz::UTC);
    }

    #[test]
    fn test_no_cleanup() {
        let mut some_args: Vec<String> =
            vec!["tsp".to_string(), "argA".to_string(), "argB".to_string()];
        let cleaned = cleanup(&mut some_args);
        assert_eq!(cleaned, false);
        assert_eq!(
            some_args,
            vec!["tsp".to_string(), "argA".to_string(), "argB".to_string()]
        );
    }

    #[test]
    fn test_some_cleanup() {
        let mut some_args: Vec<String> = vec![
            "tsp".to_string(),
            "argA".to_string(),
            "-rubbish".to_string(),
        ];
        let cleaned = cleanup(&mut some_args);
        assert_eq!(cleaned, true);
        assert_eq!(some_args, vec!["tsp".to_string(), "argA".to_string()]);
    }
}
