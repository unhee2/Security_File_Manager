// 📌 암호화/복호화 CLI 명령 처리
use crate::file_io::{reader, writer};  // 파일 읽기/쓰기 모듈
use crate::encryption::{aes, chacha};  // AES-256 및 ChaCha20 암호화 모듈
use crate::integrity::{blake3, sha256}; // 블레이크3 및 SHA-256 해시 모듈
use std::error::Error;  // 오류 처리를 위한 표준 라이브러리

/// 🔒 **파일 암호화 실행 (AES-256 또는 ChaCha20)**
///
/// * `file`: 암호화할 파일 경로 (`&str`)  
/// * `key`: 암호화 키 (`&str`)  
/// * `algo`: 사용할 암호화 알고리즘 (`"aes"` 또는 `"chacha"`)  
/// * `hash_algo`: 사용할 해쉬 알고리즘 (`"sha"` 또는 `"blake"`)
/// * **반환값**: 암호화 성공 여부 (`Result<(), Box<dyn Error>>`)
pub fn encrypt_file(file: &str, key: &str, algo: &str, hash_algo: &str) -> Result<(), Box<dyn Error>> {
    // **파일 읽기**
    let data = reader::read_file(file)?;

    // **알고리즘 선택 및 암호화 수행**
    let encrypted_data = match algo {
        "aes" => aes::aes_encrypt(&data, key),      // AES-256 암호화
        "chacha" => chacha::chacha_encrypt(&data, key), // ChaCha20 암호화
        _ => Err(format!("지원되지 않는 암호화 알고리즘: {}", algo)), // 잘못된 알고리즘 입력 처리
    }?;

    // 파일의 해쉬값 계산 
    let file_hash = match hash_algo{
        "sha" => sha256::sha256_file_hash(&file), // SHA-256 해쉬값 계산
        "blake" => blake3::blake3_file_hash(&file), // Blake3 해쉬값 계산
        _ => Err("지원되지 않는 해쉬 알고리즘".into()), // 잘못된 알고리즘 입력 처리
    }?;

    // 파일 해쉬값과 암호화된 데이터를 합쳐서 새로운 데이터 생성
    let mut final_data = file_hash.to_vec();
    final_data.extend_from_slice(&encrypted_data);

    // 암호화된 데이터를 새로운 파일로 저장 (`file.enc`)**
    writer::write_file(&format!("{}.enc", file), &final_data)?;

    // 성공 메시지 출력**
    println!("✅ {} 알고리즘으로 파일 암호화 완료! {} → {}.enc", algo, file, file);
    Ok(())
}

/// 🔓 **파일 복호화 실행 (AES-256 또는 ChaCha20)**
///
/// * `file`: 복호화할 파일 경로 (`&str`)  
/// * `key`: 복호화 키 (`&str`)  
/// * `algo`: 사용할 복호화 알고리즘 (`"aes"` 또는 `"chacha"`)  
/// * `hash_algo`: 사용할 해쉬 알고리즘 (`"sha"` 또는 `"blake"`)
/// * **반환값**: 복호화 성공 여부 (`Result<(), Box<dyn Error>>`)
pub fn decrypt_file(file: &str, key: &str, algo: &str, hash_algo: &str) -> Result<(), Box<dyn Error>> {
    // **암호화된 파일 읽기**
    let data = reader::read_file(file)?;

    // 해쉬값과 데이터 분리
    let (stored_hash_bytes, encrypted_data) = data.split_at(32);

    // **알고리즘 선택 및 복호화 수행**
    let decrypted_data = match algo {
        "aes" => aes::aes_decrypt(&encrypted_data, key),      // AES-256 복호화
        "chacha" => chacha::chacha_decrypt(&encrypted_data, key), // ChaCha20 복호화
        _ => Err(format!("지원되지 않는 복호화 알고리즘: {}", algo)), // 잘못된 알고리즘 입력 처리
    }?;

    // 무결성 검증 
    let calculate_hash = match hash_algo {
        "sha" => sha256::sha256_hash(&decrypted_data), // SHA-256 해쉬값 계산
        "blake" => blake3::blake3_hash(&decrypted_data), // Blake3 해쉬값 계산
        _ => return Err("지원되지 않는 해쉬 알고리즘".into()), // 잘못된 알고리즘 입력 처리
    };
    if stored_hash_bytes == calculate_hash {
        println!("✅ 무결성 검증 성공! 원본 파일과 동일합니다.");
    } else {
        println!("❌ 경고: 파일이 변조되었습니다!");
        return Err("무결성 검증 실패".into());
    }

    // **출력 파일 이름 설정 (`file.enc` → 원본 파일명)**
    let output_filename = file.strip_suffix(".enc").unwrap_or(file);

    // **복호화된 데이터를 원본 파일로 저장**
    writer::write_file(output_filename, &decrypted_data)?;

    // **성공 메시지 출력**
    println!("🔓 {} 알고리즘으로 파일 복호화 완료! {} → {}", algo, file, output_filename);
    Ok(())
}