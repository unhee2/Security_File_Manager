use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng}, Aes256Gcm, Key
};
use aes_gcm::aead::generic_array::GenericArray;
use crate::encryption::key;

pub fn aes_encrypt(data: &mut Vec<u8>, key_str: &str) -> Result<(), String> {
    let key_bytes = key::drive_key(key_str).map_err(|e| format!("❌ 키 생성 실패: {}", e))?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);

    cipher.encrypt_in_place(&nonce, b"", data)
        .map_err(|e| format!("❌ AES 암호화 실패: {:?}", e))?;

    data.splice(0..0, nonce.iter().cloned());
    Ok(())
}

pub fn aes_decrypt(encrypted_data: &mut Vec<u8>, key_str: &str) -> Result<(), String> {
    if encrypted_data.len() < 12 {
        return Err("❌ 암호화된 데이터가 너무 짧습니다. 올바른 파일인지 확인하세요.".into());
    }

    let key_bytes = key::drive_key(key_str).map_err(|e| format!("❌ 키 생성 실패: {}", e))?;
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let nonce = GenericArray::clone_from_slice(&encrypted_data[..12]);

    let cipher = Aes256Gcm::new(key);
    let mut ciphered_data = encrypted_data.split_off(12);

    cipher.decrypt_in_place(&nonce, b"", &mut ciphered_data)
        .map_err(|e| format!("❌ AES 복호화 실패: {:?}", e))?;

    encrypted_data.clear();
    encrypted_data.extend(&ciphered_data);

    Ok(())
}