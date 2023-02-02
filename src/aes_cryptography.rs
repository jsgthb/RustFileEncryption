use std::fs;
use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;
use aes_gcm::{Aes256Gcm, KeyInit, AeadCore};
use aes_gcm::aead::Aead;

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
    let file_key = Aes256Gcm::generate_key(&mut OsRng);
    // Generate random 96 bit iv
    let file_iv = Aes256Gcm::generate_nonce(&mut OsRng);
    // Encrypt data with generated key
    let file_cipher = Aes256Gcm::new(&file_key);
    let file_ciphertext = file_cipher.encrypt(&file_iv, input_file.as_ref()).expect("Encryption failure!");
    // Encrypt generated key with password hash
    let password_iv = Aes256Gcm::generate_nonce(&mut OsRng);
    let password_cipher = Aes256Gcm::new_from_slice(hash.as_bytes()).expect("Could not generate AES key");
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

pub fn decrypt_file(file_path: String, password: String) -> () {
    // Get encryption data from file
    let input_file = fs::read(file_path).expect("Could not read file");
    let (hashed_hash_bytes, input_file) = input_file.split_at(96);
    let (password_iv_bytes, input_file) = input_file.split_at(12);
    let (password_ciphertext, input_file) = input_file.split_at(48);
    let (file_iv, file_ciphertext) = input_file.split_at(12);
    let hashed_hash = String::from_utf8(hashed_hash_bytes.to_vec()).expect("Could not parse hash");
    if let Some(password_encryption_key) = verify_password(password, hashed_hash) {

    } else {
        println!("Decryption failure: Password is incorrect")
    }
}

fn verify_password(plaintext_password: String, hashed_hash: String) -> Option<Vec<u8>> {
    // Verify if password is correct and return encryption key if so (returns None if incorrect)
    let verification_hash_parsed = PasswordHash::new(&hashed_hash).expect("Hash could not be parsed");
    let verification_salt = verification_hash_parsed.salt.expect("Salt could not be output").to_string();
    let hashed_password = hash_password(plaintext_password, Some(verification_salt));
    let hashed_password_parsed = PasswordHash::new(&hashed_password).expect("Hash could not be parsed");
    let password_hash = hashed_password_parsed.hash.expect("Hash could not be output").to_string();
    let matches = Argon2::default().verify_password(password_hash.as_bytes(), &verification_hash_parsed).is_ok();
    if matches {
        return Some(password_hash.as_bytes().to_vec());
    } else {
        return None;
    }
}