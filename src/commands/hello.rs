use anyhow::Result;

pub fn execute(name: String, age: u32) -> Result<()> {
    println!("Hello {name}!");
    println!("Age: {age}");
    Ok(())
}
