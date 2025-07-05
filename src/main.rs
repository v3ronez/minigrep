use std::{char::CharTryFromError, collections::HashMap, env, process, thread, time::Duration};

use minigrep::{Config, run};

struct Cacher<T>
where
    T: Fn(&str) -> String,
{
    calculation: T,
    value: HashMap<String, String>,
}

impl<T> Cacher<T>
where
    T: Fn(&str) -> String,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &str) -> String {
        self.value
            .entry(arg.to_string())
            .or_insert_with(|| (self.calculation)(arg))
            .clone()
        // match self.value.get(arg) {
        //     Some(v) => v.clone(),
        //     None => {
        //         let result = (self.calculation)(arg);
        //         self.value.insert(arg.to_string(), result.clone());
        //         result
        //     }
        // }
    }
}

fn main() {
    let mut expensive_closure = Cacher::new(|s| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        s.to_string()
    });

    let k = &String::from("casa");
    let a = &String::from("casa2");
    println!("{}", expensive_closure.value(k));
    println!("calculate again: {}", expensive_closure.value(k));

    println!("{}", expensive_closure.value(a));
    println!("calculate again: {}", expensive_closure.value(a));
}

// fn main() {
//     let args = env::args().collect::<Vec<String>>();
//     // let args: Vec<String> = env::args().collect(); //another way to type collect
//
//     // one way to error handling
//     // let config = match Config::build(&args) {
//     //     Ok(config) => config,
//     //     Err(err) => return println!("{}", err.to_string()),
//     // };
//     //
//     let config = Config::build(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });
//
//     println!("Searching for {}\n", config.query);
//     println!("In file {}\n", config.path_file);
//
//     if let Err(e) = run(config) {
//         eprintln!("Application error: {e}");
//         process::exit(1);
//     }
// }
