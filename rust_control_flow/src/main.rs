fn main() {
    let mut num: u8 = 1;

    let _result = 'my_loop: loop {
        println!("value of number is {}", num);

        if num == 10 {
            break 70;
        }

        // Only compute and print `new_loop` once
        if num == 2 {
            let new_loop = loop {
                if num == 10 {
                    break 'my_loop 43;
                }
                break 90;
            };
            println!("new loop value is {new_loop}");
        }

        num += 1;
    };
    println!("the result is {_result}");

    let mut num1 = 3;

    while num1 != 0 {
        println!("{num1}");

        num1 -= 1;
    }
    println!("ending the while loop");
}
