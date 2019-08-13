use super::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    text: String,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, text: String) -> Self {
        Self { token_type, text }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let token_name = TokenType::token_name_by(self.token_type.to_index());
        write!(f, "<'{}',{}>", self.text, token_name)
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let token_name = TokenType::token_name_by(self.token_type.to_index());
        write!(f, "<'{}',{}>", self.text, token_name)
    }
}
