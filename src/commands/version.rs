pub fn execute() -> Result<(), String> {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
