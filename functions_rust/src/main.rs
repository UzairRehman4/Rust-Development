fn main() {
    println!("Hello, world!");
    // another_fn(23);
    // let x = (let y = 6);
    let res = fn_value(3, 5);
    println!("value of fn_value is {res}");
    let a = [5; 10];

    let mut sum = 0;

    for x in a {
        sum += x;
    }
    println!("{sum}");
    let mut x = 0;
    'a: loop {
        x += 1;

        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }

        break;
    }
}

fn fn_value(x: i32, y: i32) -> i32 {
    x + y
}
