// SHA-256 함수
use sha2::{Sha256, Digest};
use std::fs;
use std::error::Error;

pub fn sha256_hash(data :&[u8])->[u8; 32]{ // 32bytes 해쉬값 반환 함수
    let mut hasher = Sha256::new(); // sha256 해쉬 함수 생성
    hasher.update(data); // 해쉬값 계산
    hasher.finalize().into() // 32bytes 크기의 배열로 변환
}

pub fn sha256_file_hash(file_path: &str)->Result<[u8; 32],Box<dyn Error>>{ // 파일을 해쉬하는 함수
    let data = fs::read(file_path)?; // 파일을 읽어서 data에 저장
    Ok(sha256_hash(&data))
}
