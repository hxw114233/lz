use std::{io::{self, Read, BufReader}, env, fs};

use agent::{Postman, Librarian};
use ident::Cert;
use meili::api::{MeiliApi, MeiliKey};
use strapi::ident::{IdentPass};

use crate::strapi::api::NewStapi;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod agent;
mod envelope;
mod ident;
mod strapi;
mod meili;

fn main() {
    let path = home::home_dir().unwrap().to_str().unwrap().to_string() + "/.lz/conf.json";
    let c = read_conf(&path);
    if env::args().len() == 1 {
        return post(&c)
    }
    else {
        let q: Vec<String> = env::args().collect();
        return read(&c, &q[1])
    }
}

fn read_conf(path: &str) -> Config {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string()).unwrap();

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string()).unwrap();

    serde_json::from_str::<Config>(&file_content).unwrap()
}

fn read(c: &Config, query: &str) {
    let m = MeiliApi::new(
        &c.meilisearch_host, 
        &c.meilisearch_key
    );
    let a = m.search(query).unwrap();
    println!("{:}", a.0)
}

fn post(c: &Config) {
    let mut body = Vec::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut body).expect("read failed");

    let content = String::from_utf8(body).unwrap();

    let strapi = strapi::api::Strapi::new(&c.strapi_host, c.strapi_key.clone());
    let author = strapi.verify().unwrap();
    let article = author.write(&content);
    let res = strapi.post(&article).unwrap();

    println!("{:?}", res)
}

#[derive(Deserialize)]
struct Config {
    strapi_host: String,
    strapi_key: IdentPass,
    meilisearch_host: String,
    meilisearch_key: MeiliKey
}