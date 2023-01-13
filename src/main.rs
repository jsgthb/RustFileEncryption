mod interface;
mod aes_cryptography;

fn main() {
    let file_path = interface::get_file_path();
    let method = interface::select_method();
    let password = aes_cryptography::hash_password(interface::enter_password());
    // Encrypt or decrypt
    match method.as_str() {
        "Encrypt" => println!("{}; {}; {}", file_path, method, password),
        _=> println!("{}; {}; {}", file_path, method, password)
    }
}