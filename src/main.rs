use ll1_recursive_descent_lexer;

fn main() {
    println!("Start Language Implementation Pattern");
    println!("Please select a pattern");
    let mut args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("The input is invalid");
        std::process::exit(1);
    }

    ll1_recursive_descent_lexer::lexing(args.pop().unwrap());
}
