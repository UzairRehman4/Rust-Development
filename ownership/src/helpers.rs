// #[allow(dead_code)]  this is attribute to remove warnings of compiler

pub mod namehelpers {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let name = format!("{0} {1}", first, last);
        name
    }
}

pub fn test_if() {
    let age_to_drive = 16u8;

    println!("Enter the person's age: ");
    let myinput = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();

    let age = myinput.replace("\n", "").parse::<u8>().unwrap();

    if age >= age_to_drive {
        println!(" You can't drive");
    }
}
