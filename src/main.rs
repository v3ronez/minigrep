use std::{char::CharTryFromError, env, process, thread, time::Duration};

use minigrep::{Config, run};

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    println!("{}", expensive_closure.value(2));
    println!("calculate again {}", expensive_closure.value(2));
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
