fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v_infer = vec![1, 2, 3, 4, 5];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector v: '{:?}", v);

    let third: &i32 = &v_infer[2];

    println!("value of third is {}", *third);

    let third_get: Option<&i32> = v_infer.get(2);

    match third_get {
        Some(third) => println!("Value of third get is {}", *third),
        None => println!("There is no third element"),
    }
}
