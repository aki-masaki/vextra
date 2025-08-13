use crate::parser::Parser;
use crate::args::parse_args;

mod args;
mod parser;

fn main() -> Result<(), std::io::Error> {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let file = match std::fs::read_to_string(args.input) {
        Ok(file) => file,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    println!("{}", file);

    let parser = Parser::new(file);
    let tokens = parser.tokenize();
    let ast = parser.parse(&tokens);

    println!("{:?}", ast);

    Ok(())
}
