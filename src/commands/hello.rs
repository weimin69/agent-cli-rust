pub fn execute(name: String, age: u32) -> Result<(), String> {
    println!("Hello {name}!");
    println!("Age: {age}");
    Ok(())
}
