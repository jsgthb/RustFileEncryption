use argon2::{Argon2, PasswordHasher, PasswordHash};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;
use rand::RngCore; 
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

pub fn hash_password(plaintext_password: String) -> () {
    // Generate salt
    let salt = SaltString::generate(&mut OsRng);
    // Hash password
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(plaintext_password.as_bytes(), &salt).expect("Password hash failed").to_string();
}

pub fn encrypt_file(file_path: String, password_hash: String) -> () {
    // Convert hash string to password hash
    let parsed_hash_full = PasswordHash::new(&password_hash).expect("Hash could not be parsed");
    let hash = parsed_hash_full.hash.expect("Hash could not be output").to_string();
    // Open selected file
    let input_file = std::fs::read(file_path.clone()).unwrap();
    // Generate random 256 bit key
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    let file_key = Key::from_slice(&key_bytes);
    let file_cipher = Aes256Gcm::new(file_key);
    // Generate random 96 bit iv
    let mut file_iv_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut file_iv_bytes);
    let file_iv = Nonce::from_slice(&file_iv_bytes);
    // Encrypt data with generated key
    let file_ciphertext = file_cipher.encrypt(file_iv, input_file.as_ref()).expect("Encryption failure!");
    // Encrypt generated key with password hash
    let mut password_key_bytes = [0u8; 32];
    password_key_bytes[..32].clone_from_slice(hash.as_bytes()[..32].as_ref().into());
    let password_key = Key::from_slice(&password_key_bytes);
    let password_cipher = Aes256Gcm::new(password_key);
    let mut password_iv_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut password_iv_bytes);
    let password_iv = Nonce::from_slice(&password_iv_bytes);
    let password_ciphertext = file_cipher.encrypt(password_iv, password_key_bytes.as_ref()).expect("Encryption failure!");
    // Store data
    //fs::write(format!("{}.encrypted", file_path), ciphertext).expect("Could not write file to disk");
}

pub fn decrypt_file() -> () {

}