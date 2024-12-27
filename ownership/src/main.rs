fn main() {
    // let mut s = String::from("Hello");

    // s.push_str(" Uzair");

    // println!("S = {s}");
    let mut x = 10;
    let y = x;

    x = 20;
    println!("x = {x}, y = {y}");

    let v = vec![1, 2, 3];

    let v2 = v;
    println!("v2", { v2 });
}
