fn main() {
    println!("Start Language Implementation Pattern");
    println!("Please select a pattern");
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("The input is invalid");
        std::process::exit(1);
    }
    println!("You input is {}", args[1]);
}
