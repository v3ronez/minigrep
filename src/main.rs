#[allow(dead_code)]
#[allow(unused_imports)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::{env, process, rc::Rc};

use minigrep::{Config, run};

#[allow(dead_code)]
struct Cacher<T>
where
    T: Fn(&str) -> String,
{
    calculation: T,
    value: HashMap<String, String>,
}

#[allow(dead_code)]
// concrate types dont have to add <T> after impl
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

// implementing Iterator for Counter
//
// struct Counter {
//     count: u32,
// }
//
// impl Counter {
//     fn new() -> Counter {
//         Counter { count: 0 }
//     }
// }
// impl Iterator for Counter {
//     type Item = u32;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.count += 1;
//         if self.count < 6 {
//             return Some(self.count);
//         }
//         None
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn calling_next_directly() {
//         let mut counter = Counter::new();
//         assert_eq!(counter.next(), Some(1));
//         assert_eq!(counter.next(), Some(2));
//         assert_eq!(counter.next(), Some(3));
//         assert_eq!(counter.next(), Some(4));
//         assert_eq!(counter.next(), Some(5));
//         assert_eq!(counter.next(), None);
//     }
// }

// fn main() {
//     let mut expensive_closure = Cacher::new(|s| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         s.to_string()
//     });
//
//     let k = &String::from("casa");
//     let a = &String::from("casa2");
//     println!("{}", expensive_closure.value(k));
//     println!("calculate again: {}", expensive_closure.value(k));
//
//     println!("{}", expensive_closure.value(a));
//     println!("calculate again: {}", expensive_closure.value(a));
// }
//
// fn main() {
//     // let args = env::args().collect::<Vec<String>>();
//     // let args: Vec<String> = env::args().collect(); //another way to type collect
//
//     // one way to error handling
//     // let config = match Config::build(&args) {
//     //     Ok(config) => config,
//     //     Err(err) => return println!("{}", err.to_string()),
//     // };
//     //
//     let config = Config::build(env::args()).unwrap_or_else(|err| {
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

// enum List {
//     // Cons(i32, Box<List>), //Box has a default size value "usize" so that break the loop;
//     Cons(i32, Rc<List>), //RC<T> allowed to use multi reference on the same time;
//     Nil,
// }
// use List::{Cons, Nil};
//
// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Cons(15, Rc::new(Nil)))))));
//     // let b = Cons(3, Box::new(Nil));
//     let b = Cons(3, Rc::clone(&a));
//     let c = Cons(4, Rc::clone(&a));
//     // let c = Cons(4, Box::new(Nil));
// }
//
//

pub trait Messager {
    fn send(&self, msg: &str);
}

// T must implement the Messager trait.
// ensures that any references inside T live at least as long as 'a.
pub struct LimitTracker<'a, T: 'a + Messager> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: 'a + Messager,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;
        if percentage >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of you quota!");
            return;
        }

        if percentage >= 0.9 && percentage < 1.0 {
            self.messenger
                .send("Warning: You've used up over 90% of you quota!");
            return;
        }
        if percentage >= 1.0 {
            self.messenger.send("ERROR: You are over your quota!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MockMessenger {
        // sent_messages: Vec<String>;
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        pub fn new() -> Self {
            Self {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    // the `fn send(&self, msg: &str)` on Message trait uses an immutable &self but in the mock I want
    // `fn send()` to use `&mut self` so I can store all messages that were sent. To solved it use
    // RefCell<T>
    impl Messager for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_message = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_message.sent_messages.borrow().len(), 1);
    }
}

fn main() {}
