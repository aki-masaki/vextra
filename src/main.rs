use crate::args::parse_args;
use crate::parser::Parser;
use std::io::Write;
use std::path::PathBuf;

mod args;
mod ast;
mod parser;

fn main() -> Result<(), std::io::Error> {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {e}.");
            std::process::exit(1);
        }
    };

    let file = match std::fs::read_to_string(args.input) {
        Ok(file) => file,
        Err(err) => {
            println!("{err}");
            std::process::exit(1);
        }
    };

    let parser = Parser::new(file);
    let tokens = parser.tokenize();
    let ast = Parser::parse(&tokens);

    let mut f = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(args.output.unwrap_or(PathBuf::from("index.html")))?;

    f.write_all(ast.render_html().as_bytes())?;
    f.flush()?;

    Ok(())
}
