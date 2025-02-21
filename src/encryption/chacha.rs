use aes_gcm::AeadCore;
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce, aead::{Aead, KeyInit, OsRng}
};


/// 🔒 **ChaCha20으로 데이터를 암호화하는 함수**
pub fn chacha_encrypt(data: &[u8], key_str: &str) -> Result<Vec<u8>, String> {
    let key = Key::from_slice(key_str.as_bytes());

    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    // ChaCha20 암호화 객체 생성
    let cipher = ChaCha20Poly1305::new(key);

    // 데이터 암호화 수행
    let ciphertext = cipher
        .encrypt(&nonce, data.as_ref())
        .map_err(|e| format!("ChaCha20 암호화 실패: {:?}", e))?;

    // Nonce(IV) + 암호문을 하나의 데이터로 합쳐 반환
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);

    Ok(encrypted_data)
}

/// 🔓 **ChaCha20으로 데이터를 복호화하는 함수**
pub fn chacha_decrypt(encrypted_data: &[u8], key_str: &str) -> Result<Vec<u8>, String> {
    let key = Key::from_slice(key_str.as_bytes());

    // Nonce(IV)와 암호문 분리
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    // ChaCha20 암호화 객체 생성
    let cipher = ChaCha20Poly1305::new(key);

    // 데이터 복호화 수행
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("ChaCha20 복호화 실패: {:?}", e))?;

    Ok(plaintext)
}