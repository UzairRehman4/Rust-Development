const GLOBAL_NUMBER: u32 = 100;

fn main() {
    // println!("Hello, world!");

    // const PI: u32 = 10;
    // println!("Pi Value {}", PI);

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3 + PI + GLOBAL_NUMBER;
    // println!("Three Hours In Seconds {}", THREE_HOURS_IN_SECONDS);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!(
            "The value of x in the inner scope is: {x} , and global number is {GLOBAL_NUMBER}"
        );
    }

    println!("The value of x is: {x}");
}
