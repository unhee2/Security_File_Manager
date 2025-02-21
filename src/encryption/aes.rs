// AES256 으로 암호화 수행 (aes_gcm 라이브러리 사용)
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};
use std::error::Error;

pub fn aes_encrypt(data :&[u8], key_str: &str)-> Result<Vec<u8>,Box<dyn Error>>{
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);

    let ciphered_data = cipher.encrypt(&nonce,data.as_bytes())?;

    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extended_from_slice(&ciphered_data);
    Ok(encrypted_data)
}

pub fn aes_decrypt(encrypted_data: &[u8], key_str:&str)->Result<Vec<u8>,Box<dyn Error>>{
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());

    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);

    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher.decrypt(nonce, ciphered_data.as_bytes())?;
    
    Ok(plaintext)
}