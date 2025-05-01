use std::{
    fs::File,
    io::{self, Read},
};

fn main() {}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn last_chart_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
