use std::fmt::Error;

use crate::{envelope::Article, agent::Postman};

pub trait Cert<T> 
where
    T: Postman
{
    fn verify(&self) -> Result<Author, Error>;
    fn assign(&self) -> Result<T, Error>;
}

#[derive(Debug, Clone)]
pub struct Author {
    pub id: i16,
    pub name: String
}

impl Author {
    pub fn write(&self, content: &str) -> Article {
        Article { id: None, content: content.to_string(), author: self.clone() }
    }
}