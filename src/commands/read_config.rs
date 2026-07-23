use anyhow::{Context, Result};
use serde::Deserialize;
pub fn execute(path: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path))?;

    let config: Config = toml::from_str(&content).context("failed to parse config file")?;

    println!("model: {}", config.model);
    println!("temperature: {}", config.temperature);

    Ok(())
}
#[derive(Debug, Deserialize)]
struct Config {
    model: String,
    temperature: f64,
}
