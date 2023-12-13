fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x=6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    let mut spaces = "      ";
    spaces = spaces.len();
    println!("The value of spaces.len() is: {spaces.len()}");
}
