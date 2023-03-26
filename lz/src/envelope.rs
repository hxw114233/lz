use crate::ident::Author;

pub struct Markdown(pub String);

#[derive(Debug, Clone)]
pub struct Article {
    pub id: Option<i16>,
    pub content: String,
    pub author: Author,
}

impl Article {
    pub fn set_id(&self, id: i16) -> Self {
        Self { id: Some(id), content: self.content.clone(), author: self.author.clone() }
    }
}

impl From<Article> for Markdown {
    fn from(value: Article) -> Self {
        let author: String = "__author: ".to_string() + &value.author.name + "__";
        let content = value.content.clone() + "\n\n" + &author;
        Markdown(content)
    }
}
