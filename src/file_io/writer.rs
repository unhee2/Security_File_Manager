use std::fs;
use std::error::Error;

pub fn write_file(filename: &str, data:&[u8])->Result<(),Box<dyn Error>>{
    fs::write(filename,data);
    Ok(())
}