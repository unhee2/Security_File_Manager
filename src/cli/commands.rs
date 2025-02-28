// 📌 암호화/복호화 CLI 명령 처리
use indicatif::{ProgressBar, ProgressStyle};  // 진행률 표시 모듈
use crate::file_io::{reader, writer};  // 파일 읽기/쓰기 모듈
use crate::encryption::{aes, chacha};  // AES-256 및 ChaCha20 암호화 모듈
use std::error::Error;

/// 🔒 **파일 암호화 실행 (AES-256 또는 ChaCha20)**
pub fn encrypt_file(file: &str, key: &str, algo: &str) -> Result<(), Box<dyn Error>> {
    let mut data = reader::read_file(file)?;

    // 📌 진행 바 생성 (파일 크기를 기준으로 설정)
    let pb = ProgressBar::new(data.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("🔐 암호화 진행 중: [{bar:40.cyan/blue}] {pos}/{len} bytes ({eta})")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    match algo {
        "aes" => aes::aes_encrypt(&mut data, key),
        "chacha" => chacha::chacha_encrypt(&mut data, key),
        _ => return Err("❌ 지원되지 않는 암호화 알고리즘입니다. (AES 또는 ChaCha만 사용 가능)".into()),
    }?;

    for _ in 0..data.len() {
        pb.inc(1); // 암호화 진행 상황 업데이트
    }

    pb.finish_with_message("✅ 암호화 완료!");

    writer::write_file(&format!("{}.enc", file), &data)?;
    println!("✅ {} 알고리즘으로 파일 암호화 완료: {} → {}", algo, file, file);

    Ok(())
}

/// 🔓 **파일 복호화 실행 (AES-256 또는 ChaCha20)**
pub fn decrypt_file(file: &str, key: &str, algo: &str) -> Result<(), Box<dyn Error>> {
    let mut data = reader::read_file(file)?;

    if data.len() < 12 {
        return Err("❌ 파일 크기가 너무 작아 복호화할 수 없습니다. 올바른 암호화된 파일인지 확인하세요.".into());
    }

    // 📌 진행 바 생성 (파일 크기를 기준으로 설정)
    let pb = ProgressBar::new(data.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("🔐 복호화 진행 중: [{bar:40.cyan/blue}] {pos}/{len} bytes ({eta})")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    match algo {
        "aes" => aes::aes_decrypt(&mut data, key),
        "chacha" => chacha::chacha_decrypt(&mut data, key),
        _ => return Err("❌ 지원되지 않는 복호화 알고리즘입니다. (AES 또는 ChaCha만 사용 가능)".into()),
    }?;

    for _ in 0..data.len() {
        pb.inc(1); // 암호화 진행 상황 업데이트
    }

    pb.finish_with_message("✅ 복호화 완료!");

    let output_file = file.replace(".enc", "");
    writer::write_file(&output_file, &data)?;

    println!("✅ {} 알고리즘으로 파일 복호화 완료: {} → {}", algo, file, output_file);
    Ok(())
}