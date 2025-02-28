use aes_gcm::{AeadCore};
use chacha20poly1305::{
    ChaCha20Poly1305, Key, aead::{AeadInPlace, KeyInit, OsRng}
};
use crate::encryption::key; // 암호화 키 생성 함수 가져오기

/// 🔒 **ChaCha20으로 데이터를 암호화하는 함수**
pub fn chacha_encrypt(data: &mut Vec<u8>, key_str: &str) -> Result<(), String> {
    
    let key_bytes = key::drive_key(key_str).map_err(|e| format!("키 생성 실패: {}", e))?;
    let key = Key::from_slice(&key_bytes);

    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    // ChaCha20 암호화 객체 생성
    let cipher = ChaCha20Poly1305::new(key);

    cipher.encrypt_in_place(&nonce, b"", data)
        .map_err(|e| format!("ChaCha20 암호화 실패: {:?}", e))?;
    // 데이터 암호화 수행
    
    data.splice(0..0,nonce.iter().cloned());
    Ok(())
}

/// 🔓 **ChaCha20으로 데이터를 복호화하는 함수**
pub fn chacha_decrypt(encrypted_data: &mut Vec<u8>, key_str: &str) -> Result<(), String> {

    if encrypted_data.len() < 12 {
        return Err("❌ 암호화된 데이터가 너무 짧습니다. 올바른 파일인지 확인하세요.".into());
    }

    let key_bytes = key::drive_key(key_str).map_err(|e| format!("키 생성 실패: {}", e))?;
    let key = Key::from_slice(&key_bytes);

    let nonce = chacha20poly1305::Nonce::clone_from_slice(&encrypted_data[..12]);

    let cipher = ChaCha20Poly1305::new(key);
    let mut ciphered_data = encrypted_data.split_off(12);

    cipher.decrypt_in_place(&nonce,b"", &mut ciphered_data)
        .map_err(|e| format!("ChaCha20 복호화 실패: {:?}", e))?;

    encrypted_data.clear();
    encrypted_data.extend(&ciphered_data);
    
    Ok(())
}