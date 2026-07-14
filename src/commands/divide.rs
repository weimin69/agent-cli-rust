pub fn execute(dividend: i32, divisor: i32) {
    if divisor == 0 {
        println!("Error: divisor cannot be zero");
    } else {
        println!("{}", dividend / divisor);
    }
}
