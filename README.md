# 🔐 File Encryption Tool

## 📌 프로젝트 개요
이 프로젝트는 AES-256 및 ChaCha20 암호화를 사용하여 파일을 안전하게 보호하는 CLI 도구입니다.

## 🚀 기능
- AES-256 및 ChaCha20을 이용한 파일 암호화/복호화
- 진행률 표시 (`indicatif` 활용)
- CLI 기반 간편한 사용

## 🔧 설치 방법

```sh
# 프로젝트 다운로드
git clone https://github.com/unhee2/Security_File_Manager.git
cd file-encryption-tool

# 실행 파일 빌드
cargo build --release
'''

## 📖 사용법
'''sh
# 파일 암호화
./target/release/file_enc_tool encrypt --file example.txt --key mysecretkey --algo aes

# 파일 복호화
./target/release/file_enc_tool decrypt --file example.txt.enc --key mysecretkey --algo aes
'''