use std::fs;
use aes_gcm::aead::consts::{B1, B0};
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::generic_array::typenum::{UInt, UTerm};
use argon2::{Argon2, PasswordHasher, PasswordHash};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;
use rand::RngCore; 
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

fn hash_password(plaintext_password: String, existing_salt: Option<String>) -> String {
    // Generate salt if none was passed
    let salt = existing_salt.unwrap_or(SaltString::generate(&mut OsRng).to_string());
    // Hash password with argon2
    let argon2 = Argon2::default();
    return argon2.hash_password(plaintext_password.as_bytes(), &salt).expect("Password hash failed").to_string();
}

pub fn encrypt_file(file_path: String, password: String) -> () {
    // Hash password
    let password_hash = hash_password(password, None);
    let password_hash_parsed = PasswordHash::new(&password_hash).expect("Hash could not be parsed");
    let hash = password_hash_parsed.hash.expect("Hash could not be output");
    let salt = password_hash_parsed.salt.expect("Salt could not be output");
    // Open selected file
    let input_file = std::fs::read(&file_path).unwrap();
    // Generate random 256 bit encryption key
    let file_key = generate_encryption_key(None);
    // Generate random 96 bit iv
    let file_iv = generate_nonce();
    // Encrypt data with generated key
    let file_cipher = Aes256Gcm::new(&file_key);
    let file_ciphertext = file_cipher.encrypt(&file_iv, input_file.as_ref()).expect("Encryption failure!");
    // Encrypt generated key with password hash
    let password_key = generate_encryption_key(Some(hash.as_bytes()));
    let password_iv = generate_nonce();
    let password_cipher = Aes256Gcm::new(&password_key);
    let password_ciphertext = password_cipher.encrypt(&password_iv, file_key.to_vec().as_ref()).expect("Encryption failure!");
    // Hash hashed password again for storage
    let hashed_hash = hash_password(hash.to_string(), Some(salt.to_string()));
    // Store data in a file
    let mut output: Vec<u8> = vec![];
    // Bytes 0-95 for hashed hash of password (96 bytes)
    output.extend(hashed_hash.as_bytes());
    // Bytes 96-107 for password nonce (12 bytes)
    output.extend(password_iv);
    // Bytes 108-155 for password ciphertext (48 bytes)
    output.extend(&password_ciphertext);
    // Bytes 156-168 for file nonce (12 bytes)
    output.extend(file_iv);
    // Rest of bytes for file ciphertext
    output.extend(&file_ciphertext);
    fs::write(format!("{}.encrypted", file_path), output).expect("Could not write file to disk");
}

pub fn decrypt_file() -> () {

}

fn generate_encryption_key(exisiting_bytes: Option<&[u8]>) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>> {
    // Initialize 256 bit (32 byte) array
    let mut key_bytes = [0u8; 32];
    // Generate key from exisiting bytes or use random bytes
    if let Some(bytes_to_copy) = exisiting_bytes {
        key_bytes[..32].clone_from_slice(bytes_to_copy[..32].as_ref().into());
    } else {
        OsRng.fill_bytes(&mut key_bytes);
    }
    return *Key::from_slice(&key_bytes); 
}

fn generate_nonce() -> GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>> {
    // Initialize 96 bit (12 byte) random array and generate + return nonce
    let mut file_iv_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut file_iv_bytes);
    return *Nonce::from_slice(&file_iv_bytes);
}
 