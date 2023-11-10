use crate::tokens::{Token, Number};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref IDENT_REGEX: Regex = Regex::new(r"[a-zA-Z0-9_]").unwrap();
    static ref LETTER_REGEX: Regex = Regex::new(r"[a-zA-Z_]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"[0-9.]").unwrap();
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

fn is_match<F>(chars: &[u8], i: usize, c: F) -> bool
where
    F: Fn(char) -> bool,
{
    peak(chars, i).map(c).unwrap_or(false)
}

fn next(chars: &[u8], i: usize) -> (Token, usize) {
    if i >= chars.len() {
        return (Token::EOF, i);
    }

    let c = chars[i] as char;

    match c {
        '|' => (Token::Pipe, i),
        ':' => (Token::TypeAnnotator, i),
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
        }
        '<' => {
            if is(chars, i + 1, '=') {
                return (Token::LessEqual, i + 1);
            }

            (Token::LessThan, i)
        }
        '\n' => (Token::Newline, i),
        '\t' => {
            let mut new_i = i + 1;
            let mut level = 1;

            while is(chars, new_i, '\t') {
                level += 1;
                new_i += 1;
            }

            (Token::Indent(level), new_i - 1)
        }
        _ => {
            // TODO: maybe possible to avoid .to_string everytime?
            if LETTER_REGEX.is_match(&c.to_string()) {
                let mut ident = vec![c];
                let mut new_i = i + 1;

                while is_match(chars, new_i, |nc| IDENT_REGEX.is_match(&nc.to_string())) {
                    ident.push(chars[new_i] as char);
                    new_i += 1;
                }

                return (Token::Ident(ident.iter().collect()), new_i - 1);
            }

            if NUMBER_REGEX.is_match(&c.to_string()) {
                let mut number = vec![c];
                let mut new_i = i + 1;
                let mut has_decimal = false;

                while is_match(chars, new_i, |nc| NUMBER_REGEX.is_match(&nc.to_string())) {
                    number.push(chars[new_i] as char);
                    new_i += 1;
                }

                let number_str: String = number.iter().collect();
                if let Ok(num) = Number::parse_number(&number_str, has_decimal) {
                    return (Token::Number(num), new_i - 1);
                }
            }

            (Token::Unknown(c.to_string()), i)
        }
    }
}

pub fn lex(source: String) -> Vec<(Token, usize)> {
    let mut tokens = Vec::<(Token, usize)>::new();
    let chars = source.as_bytes();
    let mut i = 0;

    loop {
        if i > chars.len() {
            break;
        }

        let token = next(chars, i);
        i = token.1 + 1;
        tokens.push(token);
    }

    tokens
}
