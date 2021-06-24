#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenType {
    ILLEGAL,
    IDENT,
    EOF,
    GET,
    SET,
    DEL,
    ALL,
}

pub struct Token {
    pub tokentype: TokenType,
    literal: String,
}

impl Token {
    pub fn new(tokentype: TokenType, literal: String) -> Self {
        Self {tokentype, literal}
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {})", self.tokentype, self.literal)
    }
}

pub fn generate_keyword_token(data: &Vec<char>) -> Result<TokenType, String> {
    let content: String = data.into_iter().collect();

    match &content[..] {
        "GET" => Ok(TokenType::GET),
        "SET" => Ok(TokenType::SET),
        "DEL" => Ok(TokenType::DEL),
        "ALL" => Ok(TokenType::ALL),
        _ => Err(String::from("Not a keyword"))
    }
}
