pub fn generate_key() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut key = vec![0u8; 32];
    getrandom::getrandom(&mut key)?;
    Ok(key)
}

pub fn aes_gcm_encrypt(key: &[u8], plaintext: &[u8], nonce: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(plaintext.to_vec())
}

pub fn aes_gcm_decrypt(key: &[u8], ciphertext: &[u8], nonce: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(ciphertext.to_vec())
}
