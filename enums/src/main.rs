enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    value_in_certs(Coin::Penny);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in_certs(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
