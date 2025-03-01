# 🔐 Security File Manager 보안 문서

## 1. 암호화 개요
Security File Manager는 AES-256 및 ChaCha20 알고리즘을 사용하여 파일을 안전하게 보호합니다.

## 2. 암호화 알고리즘
- **AES-256 (Advanced Encryption Standard)**
  - 대칭 키 암호화 방식
  - 256비트 키를 사용하여 보안성이 높음
  - GCM 모드 지원 (무결성 검증 포함)

- **ChaCha20**
  - 스트림 암호 방식
  - 고속 연산 및 우수한 보안성 제공
  - Poly1305를 통한 인증 지원

## 3. 보안 고려 사항
1. **키 보안**
   - 암호화 키는 안전한 환경에서 관리해야 합니다.
   - 키 파일을 별도로 저장하거나 환경 변수로 설정하는 것이 추천됩니다.

2. **무결성 검증**
   - 파일 암호화 시 HMAC 또는 GCM을 사용하여 변조 감지 가능

3. **파일 삭제 및 데이터 복구**
   - 암호화된 파일을 삭제할 때 복구가 불가능하도록 안전한 삭제 방법을 사용하세요.

## 4. 키 관리 방법
- 안전한 저장소 (예: HashiCorp Vault, AWS KMS) 활용 권장

## 5. 추가 보안 조치
- 암호화 키 노출 방지를 위해 환경 변수 활용 가능
```sh
export FILE_ENC_KEY="my_secure_key"
security-file-manager enc -f input.txt -k $FILE_ENC_KEY
```
