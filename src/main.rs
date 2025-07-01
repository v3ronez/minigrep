use std::{env, process};

use minigrep::{Config, run};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    // let args: Vec<String> = env::args().collect(); //another way to type collect

    // one way to error handling
    // let config = match Config::build(&args) {
    //     Ok(config) => config,
    //     Err(err) => return println!("{}", err.to_string()),
    // };
    //
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}\n", config.query);
    println!("In file {}\n", config.path_file);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
