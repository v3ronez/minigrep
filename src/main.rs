use std::{env, error::Error, fs, process};

struct Config {
    query: String,
    path_file: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Self {
            query: args[1].clone(),
            path_file: args[2].clone(),
        })
    }
}

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
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.path_file);

    run(config);
}

// know that Box<dyn Error> means the function will return a type
// that implements the Error trait, but we don’t have to specify
// what particular type the return value will be. This gives us flexibility
// to return error values that may be of different types in different error cases.
// The dyn keyword is short for dynamic.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.path_file)?;
    println!("\n{content}");
    Ok(())
}

// fn parse_config(args: &Vec<String>) -> Config {
//     // Config::new(args[0].clone, args[1].clone())
//
//     Config {
//         query: args[1].clone(),
//         path_file: args[2].clone(),
//     }
//
//     //Or  Config {
//     //     query
//     //     path_file
//     // }
// }
