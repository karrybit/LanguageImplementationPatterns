#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenType {
    EOF,
    Name,
    Comma,
    Lbrack,
    Rbrack,
}

static TOKEN_NAMES: [&'static str; 5] = ["<EOF>", "NAME", "COMMA", "LBRACK", "RBRACK"];

impl TokenType {
    pub fn token_name_by(i: usize) -> String {
        TOKEN_NAMES[i].to_string()
    }

    pub fn to_index(&self) -> usize {
        match self {
            TokenType::EOF => 0,
            TokenType::Name => 1,
            TokenType::Comma => 2,
            TokenType::Lbrack => 3,
            TokenType::Rbrack => 4,
        }
    }
}
