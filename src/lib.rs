use std::{
    env::{self, Args},
    error::Error,
    fs, vec,
};

pub struct Config {
    pub query: String,
    pub path_file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: Args) -> Result<Self, &'static str> {
        args.next(); // the binary filename;
        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn`t get a query string"),
        };
        let path_file = match args.next() {
            Some(file_path) => file_path,
            None => return Err("Didn`t get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self {
            query: query,
            path_file: path_file,
            case_sensitive: case_sensitive,
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
    let contents =
        fs::read_to_string(config.path_file).expect("Should have been able to read the file");

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }
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
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = vec![];
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    fn case_insensitive() {
        let query = "ruSt";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        )
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
