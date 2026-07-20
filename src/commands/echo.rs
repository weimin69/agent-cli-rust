use anyhow::Result;

pub fn execute(words: &[String], upper: bool) -> Result<()> {
    let output = words.join(" ");
    if upper {
        println!("{}", output.to_uppercase());
    } else {
        println!("{}", output);
    }
    Ok(())
}
