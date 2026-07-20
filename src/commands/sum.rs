use anyhow::{Result, bail};

pub fn execute(numbers: &[f64]) -> Result<()> {
    if numbers.is_empty() {
        bail!("please input at least one number");
    }

    let mut sum: f64 = 0.0;
    for num in numbers {
        sum += *num;
    }
    println!("sum: {}", sum);
    Ok(())
}
