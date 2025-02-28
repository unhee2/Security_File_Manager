use std::error::Error;

pub fn drive_key(user_key: &str) -> Result<[u8; 32], Box<dyn Error>> {
    let mut key = [0u8; 32];

    let user_key_bytes = user_key.as_bytes();
    let key_len = user_key_bytes.len();

    if key_len > 32 {
        return Err("❌ 키 길이가 32바이트를 초과했습니다. 키를 32바이트 이하로 입력하세요.".into());
    }

    key[..key_len].copy_from_slice(user_key_bytes);
    Ok(key)
}