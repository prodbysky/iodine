mod built_in_words;
mod errors;
mod interpreter;
mod lexer;
mod stack;

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
    let mut interpreter = interpreter::Interpreter::new(lexer::Lexer::new(&source), None, None);
    match interpreter.run() {
        Ok(()) => {}
        Err(e) => eprintln!("{}", e),
    }
}
