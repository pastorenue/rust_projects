use std::fs;

fn main() {
    let contents = fs::read_to_string("hello.txt");
    println!("{}", contents.unwrap());
