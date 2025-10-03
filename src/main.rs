use std::env;

mod args;
mod dump;
mod outcome;
mod parse;
mod process;
mod unit;
mod usage;
mod value;
use args::{cleanup, get_dump_fn, get_fmt_str};

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    do_it(cli_args.to_owned());
}

fn do_it(mut cli_args: Vec<String>) {
    if usage::is_help(cli_args.to_owned()) {
        usage::print_usage();
    } else {
        let dump_fn = get_dump_fn(&mut cli_args);
        let fmt_str = get_fmt_str(&mut cli_args);
        if cleanup(&mut cli_args) {
            eprintln!("! cleaned rubbish parameters");
        }
        process::go(cli_args.to_owned(), fmt_str, dump_fn);
    }
}

#[cfg(test)]
mod test_dummy {
    #[test]
    fn test_default() {
        crate::main()
    }

    #[test]
    fn test_do_it_usage() {
        crate::do_it(vec!["tsp".to_string(), "-h".to_string()]);
    }

    #[test]
    fn test_warning() {
        let args: Vec<String> = vec![
            "tsp".to_string(),
            "-rubbish".to_string(),
            "argA".to_string(),
        ];
        crate::do_it(args);
    }
}
