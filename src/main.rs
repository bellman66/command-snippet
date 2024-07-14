use std::fs::{create_dir, File, read_dir};
use std::io::Read;
use std::string::String;

use clap::Parser;

const DEFAULT_COMMAND_LINE_DIR_PATH: &str = "resources";
const DEFAULT_COMMAND_LINE_FILE_PATH: &str = "command-group.txt";

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[arg(default_value = DEFAULT_COMMAND_LINE_FILE_PATH)]
    path: std::path::PathBuf,
}

// https://rust-cli.github.io/book/tutorial/errors.html
fn main() {
    let args = Cli::parse();

    let mut content = init_default_file();

    let mut file_value = String::new();
    content.read_to_string(&mut file_value).expect("failed to string read");

    for line in file_value.lines() {
        println!("{}", line);
    }
}

fn init_default_file() -> File {
    if let Err(_) = read_dir(DEFAULT_COMMAND_LINE_DIR_PATH) {
        match create_dir(DEFAULT_COMMAND_LINE_DIR_PATH) {
            Ok(_) => {
                read_dir(DEFAULT_COMMAND_LINE_DIR_PATH).expect("Create But Not Found");
            }
            Err(_) => { panic!("Not Create Main Folder"); }
        }
    }

    let full_path = format!("{}/{}", DEFAULT_COMMAND_LINE_DIR_PATH, DEFAULT_COMMAND_LINE_FILE_PATH);

    match File::open(&full_path) {
        Ok(file) => { file }
        Err(_) => { File::create_new(&full_path).expect("failed to create file") }
    }
}
