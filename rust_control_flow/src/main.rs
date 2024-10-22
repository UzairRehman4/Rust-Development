// fn main() {
// let y = is_even(4);
// println!("value of y is {y:?}");

// if y {
//     println!("Fn returned true");
// }
// let calling = is_even(6);
// println!("calling fun value is {calling:?}");

// looping

// let mut num = 1;

// loop {
//     println!("looping bro {num:?}");

//     if num == 10 {
//         break;
//     }

//     num = num + num;
// }
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// fn is_even(x: i32) -> bool {
//     if x % 2 == 0 {
//         return true;
//     } else {
//         false
//     }
// }

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }   
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
