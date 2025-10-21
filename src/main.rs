use std::fs;
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim();

    match fs::read_to_string(path) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}
