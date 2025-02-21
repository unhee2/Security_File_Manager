use std::fs;
use std::error::Error;

pub fn read_file(filename: &str)->Result<Vec<u8>,Box<dyn Error>>{
    let file = fs::read(filename)?;
    Ok(file)
}   