use anyhow::{Context, Result};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305, XNonce,
};
use rand::RngCore;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

// File Format Constants
const MAGIC_HEADER: &[u8; 6] = b"SHADOW";
const NONCE_LEN: usize = 24; // XChaCha20 Nonce

/// Encrypts bytes using a password and writes to a file.
pub fn encrypt_data(data: &[u8], password: &str, output_path: &Path) -> Result<()> {
    // 1. Generate a random salt for Argon2
    let salt = SaltString::generate(&mut OsRng);
    
    // 2. Derive Key (32 bytes)
    let key = derive_key(password, &salt)?;

    // 3. Initialize Cipher
    let cipher = XChaCha20Poly1305::new(&key.into());

    // 4. Generate Random Nonce
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from_slice(&nonce_bytes);

    // 5. Encrypt
    let ciphertext = cipher
        .encrypt(nonce, Payload { msg: data, aad: &[] })
        .map_err(|_| anyhow::anyhow!("Encryption failed internally"))?;

    // 6. Write to File
    let mut file = File::create(output_path)?;
    file.write_all(MAGIC_HEADER)?;
    
    // Write Salt (Length-prefixed)
    let salt_bytes = salt.as_str().as_bytes();
    file.write_all(&(salt_bytes.len() as u32).to_le_bytes())?;
    file.write_all(salt_bytes)?;

    // Write Nonce
    file.write_all(&nonce_bytes)?;

    // Write Ciphertext
    file.write_all(&ciphertext)?;

    Ok(())
}

/// Reads a file, parses headers, derives key, and decrypts.
pub fn decrypt_data(input_path: &Path, password: &str) -> Result<Vec<u8>> {
    let mut file = File::open(input_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut cursor = 0;

    // 1. Check Magic
    if buffer.len() < 6 || &buffer[0..6] != MAGIC_HEADER {
        anyhow::bail!("Invalid file format. Not a .shadow file.");
    }
    cursor += 6;

    // 2. Read Salt
    if buffer.len() < cursor + 4 { anyhow::bail!("File truncated (salt len)"); }
    let salt_len = u32::from_le_bytes(buffer[cursor..cursor+4].try_into()?) as usize;
    cursor += 4;

    if buffer.len() < cursor + salt_len { anyhow::bail!("File truncated (salt)"); }
    let salt_str = std::str::from_utf8(&buffer[cursor..cursor+salt_len])
        .context("Invalid salt encoding")?;
    
    // FIX: Use from_b64 instead of new
    let salt = SaltString::from_b64(salt_str).map_err(|e| anyhow::anyhow!(e))?;
    cursor += salt_len;

    // 3. Derive Key
    let key = derive_key(password, &salt)?;

    // 4. Read Nonce
    if buffer.len() < cursor + NONCE_LEN { anyhow::bail!("File truncated (nonce)"); }
    let nonce = XNonce::from_slice(&buffer[cursor..cursor+NONCE_LEN]);
    cursor += NONCE_LEN;

    // 5. Decrypt
    let ciphertext = &buffer[cursor..];
    let cipher = XChaCha20Poly1305::new(&key.into());

    let plaintext = cipher
        .decrypt(nonce, Payload { msg: ciphertext, aad: &[] })
        .map_err(|_| anyhow::anyhow!("Decryption failed. MAC mismatch or wrong password."))?;

    Ok(plaintext)
}

fn derive_key(password: &str, salt: &SaltString) -> Result<[u8; 32]> {
    let mut output_key_material = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut output_key_material)
        .map_err(|e| anyhow::anyhow!(e))?;
    
    Ok(output_key_material)
}
