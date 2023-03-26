use std::fmt::Error;
use crate::envelope::{Article, Markdown};

pub trait Postman {
    fn post(&self, a: &Article) -> Result<Article, Error>;
}

pub trait Librarian {
   fn search(&self, query: &str) -> Result<Markdown, Error>; 
}