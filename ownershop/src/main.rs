fn main() {
    let s = String::from("RUST");
    let len = calculate_len(&s);
    println!("The len of {s} is {len}")
}

fn calculate_len(s: &String) -> usize {
    let result = s.len();
    result
}
