const GLOBAL_NUMBER: u32 = 100;
const ARRAY_VALUES: [i32; 4] = [1, 2, 3, 45];

fn main() {
    let mut _values = 10;
    _values = 20;

    let avalue = 20;
    let avalue = true;
    // println!("Hello, world!");

    // const PI: u32 = 10;
    // println!("Pi Value {}", PI);

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3 + PI + GLOBAL_NUMBER;
    // println!("Three Hours In Seconds {}", THREE_HOURS_IN_SECONDS);
    println!("ARRAY value are here :{:?}", ARRAY_VALUES);
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!(
            "The value of x in the inner scope is: {x} , and global number is {GLOBAL_NUMBER}"
        );
    }

    let _apple = 10;

    let _apple = 20;

    println!("apples {}", _apple);

    let _daredevil = "Mathew";
    let _daredevil = "Born again";

    let _daredevil = _daredevil;
    // _daredevil = "20";

    println!("daredevil {}", _daredevil);

    test_func();
}

fn test_func() {
    let x = ();
    println!("{:?}", x);
}
