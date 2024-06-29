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
    source_file: String,
}

fn main() {
    let args = Args::parse();
    let source = match std::fs::read_to_string(args.source_file) {
        Ok(str) => str,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut interpreter = interpreter::Interpreter::new(lexer::Lexer::new(&source), None, None);
    match interpreter.run() {
        Ok(()) => {}
        Err(e) => eprintln!("{}", e),
    }
}
