use std::fs::File;

pub trait Snippet {
    fn get_content(&self) -> &str;
}

pub trait Context {
    fn find_by_id(&self, id: usize) -> Option<&Box<dyn Snippet>>;

    fn find_by_title(&self, title: &str) -> Option<&Box<dyn Snippet>>;
}

pub trait Separate {
    fn execute(file: File) -> Option<&Box<dyn Context>>;
}
