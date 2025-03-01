use aes_gcm::{AeadCore};
use chacha20poly1305::{
    ChaCha20Poly1305, Key, aead::{AeadInPlace, KeyInit, OsRng}
};
use crate::encryption::key; // 암호화 키 생성 함수 가져오기

/// 🔒 **ChaCha20으로 데이터를 암호화하는 함수**
///
/// # Arguments
/// * `data` - 암호화할 데이터 (Vec<u8>)
/// * `key_str` - 사용자 입력 키 문자열
///
/// # Returns
/// * `Ok(())` - 성공적으로 암호화됨
/// * `Err(String)` - 암호화 실패 시 오류 메시지 반환
pub fn chacha_encrypt(data: &mut Vec<u8>, key_str: &str) -> Result<(), String> {
    let key_bytes = key::drive_key(key_str).map_err(|e| format!("키 생성 실패: {}", e))?;
    let key = Key::from_slice(&key_bytes);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let cipher = ChaCha20Poly1305::new(key);

    // 데이터를 ChaCha20 방식으로 암호화
    cipher.encrypt_in_place(&nonce, b"", data)
        .map_err(|e| format!("ChaCha20 암호화 실패: {:?}", e))?;
    
    // Nonce 값을 암호화 데이터 앞에 삽입
    data.splice(0..0, nonce.iter().cloned());
    Ok(())
}

/// 🔓 **ChaCha20으로 데이터를 복호화하는 함수**
///
/// # Arguments
/// * `encrypted_data` - 암호화된 데이터 (Vec<u8>)
/// * `key_str` - 사용자 입력 키 문자열
///
/// # Returns
/// * `Ok(())` - 성공적으로 복호화됨
/// * `Err(String)` - 복호화 실패 시 오류 메시지 반환
pub fn chacha_decrypt(encrypted_data: &mut Vec<u8>, key_str: &str) -> Result<(), String> {
    if encrypted_data.len() < 12 {
        return Err("❌ 암호화된 데이터가 너무 짧습니다. 올바른 파일인지 확인하세요.".into());
    }

    let key_bytes = key::drive_key(key_str).map_err(|e| format!("키 생성 실패: {}", e))?;
    let key = Key::from_slice(&key_bytes);
    let nonce = chacha20poly1305::Nonce::clone_from_slice(&encrypted_data[..12]);

    let cipher = ChaCha20Poly1305::new(key);
    let mut ciphered_data = encrypted_data.split_off(12);

    // 데이터를 ChaCha20 방식으로 복호화
    cipher.decrypt_in_place(&nonce, b"", &mut ciphered_data)
        .map_err(|e| format!("ChaCha20 복호화 실패: {:?}", e))?;

    encrypted_data.clear();
    encrypted_data.extend(&ciphered_data);
    
    Ok(())
}
