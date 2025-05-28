#[derive(Debug)]
enum List {
    Coins(i32, Box<List>),
    Nil,
}
fn main() {}
