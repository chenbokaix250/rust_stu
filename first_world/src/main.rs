fn main() {
    let s = String::from("Hello world");
    let wordindex = first_word(&s);
    println!("wordindex:{}",wordindex);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

