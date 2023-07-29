use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", path);

    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
