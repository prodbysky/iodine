mod errors;
mod lexer;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the source file to compile
    #[arg(short)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let source = std::fs::read_to_string(args.input);

    if source.is_err() {
        eprintln!("Failed to read input file because: {}", source.unwrap_err());
        return;
    }
    let source = source.unwrap();

    let lexer = lexer::Lexer::new(&source);

    for token in lexer {
        println!("{:?}", token);
    }
}
