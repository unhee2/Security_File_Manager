# 🔐 Security File Manager

## 📌 프로젝트 개요
Security File Manager는 AES-256 및 ChaCha20 암호화를 사용하여 파일을 안전하게 보호하는 CLI 도구입니다.

## 🚀 기능
- AES-256 및 ChaCha20을 이용한 파일 암호화/복호화
- 진행률 표시 (`indicatif` 활용)
- CLI 기반 간편한 사용
- 파일 무결성 검증 기능 포함
- 크로스플랫폼 지원 (Windows, macOS, Linux)

## 🔧 설치 방법

### 1. 바이너리 다운로드
GitHub 릴리스 페이지에서 최신 버전의 실행 파일을 다운로드하여 사용하세요.

### 2. Cargo로 설치 (Rust 환경 필요)

```sh
cargo install security-file-manager
```
### 3. 소스 코드 빌드
```sh
# 프로젝트 다운로드
git clone https://github.com/unhee2/Security_File_Manager.git
cd Security_File_Manager

# 실행 파일 빌드
cargo build --release
```

## 📖 사용법

### 1. 파일 암호화
```sh
./target/release/security-file-manager enc -f example.txt -k mysecretkey -a aes
```
### 2. 파일 복호화
```sh
./target/release/security-file-manager dec -f example.txt.enc -k mysecretkey -a aes
```
### 3. cargo로 실행
프로젝트 루트 디렉토리로 이동한 후, 다음 명령을 실행하세요.
```sh
cargo run -- enc(dec) -f example.txt -k mysecretkey -a aes
```
### 4. 사용 가능한 옵션
|옵션|설명|
|-----|-----|
|enc / encrypt|파일을 암호화합니다.|
|dec / decrypt|파일을 복호화합니다.|
|-f / --file|대상 파일을 지정합니다.|
|-k / --key|암호화 키를 지정합니다.|
|-a / --algo|사용할 알고리즘을 지정합니다.(aes,chacha)|

## ✅ 테스트 및 검증
테스트 코드를 생성하거나 프로젝트 파일에 존재하는 테스트 코드를 다음 명령어로 실행할 수 있습니다.
```sh
cargo test
```
## 📜 라이선스
본 프로젝트는 MIT 라이선스를 따릅니다.

## 🤝 기여
기여를 환영합니다! Issues 또는 Pull Requests를 통해 참여해주세요.
