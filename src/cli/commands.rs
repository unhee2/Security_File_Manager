// 암호화/복호화 CLI 명령 처리
use crate::file_io::{reader,writer};
use crate::encryption::aes;
use std::error::Error;

pub fn encrypt_data(data:&[u8],_key: &str)->Vec<u8>{
    data.t
}
pub fn decrypt_data(data:&[u8],_key: &str)->Vec<u8>{
    data.to_vec()
}
pub fn encrypt_file(file: &str, key: &str)->Result<(),Box<dyn Error>>{
    let data = reader::read_file(file)?;
    let encrypted_data = encrypt_data(&data, key);
    writer::write_file(&format!("{}.enc",file), &encrypted_data)?;
    println!("파일 암호화 완료! {} -> {}.enc",file,file);
    Ok(())
}
pub fn decrypt_file(file: &str,key:&str)->Result<(), Box<dyn Error>>{
    let data = reader::read_file(file)?;
    let decrypted_data = decrypt_data(&data, key);
    let output_filename = file.strip_suffix(".enc").unwrap_or(file);
    writer::write_file(output_filename, &decrypted_data)?;
    println!("파일 복호화 완료! {}.enc -> {}",file,file);
    Ok(())
}