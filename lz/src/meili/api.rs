use std::fmt::Error;
use meilisearch_sdk::client::*;
use futures::executor::block_on;
use serde::{Serialize, Deserialize};

use crate::{agent::Librarian, envelope::{Article, Markdown}, ident::Author};

pub struct MeiliApi {
    client: Client
}

#[derive(Debug, Deserialize)]
pub struct MeiliKey {
    pub token: String
}

impl MeiliApi {
    pub fn new(host: &str, api_key: &MeiliKey) -> Self {
        Self { client: Client::new(host, api_key.token.clone()) }
    }
}

impl Librarian for MeiliApi {
    fn search(&self, query: &str) -> Result<Markdown, Error> {
        let mut res = block_on(async move {
            self.client
                .index("article")
                .search()
                .with_query(query)
                .execute::<MeiliArticle>()
                .await
                .unwrap()
        });
        if res.hits.len() > 0 {
            res.hits.sort_by(
                |a, b| { b.result.partial_cmp(&a.result).unwrap() }
            );
            let collect = 5.min(res.hits.len());
            let c = res.hits[0..collect].iter().map(|r| { r.result.clone() }).collect::<Vec<MeiliArticle>>(); 
            Ok(Markdown::from(c))
        } else {
            Ok(Markdown("# 404\n\n".to_owned()))
        }
    }
} 

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MeiliArticle {
    id: i32,
    content: String,
    author: MeiliAuthor
}

impl PartialEq for MeiliArticle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for MeiliArticle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl From<MeiliArticle> for Article {
    fn from(value: MeiliArticle) -> Self {
        Self { 
            id: Some(value.id.try_into().unwrap()), 
            content: value.content, 
            author: Author::from(value.author)
        }
    }
}

impl From<MeiliArticle> for Markdown {
    fn from(value: MeiliArticle) -> Self {
        Markdown::from(Article::from(value))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MeiliAuthor {
    id: i32,
    username: String
}

impl From<MeiliAuthor> for Author {
    fn from(value: MeiliAuthor) -> Self {
        Self { id: value.id.try_into().unwrap(), name: value.username }
    }
}

impl From<Vec<MeiliArticle>> for Markdown {
    fn from(value: Vec<MeiliArticle>) -> Self {
        let c = value
            .iter()
            .map(|x| { Markdown::from(x.to_owned()).0 })
            .collect::<Vec<String>>()
            .join("\n\n-----\n\n");
        Self(c)
    }
}