use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem passing arguments: {err}");
    //     process::exit(1);
    // });
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match minigrep::run(config) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    };
}
