use anyhow::{Context, Result};
use dotenv::dotenv;
use std::{env, str::FromStr};

use myapp::apply_transformations;

fn main() -> Result<()> {
    dotenv().ok();

    let (threshold, max_iterations) = get_environment_settings()?;

    let input = get_input_numbers()?;
    if input.is_empty() {
        println!("Usage: please provide a list of numbers separated by spaces.");
        return Ok(());
    }

    let result = apply_transformations(input, max_iterations, threshold);

    println!("Results: {:?}", result);
    Ok(())
}

fn get_input_numbers() -> Result<Vec<u64>> {
    env::args()
        .skip(1)
        .map(|arg| {
            u64::from_str(&arg).with_context(|| format!("Error: '{}' is not a valid number.", arg))
        })
        .collect()
}

fn get_environment_settings() -> Result<(usize, usize)> {
    let threshold: usize = env::var("THRESHOLD")
        .unwrap_or("3".to_string())
        .parse()
        .context("Invalid threshold value")?;

    let max_iterations: usize = env::var("MAX_ITERATIONS")
        .unwrap_or("8".to_string())
        .parse()
        .context("Invalid max_iterations value")?;

    Ok((threshold, max_iterations))
}
