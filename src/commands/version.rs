use anyhow::Result;

pub fn execute() -> Result<()> {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
