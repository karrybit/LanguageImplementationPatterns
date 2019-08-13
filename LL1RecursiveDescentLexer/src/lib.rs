pub mod lexer;
pub mod token;
pub mod token_type;

use lexer::Lexer;
use token_type::TokenType;

pub fn lexing(input: &String) {
    let mut lexer = Lexer::new(input.clone());
    let mut token = lexer.next_token();
    while match token.token_type {
        TokenType::EOF => false,
        _ => true,
    } {
        println!("{}", token);
        token = lexer.next_token();
    }
    println!("{}", token);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
