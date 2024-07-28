use std::collections::HashMap;

use crate::core::parse::parser::{Context, Snippet};

struct TextSnippet {
    sequence: usize,
    title: String,
    content: String,
}

struct TextSnippetContext {
    id_table: HashMap<usize, Box<dyn Snippet>>,
    title_table: HashMap<String, Box<dyn Snippet>>,
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
