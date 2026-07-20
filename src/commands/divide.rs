use anyhow::{Result, bail};

pub fn execute(dividend: f64, divisor: f64) -> Result<()> {
    if divisor == 0.0 {
        bail!("cannot divide by zero");
    }
    println!("{}", dividend / divisor);
    Ok(())
}
