use std::{thread, time::Duration};
fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_closure(5);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut mutable_list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", mutable_list);

    let mut borrows_mutably = || mutable_list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", mutable_list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
