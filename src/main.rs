mod interface;
mod aes_cryptography;

fn main() {
    let file_path = interface::get_file_path();
    let method = interface::select_method();
    let password = interface::enter_password();
    // Encrypt or decrypt
    match method.as_str() {
        "Encrypt" => {
            println!("Encrypting file...");
            aes_cryptography::encrypt_file(file_path, password);
        },
        _=> println!("Decryption not implemented yet")
    }
}