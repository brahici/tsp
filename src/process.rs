use crate::args::get_ts_strings;
use crate::dump::DumpOutcomeFn;
use crate::outcome::Outcome;
use crate::value::ts_from_str;

pub fn go(cli_args: Vec<String>, dump_fn: DumpOutcomeFn) {
    let mut outcomes: Vec<Outcome> = Vec::new();

    match get_ts_strings(cli_args) {
        Ok(ts_strs) => {
            for ts_str in ts_strs.iter() {
                let mut outcome = Outcome::new(ts_str.to_string());
                match ts_from_str(ts_str.to_string()) {
                    Ok(dt) => {
                        outcome.set(dt.format("%a, %d %b %Y %H:%M:%S %z").to_string());
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

#[cfg(test)]
mod test {
    use crate::outcome::Outcome;
    use crate::process::go;

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
}
