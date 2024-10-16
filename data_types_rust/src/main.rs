fn main() {
    let s = 0b1010;
    let a = 254_u8;
    let b = 254_isize;
    println!("value of s = {s} a = {a} b = {b}");

    // i8 range  = -128 to 127
    // u8 range  = 0 to 255

    let value: u8 = random_number() + 100;
    println!("Value is {value}");

    fn random_number() -> u8 {
        200
    }
}
