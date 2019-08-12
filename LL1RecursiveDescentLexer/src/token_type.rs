pub enum TokenType {
    EOF,
    Name,
    Comma,
    Lbrack,
    Rbrack,
}

impl TokenType {
    pub(crate) fn name(&self) -> String {
        match self {
            TokenType::EOF => "<EOF>".to_string(),
            TokenType::Name => "NAME".to_string(),
            TokenType::Comma => "COMMA".to_string(),
            TokenType::Lbrack => "LBRACK".to_string(),
            TokenType::Rbrack => "RBRACK".to_string(),
        }
    }
}
