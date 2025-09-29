use std::env;

mod args;
mod parse;
mod process;
mod unit;

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    match args::get_ts_strings(cli_args) {
        Ok(ts_strs) => {
            for ts_str in ts_strs.iter() {
                print!("{ts_str:20} :: ");
                match process::ts_from_str(ts_str.to_string()) {
                    Ok(dt) => {
                        println!("{}", dt.format("%a, %d %b %Y %H:%M:%S %z"));
                    }
                    Err(err) => {
                        println! {"{err}"};
                    }
                }
            }
        }
        Err(err) => {
            println!("{err}");
        }
    }
}
