use ll1_recursive_descent_lexer;
use ll1_recursive_descent_parser;

fn main() {
    println!("Start Language Implementation Pattern");
    println!("Please select a pattern");
    let mut args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("The input is invalid");
        std::process::exit(1);
    }

    let program = args.pop().unwrap();

    ll1_recursive_descent_lexer::lexing(&program);
    ll1_recursive_descent_parser::parse(&program);
}
