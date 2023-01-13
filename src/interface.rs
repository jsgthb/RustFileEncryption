use std::fs;
use inquire::{InquireError, Select, Password};

pub fn get_file_path() -> String {
    let mut files: Vec<String> = Vec::new();
    let mut file_selection_options: Vec<String> = Vec::new();
    let directory = fs::read_dir("./files").expect("Could not read directory");
    // Loop through files in file directory
    for file in directory {
        // Store file data
        let file_name = file.as_ref().expect("Could not read file").file_name().into_string().expect("Could not parse file");
        let file_length = file.expect("Could not read file").metadata().expect("Could not parse file").len();
        let file_selection = format!("{} ({})", file_name, pretty_print_filesize(file_length));
        files.push(file_name); 
        file_selection_options.push(file_selection);
    }
    // Exit if no files are found
    if 0.eq(&files.len()) {
        panic!("No files found")
    }
    // Select which file to encrypt or decrypt
    let file_selection: Result<String, InquireError> = Select::new("Select file:", file_selection_options).prompt();
    match file_selection {
        Ok(choice) => return choice.to_string(),
        Err(_) => panic!("Selection error")
    }
}

pub fn select_method() -> String {
    let method_selection_options: Vec<&str> = vec!("Encrypt", "Decrypt");
    let method_selection: Result<&str, InquireError> = Select::new("Select method:", method_selection_options).prompt();
    match method_selection {
        Ok(choice) => return choice.to_string(),
        Err(_) => panic!("Selection error")
    }
}

pub fn enter_password() -> String {
    let password = Password::new("Password:").without_confirmation().prompt();
    match password {
        Ok(pass) => return pass,
        Err(_) => panic!("Password error"),
    }
}

fn pretty_print_filesize(byte_length: u64) -> String {
    // Return filesize in bytes as string
    if byte_length < 1024 {
        return format!("{} Bytes", byte_length);
    } else if byte_length >= 1024 && byte_length < 1024 * 1024 {
        return format!("{:.2} kB", (byte_length / 1024) as f64);
    } else if byte_length >= 1024 * 1024 && byte_length < 1024 * 1024 * 1024 {
        return format!("{:.2} MB", (byte_length / 1024 / 1024) as f64);
    } else {
        return format!("{:.2} GB", (byte_length / 1024 / 1024 / 1024) as f64);
    }
}