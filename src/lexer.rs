use regex::Regex;
use std::str::FromStr;

#[derive(Clone)]
pub enum Token {
    Identifier(String),
    Constant(i32),
    Keyword(String),
    OpenPar,
    ClosePar,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

pub fn tokenize(input: String) -> Option<Vec<Token>> {
    let mut content = input;

    let re_open_par = Regex::new(r"\(").unwrap();
    let re_close_par = Regex::new(r"\)").unwrap();
    let re_open_brace = Regex::new(r"\{").unwrap();
    let re_close_brace = Regex::new(r"\}").unwrap();
    let re_semi_colon = Regex::new(r";").unwrap();
    let re_keyword = Regex::new(r"return|int|void").unwrap();
    let re_identifier = Regex::new(r"[a-zA-Z_]\w*").unwrap();
    let re_constant = Regex::new(r"[0-9]+").unwrap();

    // order is important: prioritize keywords
    let regex_list = vec![
        (Token::Keyword(String::new()), re_keyword),
        (Token::Identifier(String::new()), re_identifier),
        (Token::Constant(0), re_constant),
        (Token::OpenPar, re_open_par),
        (Token::ClosePar, re_close_par),
        (Token::OpenBrace, re_open_brace),
        (Token::CloseBrace, re_close_brace),
        (Token::Semicolon, re_semi_colon),
    ];


    // the general idea is to find the first match of all regex matches
    // then we trim the input accordingly, moving the cursor
    let mut tokens = Vec::<Token>::new();
    while !content.is_empty() {
        content = content.trim().to_string();

        let mut token_candidate = (Token::Semicolon, String::new());
        let mut found_token = false;

        for (token, re) in &regex_list {
            let Some(m) = re.captures(&content) else {
                continue;
            };

            match m.get(0) {
                Some(value) => {
                    if value.start() == 0 {
                        token_candidate.0 = token.clone();
                        token_candidate.1 = String::from_str(&m[0]).unwrap();
                        found_token = true;
                        break;
                    }
                },
                None => continue
            }
        }

        if !found_token {
            println!("Error: no match!");
            return None;
        }

        content = content[token_candidate.1.len()..].to_string();

        let token_ = match token_candidate.0 {
            Token::Constant(_) => Token::Constant(token_candidate.1.parse::<i32>().unwrap()),
            Token::Keyword(_) => Token::Keyword(token_candidate.1),
            Token::Identifier(_) => Token::Identifier(token_candidate.1),
            Token::OpenBrace => Token::OpenBrace,
            Token::CloseBrace => Token::CloseBrace,
            Token::OpenPar => Token::OpenPar,
            Token::ClosePar => Token::ClosePar,
            Token::Semicolon => Token::Semicolon,
        };
        tokens.push(token_);
    }
    return Some(tokens);
}

pub fn print_tokens(tokens: &[Token]) {
    for token in tokens {
        match token {
            Token::Constant(n) => println!("Constant({n})"),
            Token::Keyword(kw) => println!("Keyword({kw})"),
            Token::Identifier(id) => println!("Identifier({id})"),
            Token::OpenBrace => println!("{{"),
            Token::CloseBrace => println!("}}"),
            Token::OpenPar => println!("("),
            Token::ClosePar => println!(")"),
            Token::Semicolon => println!(";"),
        }
    }
}
