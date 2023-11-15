use crate::tokens::Token;
use lazy_static::lazy_static;
use regex::Regex;
use crate::types::Number;

lazy_static! {
    static ref IDENT_REGEX: Regex = Regex::new(r"[a-zA-Z0-9_]").unwrap();
    static ref LETTER_REGEX: Regex = Regex::new(r"[a-zA-Z_]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"[0-9.]").unwrap();
}

pub struct Lexer;
impl Lexer {
    fn get_char(chars: &[u8], i: usize) -> Option<char> {
        if i >= chars.len() {
            return None;
        }

        Some(chars[i] as char)
    }

    fn is(chars: &[u8], i: usize, c: char) -> bool {
        Lexer::get_char(chars, i).map(|nc| nc == c).unwrap_or(false)
    }

    fn is_match<F>(chars: &[u8], i: usize, c: F) -> bool
        where F: Fn(char) -> bool,
    {
        Lexer::get_char(chars, i).map(c).unwrap_or(false)
    }

    fn next_token(chars: &[u8], i: usize) -> (Token, usize) {
        if i >= chars.len() {
            return (Token::EOF, i);
        }

        let c = chars[i] as char;

        match c {
            '|' => (Token::Pipe, i),
            ':' => (Token::TypeAnnotator, i),
            ',' => (Token::Comma, i),
            '.' => (Token::Period, i),
            '"' => {
                let mut str = Vec::new();
                let mut new_i = i + 1;

                while Lexer::is_match(chars, new_i, |nc| nc != '"') {
                    str.push(chars[new_i] as char);
                    new_i += 1;
                }

                // not new_i - 1 to consume the ending "
                return (Token::StringLiteral(str.iter().collect()), new_i);
            },
            '{' => (Token::FuncOpen, i),
            '}' => (Token::FuncClose, i),
            '=' => {
                if Lexer::is(chars, i + 1, '=') {
                    return (Token::Equality, i + 1);
                }

                (Token::Assignment, i)
            }
            '+' => (Token::Addition, i),
            '-' => (Token::Subtraction, i),
            '*' => (Token::Multiplication, i),
            '/' => (Token::Division, i),
            '%' => (Token::Mod, i),
            '^' => (Token::Power, i),
            '(' => (Token::ParenthOpen, i),
            ')' => (Token::ParenthClose, i),
            '!' => (Token::Not, i),
            '>' => {
                if Lexer::is(chars, i + 1, '=') {
                    return (Token::GreaterEqual, i + 1);
                }

                (Token::GreaterThan, i)
            }
            '<' => {
                if Lexer::is(chars, i + 1, '=') {
                    return (Token::LessEqual, i + 1);
                }

                (Token::LessThan, i)
            }
            ' ' => (Token::Whitespace, i),
            '\n' => (Token::Newline, i),
            '\t' => {
                let mut new_i = i + 1;
                let mut level = 1;

                while Lexer::is(chars, new_i, '\t') {
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

                    while Lexer::is_match(chars, new_i, |nc| IDENT_REGEX.is_match(&nc.to_string())) {
                        ident.push(chars[new_i] as char);
                        new_i += 1;
                    }

                    return (Token::Ident(ident.iter().collect()), new_i - 1);
                }

                if NUMBER_REGEX.is_match(&c.to_string()) {
                    let mut number = vec![c];
                    let mut new_i = i + 1;
                    let mut has_decimal = false;

                    while Lexer::is_match(chars, new_i, |nc| NUMBER_REGEX.is_match(&nc.to_string())) {
                        let c = chars[new_i] as char;
                        number.push(c);
                        new_i += 1;

                        if !has_decimal {
                            has_decimal = c == '.';
                        }
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
        let mut lines = source.lines();

        while let Some(line) = lines.next() {
            let mut i = 0;
            let chars = line.as_bytes();

            while i <= chars.len() {
                let token = Lexer::next_token(chars, i);
                i = token.1 + 1;
                tokens.push(token);
            }
        }

        tokens
    }
}