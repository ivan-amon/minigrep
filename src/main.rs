use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}