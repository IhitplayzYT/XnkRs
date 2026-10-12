pub mod Crypto{
    use aes_gcm::{aead::{Aead, AeadCore, KeyInit},Aes256Gcm, Nonce};
    use base64::{engine::general_purpose::STANDARD, Engine};
    use flate2::read::GzDecoder;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use rand::Rng;
    use serde::{Deserialize, Serialize};
    use sha2::{Digest, Sha256};
    use std::io::{Read, Write};


    #[derive(Clone)]
    pub struct EncryptionKey {
        key: [u8; 32],
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct EncryptedData {
        pub encrypted_data: String,
        pub nonce: String,
        pub compressed: bool,
    }


    impl EncryptionKey {
        pub fn new() -> Self {
            let mut key = [0u8; 32];
            rand::thread_rng().fill(&mut key);
            Self { key }
        }

        pub fn from_bytes(key: [u8; 32]) -> Self { Self { key } }

        pub fn as_bytes(&self) -> &[u8; 32] { &self.key }

        /// SHA256 on password+salt
        pub fn from_password(password: &str, salt: &[u8]) -> Self {
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            hasher.update(salt);
            let mut key = [0u8; 32];
            key.copy_from_slice(&hasher.finalize());
            Self { key }
        }

    }

    /// Encrypt using AES256 gcm
    pub fn encrypt(data: &[u8], key: &EncryptionKey) -> Result<(Vec<u8>, Vec<u8>), String> {
        let cipher = Aes256Gcm::new(key.as_bytes().into());
        let nonce = Aes256Gcm::generate_nonce(&mut rand::thread_rng());
        cipher.encrypt(&nonce, data).map(|encrypted| (encrypted, nonce.to_vec())).map_err(|e| format!("Encryption failed: {}", e))
    }

    /// Decrypt AES256 gcm
    pub fn decrypt(data: &[u8], nonce: &[u8], key: &EncryptionKey) -> Result<Vec<u8>, String> {
        let cipher = Aes256Gcm::new(key.as_bytes().into());
        let nonce = Nonce::from_slice(nonce);
        cipher.decrypt(nonce, data).map_err(|e| format!("Decryption failed: {}", e))
    }

    /// Compress data using gzip
    pub fn compress(data: &[u8]) -> Result<Vec<u8>, String> {
        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
        enc.write_all(data).map_err(|e| format!("Compression failed: {}", e))?;
        enc.finish().map_err(|e| format!("Compression failed: {}", e))
    }

    /// Uncompress data using gzip
    pub fn uncompress(data: &[u8]) -> Result<Vec<u8>, String> {
        let mut dec = GzDecoder::new(data);
        let mut buff = Vec::new();
        dec.read_to_end(&mut buff).map_err(|e| format!("Uncompression failed: {}", e))?;
        Ok(buff)
    }

    /// Compresses data then encrypts 
    pub fn encrypt_and_compress(data: &[u8], key: &EncryptionKey) -> Result<EncryptedData, String> {
        let (encryp, nonce) = encrypt(&compress(data)?, key)?;
        Ok(EncryptedData {encrypted_data: STANDARD.encode(&encryp),nonce: STANDARD.encode(&nonce),compressed: true})
    }

    /// Decryppts then uncompresses 
    pub fn decrypt_and_uncompress(data: &str,nonce: &str,key: &EncryptionKey) -> Result<Vec<u8>, String> {
        uncompress(&decrypt(& STANDARD.decode(data).map_err(|e| format!("Base64 decode failed: {}", e))?, &STANDARD.decode(nonce).map_err(|e| format!("Base64 decode nonce failed: {}", e))?, key)?)
    }

    /// SHA256 hash of data
    pub fn hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    /// Verify hash
    pub fn verify_hash(data: &[u8], hash_d: &str) -> bool { hash(data) == hash_d }

}
