fn main() {
    // let y = is_even(4);
    // println!("value of y is {y:?}");

    // if y {
    //     println!("Fn returned true");
    // }
    // let calling = is_even(6);
    // println!("calling fun value is {calling:?}");

    // looping

    let mut num = 1;

    loop {
        println!("looping bro {num:?}");

        if num == 10 {
            break;
        }

        num = num + num;
    }
}

// fn is_even(x: i32) -> bool {
//     if x % 2 == 0 {
//         return true;
//     } else {
//         false
//     }
// }
