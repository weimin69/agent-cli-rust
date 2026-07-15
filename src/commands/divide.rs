pub fn execute(dividend: f64, divisor: f64) -> Result<(), String> {
    if divisor == 0.0 {
        return Err("cannot divide by zero".to_string());
    }
    println!("{}", dividend / divisor);
    Ok(())
}
