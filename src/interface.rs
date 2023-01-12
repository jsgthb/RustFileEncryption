use std::fs;
use dialoguer::{Select, Password};
use dialoguer::theme::ColorfulTheme;

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
    let file_selection =  Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select file")
                .default(0)
                .items(&file_selection_options[..])
                .interact()
                .expect("Dialoguer selection error"); 
    return files[file_selection].clone();
}

pub fn select_method() -> String {
    let method_selection_options: Vec<String> = vec!("Encrypt".to_string(), "Decrypt".to_string());
    let method_selection =  Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Encrypt or decrypt?")
                .default(0)
                .items(&method_selection_options[..])
                .interact()
                .expect("Dialoguer selection error");
    return method_selection_options[method_selection].clone();
}

pub fn enter_password() -> String {
    return Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter password")
        .interact()
        .expect("Dialoguer selection error");
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