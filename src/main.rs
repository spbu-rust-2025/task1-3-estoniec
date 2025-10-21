use std::io;
use std::fs;
use std::path::Path;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let path = input.trim();

    if !Path::new(path).exists() {
        println!("failure");
        return;
    }

    match fs::read_to_string(path) {
        Ok(_) => println!("success"),
        Err(e) => {
            if e.kind() == io::ErrorKind::PermissionDenied {
                println!("success");
            } else {
                println!("failure");
            }
        }
    }
}
