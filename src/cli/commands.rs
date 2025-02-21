// 📌 암호화/복호화 CLI 명령 처리
use crate::file_io::{reader, writer};  // 파일 읽기/쓰기 모듈
use crate::encryption::{aes, chacha};  // AES-256 및 ChaCha20 암호화 모듈
use std::error::Error;  // 오류 처리를 위한 표준 라이브러리

/// 🔒 **파일 암호화 실행 (AES-256 또는 ChaCha20)**
///
/// * `file`: 암호화할 파일 경로 (`&str`)  
/// * `key`: 암호화 키 (`&str`)  
/// * `algo`: 사용할 암호화 알고리즘 (`"aes"` 또는 `"chacha"`)  
/// * **반환값**: 암호화 성공 여부 (`Result<(), Box<dyn Error>>`)
pub fn encrypt_file(file: &str, key: &str, algo: &str) -> Result<(), Box<dyn Error>> {
    // 1️⃣ **파일 읽기**
    let data = reader::read_file(file)?;

    // 2️⃣ **알고리즘 선택 및 암호화 수행**
    let encrypted_data = match algo {
        "aes" => aes::aes_encrypt(&data, key),      // AES-256 암호화
        "chacha" => chacha::chacha_encrypt(&data, key), // ChaCha20 암호화
        _ => Err(format!("지원되지 않는 암호화 알고리즘: {}", algo)), // 잘못된 알고리즘 입력 처리
    }?;

    // 3️⃣ **암호화된 데이터를 새로운 파일로 저장 (`file.enc`)**
    writer::write_file(&format!("{}.enc", file), &encrypted_data)?;

    // 4️⃣ **성공 메시지 출력**
    println!("✅ {} 알고리즘으로 파일 암호화 완료! {} → {}.enc", algo, file, file);
    Ok(())
}

/// 🔓 **파일 복호화 실행 (AES-256 또는 ChaCha20)**
///
/// * `file`: 복호화할 파일 경로 (`&str`)  
/// * `key`: 복호화 키 (`&str`)  
/// * `algo`: 사용할 복호화 알고리즘 (`"aes"` 또는 `"chacha"`)  
/// * **반환값**: 복호화 성공 여부 (`Result<(), Box<dyn Error>>`)
pub fn decrypt_file(file: &str, key: &str, algo: &str) -> Result<(), Box<dyn Error>> {
    // 1️⃣ **암호화된 파일 읽기**
    let data = reader::read_file(file)?;

    // 2️⃣ **알고리즘 선택 및 복호화 수행**
    let decrypted_data = match algo {
        "aes" => aes::aes_decrypt(&data, key),      // AES-256 복호화
        "chacha" => chacha::chacha_decrypt(&data, key), // ChaCha20 복호화
        _ => Err(format!("지원되지 않는 복호화 알고리즘: {}", algo)), // 잘못된 알고리즘 입력 처리
    }?;

    // 3️⃣ **출력 파일 이름 설정 (`file.enc` → 원본 파일명)**
    let output_filename = file.strip_suffix(".enc").unwrap_or(file);

    // 4️⃣ **복호화된 데이터를 원본 파일로 저장**
    writer::write_file(output_filename, &decrypted_data)?;

    // 5️⃣ **성공 메시지 출력**
    println!("🔓 {} 알고리즘으로 파일 복호화 완료! {} → {}", algo, file, output_filename);
    Ok(())
}