use std::fs::File;
use std::io::ErrorKind;

pub fn error() {
    File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("not found");
            panic!("not found: {}", error);
        } else {
            println!("found");
            panic!("found")
        }
    });
}
