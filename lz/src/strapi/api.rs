use std::{fmt::Error};

use reqwest::blocking::{Client, RequestBuilder};

use crate::{ident::{Author, Cert}, envelope::Article, agent::Postman};

use super::{ident::{Jwt, IdentPass}};

#[derive(Clone)]
pub struct Strapi {
    pub host: String,
    jwt: Jwt,
    client: Client
}

pub trait NewStapi<T> {
    fn new(host: &str, ident: T) -> Self;
}

impl NewStapi<IdentPass> for Strapi {
    fn new(host: &str, ident: IdentPass) -> Self {
        let url = Self::url(host, "/api/auth/local");
        let c = reqwest::blocking::Client::new();
        let res = c.post(url).json(&Ident::from(ident)).send().unwrap();
        Self::new(host, Jwt::from(res.json::<AuthResult>().unwrap()))
    }
}

impl NewStapi<Jwt> for Strapi {
    fn new(host: &str, ident: Jwt) -> Self {
        Self { host: host.to_string(), jwt: ident, client: reqwest::blocking::Client::new() }
    }
}

impl Strapi {

    fn me(&self) -> Result<Author, Error> {
        let url = Self::url(&self.host, "/api/users/me");
        match self.auth_get(&url).send() {
            Ok(ok) => {
                Ok(Author::from(ok.json::<User>().unwrap()))
            },
            Err(_) => Err(Error { })
        }
    }
    
    fn post(&self, article: &Article) -> Result<i16, Error> {
        let url = Self::url(&self.host, "/api/articles");
        let post = Post::from(article.to_owned());
        match self.auth_post(&url).json(&post).send() {
            Ok(_) => Ok(1337),
            Err(_) => Err(Error { })
        }
    }

    fn url(host: &str, path: &str) -> String {
        let mut url = url::Url::parse(host).unwrap();
        url.set_path(path);
        return url.to_string()
    }

    fn auth_post(&self, url: &str) -> RequestBuilder {
        self.client.post(url).bearer_auth(self.jwt.token.clone())
    }

    fn auth_get(&self, url: &str) -> RequestBuilder {
        self.client.get(url).bearer_auth(self.jwt.token.clone())
    }

}

impl Cert<Strapi> for Strapi {
    fn verify(&self) -> Result<Author, Error> {
        self.me()
    }

    fn assign(&self) -> Result<Strapi, Error> {
        Ok(self.to_owned())
    }
}

impl Postman for Strapi {
    fn post(&self, a: &Article) -> Result<Article, Error> {
        self.post(a).map(|id| { a.set_id(id) })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthResult {
    jwt: String
}

impl From<AuthResult> for Jwt {
    fn from(value: AuthResult) -> Self {
        Self { token: value.jwt }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i16,
    username: String
}

impl From<User> for Author {
    fn from(value: User) -> Self {
        Self { id: value.id, name: value.username }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    data: PostData
}

#[derive(Debug, Serialize, Deserialize)]
struct PostData {
    content: String,
    author: i16
}

impl From<Article> for Post {
    fn from(value: Article) -> Self {
        Self { data: PostData { content: value.content, author: value.author.id } }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Ident {
    identifier: String,
    password: String
}

impl From<IdentPass> for Ident {
    fn from(value: IdentPass) -> Self {
        Self { identifier: value.ident, password: value.password }
    }
}