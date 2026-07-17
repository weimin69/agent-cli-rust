pub fn execute(words: &[String], times: u32) -> Result<(), String> {
    if times == 0 {
        return Err("times must be greater than 0".to_string());
    }

    for _ in 0..times {
        println!("{}", words.join(" "));
    }
    Ok(())
}
