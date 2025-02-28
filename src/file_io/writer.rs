use std::fs::File;
use std::io::{BufWriter, Write};
use std::error::Error;

pub fn write_file(filename: &str, data:&[u8])->Result<(),Box<dyn Error>>{
    // fs::write(filename,data);
    // Ok(())
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file); // Buffer를 사용하여 속도 개선선

    writer.write_all(data)?;
    writer.flush()?;
    Ok(())
}