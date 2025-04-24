fn main() {
    let my_string = String::from("hello world");

    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);

    let _word = first_word(&my_string);
    let my_string_literal = "hello world";

    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    let word = first_word(&my_string_literal);

    println!("The word is: {word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
