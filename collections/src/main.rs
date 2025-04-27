fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v_infer = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    print!("Vector v: '{:?}", v)
}
