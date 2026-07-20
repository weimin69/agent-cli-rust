use anyhow::{Result, bail};

pub fn execute(words: &[String], times: u32) -> Result<()> {
    if times == 0 {
        bail!("times must be greater than 0");
    }

    for _ in 0..times {
        println!("{}", words.join(" "));
    }
    Ok(())
}
