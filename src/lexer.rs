#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(i64),
    Assign,
    Plus,
    Minus,
    Semicolon,
    Print,
    Eof,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\t' | '\r' => {
                chars.next();
            }
            '=' => {
                tokens.push(Token::Assign);
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            _ => {
                if c.is_ascii_alphabetic() {
                    let mut ident = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_ascii_alphanumeric() || ch == '_' {
                            ident.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if ident.eq_ignore_ascii_case("PRINT") {
                        tokens.push(Token::Print);
                    } else {
                        tokens.push(Token::Ident(ident));
                    }
                } else {
                    chars.next();
                }
            }
        }
    }

    tokens.push(Token::Eof);
    tokens
}
