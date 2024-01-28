fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x=6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    // let mut spaces = "      ";
    // spaces = spaces.len();
    // println!("The value of spaces.len() is: {spaces.len()}");

    let tup : (i32, f32, u8)= (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is : {y}");

    let plus = plus_one(5);
    println!("The value of plus is : {plus}");
}

fn plus_one(x: i32) -> i32 {
    x+1
}