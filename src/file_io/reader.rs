use std::fs::File;
use std::error::Error;
use std::io::{BufReader,Read};

pub fn read_file(filename: &str)->Result<Vec<u8>,Box<dyn Error>>{
   
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file); // Buffer를 사용하여 속도 개선
    let mut buffer = Vec::new();

    buf_reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}   