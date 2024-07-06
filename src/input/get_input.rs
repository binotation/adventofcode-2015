use std::error::Error;
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn get_input(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(read_to_string(format!("src/input/{}.txt", input))?)
}
