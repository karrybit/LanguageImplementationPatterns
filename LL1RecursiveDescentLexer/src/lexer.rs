use super::token::Token;
use super::token_type::TokenType;

pub struct Lexer {
    input: String,
    c: Option<char>,
    p: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let c = input.chars().nth(0);
        Self { input, c, p: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            match self.c {
                Some(' ') | Some('\t') | Some('\n') | Some('\r') => {
                    self.ws();
                    continue;
                }
                Some(',') => {
                    self.consume();
                    return Token::new(TokenType::Comma, ",".to_string());
                }
                Some('[') => {
                    self.consume();
                    return Token::new(TokenType::Lbrack, "[".to_string());
                }
                Some(']') => {
                    self.consume();
                    return Token::new(TokenType::Rbrack, "]".to_string());
                }
                None => {
                    return Token::new(TokenType::EOF, "<EOF>".to_string());
                }
                _ => {
                    if self.is_letter() {
                        return self.name();
                    } else {
                        panic!("invalid charactor: {}", self.c.unwrap());
                    }
                }
            };
        }
    }

    fn name(&mut self) -> Token {
        let mut ident = String::new();
        loop {
            ident.push(self.c.unwrap());
            self.consume();
            if !self.is_letter() {
                break;
            }
        }
        Token::new(TokenType::Name, ident)
    }

    fn is_letter(&self) -> bool {
        self.c.map_or(false, |c| c.is_ascii_alphabetic())
    }

    fn ws(&mut self) {
        while self
            .c
            .map_or(false, |c| c == ' ' || c == '\t' || c == '\n' || c == '\r')
        {
            self.consume();
        }
    }

    fn consume(&mut self) {
        self.p += 1;
        self.c = self.input.chars().nth(self.p);
    }
}
