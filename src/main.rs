mod interface;
mod aes_cryptography;

fn main() {
    let file_path = interface::get_file_path();
    let method = interface::select_method();
    let plaintext_password = interface::enter_password();
    // Encrypt or decrypt
    match method.as_str() {
        "Encrypt" => println!("Encrypt file"),
        _=> println!("Decrypt file")
    }
}