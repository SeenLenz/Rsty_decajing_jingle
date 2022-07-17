use std::fs;
use std::error::Error;

pub fn json_to_string(path: &str) -> Result<String, Box<dyn Error>>{
    
    Ok(fs::read_to_string(path)?)

}
