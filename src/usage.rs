pub fn print_usage() {
    let version = env!("PACKAGE_VERSION");
    let chrono = env!("CHRONO_VERSION");
    println!(
        "\x1b[1mtsp\x1b[22m [{version}]

tsp is a timestamp parser.

\x1b[1mUSAGE\x1b[22m: tsp [OPTIONS] [timestamps]...

\x1b[1mOPTIONS\x1b[22m:
  -F <FORMAT>       Date format string
  -j, --json        JSON output
  -h, --help        Print help

  FORMAT supports https://docs.rs/chrono/{chrono}/chrono/format/strftime/index.html.

\x1b[1mARGS\x1b[22m:
  <timestamps>...        timestamps to convert


Timestamps granularity is either in seconds, milliseconds, microseconds or nanoseconds.
The parser supports a value prefix for each granularity level:
  * s  =>  seconds        (default)
  * m  =>  milliseconds
  * u  =>  microseconds
  * n  =>  nanoseconds
If a value has no prefix, it is parsed as seconds."
    );
}

pub fn is_help(args: Vec<String>) -> bool {
    for v in ["-h", "--help"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    {
        if args.iter().any(|s| s == &v) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::usage::is_help;

    #[test]
    fn test_is_help_true_short() {
        let some_args: Vec<String> = vec!["tsp", "1758642010", "--help"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let h = is_help(some_args);
        assert_eq!(h, true);
    }

    #[test]
    fn test_is_help_true_long() {
        let some_args = vec!["tsp", "1758642010", "-h"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let h = is_help(some_args);
        assert_eq!(h, true);
    }

    #[test]
    fn test_is_help_false() {
        let some_args = vec!["tsp", "1758642010"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let h = is_help(some_args);
        assert_eq!(h, false);
    }
}

#[cfg(test)]
mod test_dummy {
    #[test]
    fn test_usage() {
        crate::usage::print_usage();
    }
}
