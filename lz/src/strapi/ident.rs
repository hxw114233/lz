#[derive(Clone, Deserialize)]
pub struct IdentPass {
    pub ident: String,
    pub password: String
}

#[derive(Clone, Deserialize)]
pub struct Jwt {
    pub token: String
}

impl Jwt {
    pub fn new(token: &str) -> Self {
        Self { token: token.to_string() }
    }
}
