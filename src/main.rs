use std::fs;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};
use rand::RngCore;
use rand::rngs::OsRng; 
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut files: Vec<String> = Vec::new();
    let mut file_selection_options: Vec<String> = Vec::new();
    // Loop through files in file directory
    for file in fs::read_dir("./files").expect("Could not read directory") {
        // Store file data
        let file_name = file.as_ref().expect("Could not read file").file_name().into_string().unwrap();
        let file_length = file.expect("Could not read file").metadata().unwrap().len();
        files.push(file_name.clone());
        file_selection_options.push(format!("{} ({})", file_name, pretty_print_filesize(file_length)));
    }
    // Exit if no files are found
    if files.len() == 0 {
        panic!("No files found")
    } 
    // Select which file to encrypt or decrypt
    let file_selection =  Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select file")
                .default(0)
                .items(&file_selection_options[..])
                .interact()
                .expect("Dialoguer selection error");
    // Select encryption or decryption
    let method_selection_options: Vec<String> = vec!("Encrypt".to_string(), "Decrypt".to_string());
    let method_selection =  Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Encrypt or decrypt?")
                .default(0)
                .items(&method_selection_options[..])
                .interact()
                .expect("Dialoguer selection error");
    match &method_selection_options[method_selection][..] {
        "Encrypt" => {
            encrypt_file("./files/test.txt".to_string());
        },
        _=> println!("Decryption not added yet")
    }

}

pub fn pretty_print_filesize(length: u64) -> String {
    // Return filesize in bytes as string
    if length < 1024 {
        return format!("{} Bytes", length);
    } else if length >= 1024 && length < 1024 * 1024 {
        return format!("{:.2} kB", (length / 1024) as f64);
    } else if length >= 1024 * 1024 && length < 1024 * 1024 * 1024 {
        return format!("{:.2} MB", (length / 1024 / 1024) as f64);
    } else {
        return format!("{:.2} GB", (length / 1024 / 1024 / 1024) as f64);
    }
}

fn encrypt_file(file_path: String) {
    // Open selected file
    let input_file = std::fs::read(file_path.clone()).unwrap();
    // 256 bit key
    let key = Key::from_slice(b"00000000000000000000000000000000");
    let cipher = Aes256Gcm::new(key);
    // Generate random 96 bit nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    // Encrypt data
    let ciphertext = cipher.encrypt(nonce, input_file.as_ref()).expect("Encryption failure!");
    // Store data
    fs::write(format!("{}.encrypted", file_path), ciphertext).expect("Could not write file to disk");
}