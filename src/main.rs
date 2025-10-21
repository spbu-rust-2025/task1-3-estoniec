use std::fs::File;
use std::io;
use std::path::Path;
use std::io::Read;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim();

    if !Path::new(path).exists() {
        println!("failure");
        return;
    }

    match File::open(path) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            match file.read_to_end(&mut buffer) {
                Ok(_) => println!("success"),
                Err(_) => println!("failure"),
            }
        }
        Err(_) => println!("failure"),
    }
}
