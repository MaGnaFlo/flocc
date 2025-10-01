use std::env;
use std::fs;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];
    let file_content = fs::read_to_string(filepath).expect("Couldn't read file.");

    match lexer::tokenize(file_content) {
        Some(tokens) => lexer::print_tokens(&tokens),
        None => panic!("Wow! Can't lex that!"),
    }
}
