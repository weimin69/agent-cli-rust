pub fn execute(words: &[String], upper: bool) {
    let output = words.join(" ");
    if upper {
        println!("{}", output.to_uppercase());
    } else {
        println!("{}", output);
    }
}
