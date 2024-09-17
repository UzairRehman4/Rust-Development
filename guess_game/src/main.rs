use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please Enter Your Number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter Valid Number");
                continue;
            }
        }; //shadowing

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your Number is Low"),
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Your Number is high"),
        }
    }
}
