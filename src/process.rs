use chrono::{DateTime, Utc};
use chrono_tz::Tz;

use crate::args::get_ts_strings;
use crate::dump::DumpOutcomeFn;
use crate::outcome::Outcome;
use crate::value::ts_from_str;

pub fn go(cli_args: Vec<String>, fmt: String, tz: Tz, dump_fn: DumpOutcomeFn) {
    let mut outcomes: Vec<Outcome> = Vec::new();

    match get_ts_strings(cli_args) {
        Ok(ts_strs) => {
            for ts_str in ts_strs.iter() {
                let mut outcome = Outcome::new(ts_str.to_string());
                match ts_from_str(ts_str.to_string()) {
                    Ok(dt) => {
                        let fmt_ts = FmtDate::new(dt, fmt.to_owned(), tz);
                        outcome.set(format!("{fmt_ts}"));
                    }
                    Err(err) => {
                        outcome.set(format!("{err}"));
                    }
                }
                outcomes.push(outcome);
            }
            dump_fn(outcomes);
        }
        Err(err) => {
            println!("{err}");
        }
    }
}

#[derive(Debug)]
pub struct FmtDate {
    dt: DateTime<Utc>,
    fmt: String,
    tz: Tz,
}
impl std::fmt::Display for FmtDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt = self.dt.with_timezone(&self.tz);
        write!(f, "{}", dt.format(self.fmt.as_str()))
    }
}
impl FmtDate {
    pub fn new(dt: DateTime<Utc>, fmt: String, tz: Tz) -> FmtDate {
        FmtDate { dt, fmt, tz }
    }
}

#[cfg(test)]
mod test {
    use crate::outcome::Outcome;
    use crate::process::go;
    use crate::process::FmtDate;

    use chrono::{DateTime, Utc};
    use chrono_tz::UTC;
    use std::sync::Mutex;

    #[test]
    fn test_go() {
        static COLLECTED_OUTCOMES: Mutex<Vec<Outcome>> = Mutex::new(Vec::new());

        fn test_dump_fn(outcomes: Vec<Outcome>) {
            let mut collected = COLLECTED_OUTCOMES.lock().unwrap();
            collected.extend(outcomes);
        }

        go(
            vec!["tsp".to_string(), "1337".to_string(), "errful".to_string()],
            "%a, %d %b %Y %H:%M:%S %z".to_string(),
            UTC,
            test_dump_fn,
        );

        let collected = COLLECTED_OUTCOMES.lock().unwrap().to_owned();
        assert_eq!(
            collected,
            vec![
                Outcome::new("1337".to_string()).set("Thu, 01 Jan 1970 00:22:17 +0000".to_string()),
                Outcome::new("errful".to_string()).set("can't interpret the value".to_string()),
            ]
        );
    }

    #[test]
    fn test_fmtdate_implementation() {
        let dt: DateTime<Utc> =
            DateTime::parse_from_str("2025-09-23 20:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .into();
        let fmt_date = FmtDate::new(dt, "%Y-%m-%d".to_string(), UTC);
        assert_eq!("2025-09-23", format!("{}", fmt_date));
    }

    #[test]
    fn test_fmtdate_implementation_with_tz() {
        let dt: DateTime<Utc> =
            DateTime::parse_from_str("2025-09-23 23:30:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .into();
        let fmt_date = FmtDate::new(dt, "%Y-%m-%d".to_string(), chrono_tz::Europe::Paris);
        assert_eq!("2025-09-24", format!("{}", fmt_date));
    }
}
