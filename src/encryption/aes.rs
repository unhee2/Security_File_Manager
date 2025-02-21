// 📌 AES-256 GCM 암호화 및 복호화 수행 (aes_gcm 라이브러리 사용)
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng}, // AES-GCM을 위한 주요 트레이트
    Aes256Gcm, Nonce, Key // AES-256-GCM 및 관련 구조체 사용
};

/// 🔒 **AES-256 GCM을 이용하여 데이터를 암호화하는 함수**
/// 
/// * `data`: 암호화할 바이트 배열 (`&[u8]`)
/// * `key_str`: 암호화 키 (문자열, 최소 32바이트 권장)
/// * **반환값**: 암호화된 데이터 (`Result<Vec<u8>, String>`)
pub fn aes_encrypt(data: &[u8], key_str: &str) -> Result<Vec<u8>, String> {
    // 1️⃣ **AES-256 키 생성 (32바이트)**
    // `key_str`을 바이트 배열로 변환 후, AES-256 키 객체 생성
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());

    // 2️⃣ **96비트(12바이트) 랜덤 Nonce(IV) 생성**
    // 매번 암호화할 때 새로운 Nonce(IV)를 생성해야 보안이 보장됨
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // 3️⃣ **AES-256-GCM 암호화 객체 생성**
    let cipher = Aes256Gcm::new(key);

    // 4️⃣ **데이터 암호화 수행**
    //    - `nonce`와 `data`를 AES-GCM 알고리즘으로 암호화
    //    - 오류 발생 시 에러 메시지 반환
    let ciphered_data = cipher
        .encrypt(&nonce, data.as_ref())
        .map_err(|e| format!("암호화 실패: {:?}", e))?;

    // 5️⃣ **Nonce(IV) + 암호문을 하나의 데이터로 합쳐 반환**
    let mut encrypted_data: Vec<u8> = nonce.to_vec(); // Nonce를 먼저 저장
    encrypted_data.extend_from_slice(&ciphered_data); // Nonce 뒤에 암호문 추가

    Ok(encrypted_data) // 🔐 최종 암호화된 데이터 반환
}

/// 🔓 **AES-256 GCM을 이용하여 데이터를 복호화하는 함수**
///
/// * `encrypted_data`: 암호화된 데이터 (`&[u8]`)
/// * `key_str`: 복호화 키 (문자열)
/// * **반환값**: 복호화된 데이터 (`Result<Vec<u8>, String>`)
pub fn aes_decrypt(encrypted_data: &[u8], key_str: &str) -> Result<Vec<u8>, String> {
    // 1️⃣ **AES-256 키 생성 (32바이트)**
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());

    // 2️⃣ **Nonce(IV)와 암호문 분리**
    // 암호화할 때 `Nonce + 암호문` 순서로 저장했으므로, 이를 분리
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12); // 앞 12바이트는 Nonce
    let nonce = Nonce::from_slice(nonce_arr); // Nonce 객체 생성

    // 3️⃣ **AES-256-GCM 암호화 객체 생성**
    let cipher = Aes256Gcm::new(key);

    // 4️⃣ **데이터 복호화 수행**
    //    - `nonce`와 `암호문`을 사용하여 원본 데이터를 복호화
    //    - 오류 발생 시 에러 메시지 반환
    let plaintext = cipher
        .decrypt(nonce, ciphered_data.as_ref())
        .map_err(|e| format!("복호화 실패: {:?}", e))?;

    Ok(plaintext) // 🔓 최종 복호화된 데이터 반환
}