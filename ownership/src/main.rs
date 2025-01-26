pub mod helpers;

fn main() {
    let s = String::from("Uzair");
    let (s, len) = calculate_len(s);
    println!("The length of '{s}' is {len}");

    let (first, last) = ("Uzair", "Rehman");
    let result = helpers::namehelpers::get_full_name(first, last);
    println!("Full name: {result}");

    test_if();
}

fn calculate_len(s: String) -> (String, usize) {
    let result = s.len();
    (s, result)
}

fn test_if() {
    let age_to_drive = 16u8;

    println!("Enter the person's age: ");
    let mut myinput = String::new();
    std::io::stdin().read_line(&mut myinput).unwrap();

    let age = myinput.trim().parse::<u8>().unwrap_or(0);

    if age >= age_to_drive {
        println!("You can drive!");
    } else {
        println!("Wait for a license.");
    }
}
