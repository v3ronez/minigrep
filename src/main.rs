use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    // let args: Vec<String> = env::args().collect(); //another way to type collect
    let query = &args[1];
    let path = &args[2];
    println!("Searching for {query}");
    println!("In file {path}");
}
