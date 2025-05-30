use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

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

    for i in &v {
        println!("{i}");
    }

    let mut m_vec = vec![100, 32, 57];

    for i in &mut m_vec {
        *i += 50;
    }

    println!(
        "Iterate and mutate value of a mutable vector: '{:?}'",
        m_vec
    );

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("String: {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{s3}");

    let x1 = String::from("tic");
    let x2 = String::from("tac");
    let x3 = String::from("toe");

    let x = format!("{x1}-{x2}-{x3}");
    println!("{x}");
    println!("{x1}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 30);

    println!("Scores map: '{:#?}'", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Blue score is {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name.clone(), field_value.clone());

    println!("{field_name} {field_value}");

    let mut new_scores = HashMap::new();
    new_scores.insert(String::from("Blue"), 10);

    new_scores.entry(String::from("Yellow")).or_insert(50);
    new_scores.entry(String::from("Blue")).or_insert(50);

    println!("'{:#?}'", new_scores);

    let text = "hello world wonderful world";
    let mut count_words = HashMap::new();

    for word in text.split_whitespace() {
        let count = count_words.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Count word result: '{:#?}'", count_words);
}
