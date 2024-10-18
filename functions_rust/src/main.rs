fn main() {
    println!("Hello, world!");
    another_fn(23);
    // let x = (let y = 6);
}

fn another_fn(x: isize) {
    println!("another function value of x is {x:?}");
}
