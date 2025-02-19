fn main() {
    let mut text = String::from("Rust programming is fun");

    // Borrow `text` as an immutable reference
    let first = first_word(&text);
    println!("The first word is: {}", first);

    // Attempt to modify `text` after borrowing
    // Uncommenting the line below will cause a compiler error
    text.clear();

    // The string `text` remains usable
    println!("The original text is still: {}", text);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..] // Return the entire string if no space is found
}
