use std::{fs};
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

fn main() {
    let mut files: Vec<String> = Vec::new();
    let mut selection_options: Vec<String> = Vec::new();
    // Loop through files in file directory
    for file in fs::read_dir("./files").expect("Could not read directory") {
        // Store file data
        let file_name = file.as_ref().expect("Could not read file").file_name().into_string().unwrap();
        let file_length = file.expect("Could not read file").metadata().unwrap().len();
        files.push(file_name.clone());
        selection_options.push(format!("{} ({})", file_name, pretty_print_filesize(file_length)));
    }
    // Exit if no files are found
    if files.len() == 0 {
        panic!("No files found")
    } 
    // Select which file to encrypt
    let selection =  Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select file to encrypt")
                .default(0)
                .items(&selection_options[..])
                .interact()
                .expect("Dialoguer selection error");
    println!("{} selected", files[selection])
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