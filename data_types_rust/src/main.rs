// fn main() {
//     let s = 0b1010;
//     let a = 254_u8;
//     let b = 254_isize;
//     println!("value of s = {s} a = {a} b = {b}");

//     // i8 range  = -128 to 127
//     // u8 range  = 0 to 255

//     // let value = match random_number().checked_add(50) {
//     //     Some(num) => num,
//     //     None => {
//     //         println!("cannot add");
//     //         return;
//     //     }
//     // };
//     // println!("Value is {value}");

//     let (a, b) = random_number().overflowing_add(57);
//     println!("vlaue of a is {a} b value is'nt overflowed {b}");

//     fn random_number() -> u8 {
//         200
//     }

//     let x: f32 = 5.2_f32 / 5.9_f32;
//     println!("x is {x}");

//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;

//     println!("sum = {sum}, difference = {difference}, product = {product}, quotient = {quotient}, truncated = {truncated}, remainder = {remainder}");

//     let c = 'z';
//     let name = "Uzair";
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
//     println!("{heart_eyed_cat}  {name} c = {c} z = {z} ");
// }

fn main() {
    //Compound Types
    //Tuples have a fixed length: once declared, they cannot grow or shrink in size

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("tup = {tup:?} x = {x:?} y = {y:?} z = {z:?}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("five_hundred = {five_hundred} six_point_four = {six_point_four} one = {one}");

    let tupl = ();
    println!("tuple = {tupl}");

    let a = [3; 5];
    println!("a = {a:?}");
}
