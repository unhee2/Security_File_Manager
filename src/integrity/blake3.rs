// blake3 해쉬 함수를 사용하기 위한 모듈
use blake3;
use std::fs;
use std::error::Error;

pub fn blake3_hash(data : &[u8])->[u8; 32]{
    blake3::hash(data).into()
}

pub fn blake3_file_hash(file_path: &str)->Result<[u8; 32],Box<dyn Error>>{
    let data = fs::read(file_path)?;
    Ok(blake3_hash(&data))
}