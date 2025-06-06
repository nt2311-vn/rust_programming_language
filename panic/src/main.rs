use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:#?}", error);
            })
        } else {
            panic!("Problem opening the file: {:#?}", error);
        }
    });
}
