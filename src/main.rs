use std::env;
use std::fs;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];
    let mut file_content = fs::read_to_string(filepath).expect("Couldn't read file.");

    let tokens = lexer::tokenize(&mut file_content);
    lexer::print_tokens(&tokens);
}
