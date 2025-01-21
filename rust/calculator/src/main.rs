use std::env;

#[no_mangle]
pub extern "C" fn sub_two_numbers(a: i32, b: i32) -> i32 {
    return a - b
}

#[no_mangle]
pub extern "C" fn add_two_numbers(a: i32, b: i32) -> i32 {
    let c = a + b;
    println!("{} + {} = {}", a, b, c);
    return c;
}

fn main() {    
    let args = env::args().collect::<Vec<String>>();
    let mut a = 5;
    let mut b = 8;
    match args[1].to_string().parse::<i32>() {
        Ok(int_val) => a = int_val,
        Err(_error) => println!("NaN: {} using 5", args[1]),
    };

    match args[2].to_string().parse::<i32>() {
        Ok(int_val) => b = int_val,
        Err(_error) => println!("NaN: {} using 8", args[2]),
    };

    let _c = add_two_numbers(a, b);
}
