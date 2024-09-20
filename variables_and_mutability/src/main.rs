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
    main1();
    main2();
    main3();
}
// Example 1: Basic Shadowing
fn main1() {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    let x = x * 2;
    println!("The value of x is: {}", x);
}

// Example 2: Shadowing with Different Types
fn main2() {
    let spaces = "   ";
    println!("Spaces: '{}'", spaces);

    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);
}

// Example 3: Shadowing in Different Scopes
fn main3() {
    let x = 10;
    println!("x in outer scope: {}", x);

    {
        let x = "inner";
        println!("x in inner scope: {}", x);
    }

    println!("x in outer scope: {}", x);
}
