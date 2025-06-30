use std::{error::Error, fs, vec};

pub struct Config {
    pub query: String,
    pub path_file: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Self {
            query: args[1].clone(),
            path_file: args[2].clone(),
        })
    }
}

// know that Box<dyn Error> means the function will return a type
// that implements the Error trait, but we don’t have to specify
// what particular type the return value will be. This gives us flexibility
// to return error values that may be of different types in different error cases.
//
// The *dyn* keyword is short for dynamic.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content =
        fs::read_to_string(config.path_file).expect("Should have been able to read the file");
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
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
