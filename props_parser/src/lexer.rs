use lazy_static::lazy_static;
use regex::Regex;

use crate::tokens::Token;

lazy_static! {
    static ref IDENT_REGEX: Regex = Regex::new(r"[a-zA-Z0-9_]").unwrap();
}

fn peak(chars: &[u8], i: usize) -> Option<char> {
    if i >= chars.len() {
        return None;
    }

    Some(chars[i] as char)
}

fn is(chars: &[u8], i: usize, c: char) -> bool {
    peak(chars, i).map(|nc| nc == c).unwrap_or(false)
}

fn is_match<F>(chars: &[u8], i: usize, c: F) -> bool where F: Fn(char) -> bool {
    peak(chars, i).map(c).unwrap_or(false)
}

fn next(chars: &[u8], i: usize) -> (Token, usize) {
    if i >= chars.len() {
        return (Token::EOF, i);
    }

    let c = chars[i] as char;

    match c {
        '|' => (Token::Pipe, i),
        ' ' => (Token::Whitespace, i),
        '{' => (Token::FuncOpen, i),
        '}' => (Token::FuncClose, i),
        '=' => {
            if is(chars, i + 1, '=') {
                return (Token::Equality, i + 1);
            }

            (Token::Assignment, i)
        }
        '+' => (Token::Plus, i),
        '-' => (Token::Minus, i),
        '*' => (Token::Multiplication, i),
        '/' => (Token::Division, i),
        '%' => (Token::Mod, i),
        '^' => (Token::Power, i),
        '(' => (Token::ParenthOpen, i),
        ')' => (Token::ParenthClose, i),
        '!' => (Token::Not, i),
        '>' => {
            if is(chars, i + 1, '=') {
                return (Token::GreaterEqual, i + 1);
            }

            (Token::GreaterThan, i)
        },
        '<' => {
            if is(chars, i + 1, '=') {
                return (Token::LessEqual, i + 1);
            }

            (Token::LessThan, i)
        },
        '\n' => (Token::Newline, i),
        '\t' => {
            let mut new_i = i + 1;
            let mut level = 1;

            while is(chars, new_i, '\t') {
                level += 1;
                new_i += 1;
            }

            (Token::Indent(level), new_i - 1)
        },
        _ => {
            // TODO: maybe possible to avoid .to_string everytime?
            if IDENT_REGEX.is_match(&c.to_string()) {
                let mut ident = vec![c];
                let mut new_i = i + 1;

                while is_match(chars, new_i, |nc| IDENT_REGEX.is_match(&nc.to_string())) {
                    ident.push(chars[new_i] as char);
                    new_i += 1;
                }

                return (Token::Ident(ident.iter().collect()), new_i - 1);
            }

            (Token::Unknown(c.to_string()), i)
        }
    }
}

pub fn lex(source: String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let chars = source.as_bytes();
    let mut i = 0;

    loop {
        if i > chars.len() {
            break;
        }

        let (token, index) = next(chars, i);
        tokens.push(token);
        i = index + 1;
    }

    tokens
}
