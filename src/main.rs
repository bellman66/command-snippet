use std::fs::{create_dir, File, read_dir};
use std::io::Read;

use clap::Parser;

use crate::core::parse::parser::Separate;
use crate::core::parse::text::text_parser::TextSnippetContext;

mod core;

const DEFAULT_COMMAND_LINE_DIR_PATH: &str = "resources";
const DEFAULT_COMMAND_LINE_FILE_PATH: &str = "command-group.txt";

#[derive(Parser)]
struct Cli {
    #[arg(default_value = DEFAULT_COMMAND_LINE_DIR_PATH)]
    dir: std::path::PathBuf,
    #[arg(default_value = DEFAULT_COMMAND_LINE_FILE_PATH)]
    path: std::path::PathBuf,
}

impl Cli {
    fn get_full_path(&self) -> String {
        let dir = self.dir.to_str().expect("not found dir");
        let path = self.path.to_str().expect("not found dir");

        format!("{}/{}", dir, path)
    }

    fn get_dir_path(&self) -> &str {
        self.dir.to_str().expect("not found dir")
    }
}

// https://rust-cli.github.io/book/tutorial/errors.html
fn main() {
    let args = Cli::parse();

    let mut file = init_default_file(&args);

    let context = TextSnippetContext::create();
    context.execute(&mut file);
}

fn init_default_file(cli: &Cli) -> File {
    let dir = cli.get_dir_path();

    if let Err(_) = read_dir(dir) {
        match create_dir(dir) {
            Ok(_) => {
                read_dir(dir).expect("Create But Not Found");
            }
            Err(_) => { panic!("Not Create Main Folder"); }
        }
    }

    let full_path = cli.get_full_path();

    match File::open(&full_path) {
        Ok(file) => { file }
        Err(_) => { File::create_new(&full_path).expect("failed to create file") }
    }
}
