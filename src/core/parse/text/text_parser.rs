use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::core::parse::parser::{Context, Separate, Snippet};

struct TextSnippet {
    sequence: usize,
    title: String,
    content: String,
}

pub struct TextSnippetContext {
    id_table: HashMap<usize, Box<dyn Snippet>>,
    title_table: HashMap<String, Box<dyn Snippet>>,
}

impl TextSnippetContext {
    pub fn create() -> Self {
        TextSnippetContext {
            id_table: Default::default(),
            title_table: Default::default(),
        }
    }
}

impl Snippet for TextSnippet {
    fn get_content(&self) -> &str {
        self.content.as_str()
    }
}

impl Context for TextSnippetContext {
    fn find_by_id(&self, id: usize) -> Option<&Box<dyn Snippet>> {
        self.id_table.get(&id)
    }

    fn find_by_title(&self, title: &str) -> Option<&Box<dyn Snippet>> {
        self.title_table.get(title)
    }
}

impl Separate for TextSnippetContext {
    fn execute(&self, file: &mut File) -> Option<&'static Box<dyn Context>> {
        let mut content = String::new();
        file.read_to_string(&mut content).expect("failed to string read");

        for line in content.lines() {
            println!("{}", line);
        }
        None
    }
}
