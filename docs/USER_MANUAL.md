# 📘 Security File Manager 사용자 매뉴얼

## 1. 개요
Security File Manager는 AES-256 및 ChaCha20 암호화를 지원하는 파일 암호화 도구입니다. 이 매뉴얼은 도구의 설치, 사용법 및 주요 기능을 설명합니다.

## 2. 설치 방법

### 2.1 바이너리 다운로드
1. [GitHub Releases](https://github.com/unhee2/Security_File_Manager/releases)에서 최신 버전을 다운로드합니다.
2. 운영체제에 맞는 바이너리를 실행 가능한 위치에 배치합니다.

### 2.2 Cargo로 설치 (Rust 환경 필요)
```sh
cargo install security-file-manager
```

### 2.3 소스 코드 빌드
```sh
git clone https://github.com/unhee2/Security_File_Manager.git
cd Security_File_Manager
cargo build --release
```

## 3. 사용법

### 3.1 파일 암호화
```sh
security-file-manager enc -f input.txt -k mysecretkey -a aes
```
- `enc` 또는 `encrypt` 명령어 사용 가능
- `-f` / `--file`: 암호화할 파일 지정
- `-k` / `--key`: 암호화 키 입력
- `-a` / `--algo`: 암호화 알고리즘 선택 (`aes` 또는 `chacha`)

### 3.2 파일 복호화
```sh
security-file-manager dec -f input.txt.enc -k mysecretkey -a aes
```
- `dec` 또는 `decrypt` 명령어 사용 가능

### 3.3 명령어 목록
```sh
security-file-manager --help
```

## 4. 옵션 설명
| 옵션                | 설명                                   |
|-----------------|--------------------------------------|
| `enc` / `encrypt` | 파일을 암호화합니다.                        |
| `dec` / `decrypt` | 파일을 복호화합니다.                        |
| `-f` / `--file`   | 암호화 또는 복호화할 파일 지정               |
| `-k` / `--key`    | 암호화 키 지정                            |
| `-a` / `--algo`   | 암호화 알고리즘 선택 (`aes`, `chacha`) |
| `-h` / `--help`   | 도움말 출력                              |
| `-V` / `--version`| 버전 정보 출력                           |

## 5. 예제

### 5.1 AES 알고리즘을 사용한 암호화
```sh
security-file-manager enc -f secret.txt -k strongpassword -a aes
```

### 5.2 암호화된 파일 복호화
```sh
security-file-manager dec -f secret.txt.enc -k strongpassword -a aes
```

## 6. 문제 해결

### 6.1 `Invalid key length` 오류 발생
- 사용한 키 길이가 32바이트(AES) 또는 256비트(ChaCha20)가 맞는지 확인하세요.

### 6.2 `File not found` 오류 발생
- 입력한 파일 경로가 올바른지 확인하세요.

## 7. 라이선스
본 프로젝트는 MIT 라이선스를 따릅니다.
