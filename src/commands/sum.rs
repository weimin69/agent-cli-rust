pub fn execute(numbers: &[f64]) -> Result<(), String> {
    if numbers.is_empty() {
        return Err("please input at least one number".to_string());
    }

    let mut sum: f64 = 0.0;
    for num in numbers {
        sum += *num;
    }
    println!("sum: {}", sum);
    Ok(())
}
