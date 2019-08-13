mod parser;

use ll1_recursive_descent_lexer::lexer::Lexer;
use parser::Parser;

pub fn parse(program: &String) {
    let lexer = Lexer::new(program.clone());
    let mut parser = Parser::new(lexer);
    parser.list();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
