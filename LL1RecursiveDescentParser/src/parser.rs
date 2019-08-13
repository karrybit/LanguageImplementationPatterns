use ll1_recursive_descent_lexer::lexer::Lexer;
use ll1_recursive_descent_lexer::token::Token;
use ll1_recursive_descent_lexer::token_type::TokenType;

pub(crate) struct Parser {
    input: Lexer,
    look_ahead: Option<Token>,
}

impl Parser {
    pub(crate) fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            input: lexer,
            look_ahead: None,
        };
        parser.consume();
        parser
    }

    pub(crate) fn list(&mut self) {
        self.match_token_type(TokenType::Lbrack);
        self.elements();
        self.match_token_type(TokenType::Rbrack);
    }

    fn elements(&mut self) {
        self.element();
        while self
            .look_ahead
            .as_ref()
            .map_or(false, |token| token.token_type == TokenType::Comma)
        {
            self.match_token_type(TokenType::Comma);
            self.element();
        }
    }

    fn element(&mut self) {
        match self.look_ahead.as_ref() {
            Some(token) if token.token_type == TokenType::Name => {
                self.match_token_type(TokenType::Name)
            }
            Some(token) if token.token_type == TokenType::Lbrack => self.list(),
            _ => panic!("expecting name of list; found {:?}", self.look_ahead),
        }
    }

    fn match_token_type(&mut self, token_type: TokenType) {
        if self
            .look_ahead
            .as_ref()
            .map_or(false, |token| token.token_type == token_type)
        {
            self.consume();
        } else {
            panic!(
                "expecting {}; found {}",
                TokenType::token_name_by(token_type.to_index()),
                TokenType::token_name_by(self.look_ahead.as_ref().unwrap().token_type.to_index())
            );
        }
    }

    fn consume(&mut self) {
        self.look_ahead = Some(self.input.next_token());
    }
}
