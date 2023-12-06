use crate::tokens::Token;
use lazy_static::lazy_static;
use regex::Regex;
use crate::types::Number;

lazy_static! {
    static ref IDENT_REGEX: Regex = Regex::new(r"[a-zA-Z0-9_]").unwrap();
    static ref LETTER_REGEX: Regex = Regex::new(r"[a-zA-Z_]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"-?[0-9.]").unwrap();
}

const RETURN_WORD_LEN: usize = 6;

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

    fn next_token(chars: &[u8], i: usize) -> Option<(Token, usize)> {
        if i >= chars.len() {
            return None;
        }

        let c = chars[i] as char;

        Some(match c {
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
                return Some((Token::StringLiteral(str.iter().collect()), new_i));
            }
            '{' => (Token::FuncOpen, i),
            '}' => (Token::FuncClose, i),
            '=' => {
                if Lexer::is(chars, i + 1, '=') {
                    return Some((Token::Equality, i + 1));
                }

                (Token::Assignment, i)
            }
            '+' => (Token::Addition, i),
            '-' => (Token::Subtraction, i),
            '*' => (Token::Multiplication, i),
            '/' => {
                if Lexer::is(chars, i + 1, '/') {
                    let mut new_i = i + 2;

                    while Lexer::is_match(chars, new_i, |nc| nc != '\n') {
                        new_i += 1;
                    }

                    // + 1 because not directly returning therefore not advancing
                    return Lexer::next_token(chars, new_i + 1);
                }

                (Token::Division, i)
            }
            '%' => (Token::Mod, i),
            '^' => (Token::Power, i),
            '(' => (Token::ParenthOpen, i),
            ')' => (Token::ParenthClose, i),
            '!' => (Token::Not, i),
            '>' => {
                if Lexer::is(chars, i + 1, '=') {
                    return Some((Token::GreaterEqual, i + 1));
                }

                (Token::GreaterThan, i)
            }
            '<' => {
                if Lexer::is(chars, i + 1, '=') {
                    return Some((Token::LessEqual, i + 1));
                }

                (Token::LessThan, i)
            }
            ' ' => (Token::Whitespace, i),
            '\t' => {
                let mut new_i = i + 1;
                let mut level = 1;

                while Lexer::is(chars, new_i, '\t') {
                    level += 1;
                    new_i += 1;
                }

                (Token::Indent(level), new_i - 1)
            }
            'r' => {
                if i + RETURN_WORD_LEN < chars.len() {
                    let word = &chars[i..i + RETURN_WORD_LEN];
                    return match std::str::from_utf8(word) {
                        Ok(str) => Some(if str == "return" { (Token::Return, i + RETURN_WORD_LEN) } else { Lexer::tokenize_else(c, chars, i) }),
                        Err(_) => Some(Lexer::tokenize_else(c, chars, i))
                    };
                }

                Lexer::tokenize_else(c, chars, i)
            }
            _ => Lexer::tokenize_else(c, chars, i)
        })
    }

    fn tokenize_else(c: char, chars: &[u8], i: usize) -> (Token, usize) {
        let str = c.to_string();

        if LETTER_REGEX.is_match(&str) {
            let mut ident = vec![c];
            let mut new_i = i + 1;

            while Lexer::is_match(chars, new_i, |nc| IDENT_REGEX.is_match(&nc.to_string())) {
                ident.push(chars[new_i] as char);
                new_i += 1;
            }

            return (Token::Ident(ident.iter().collect()), new_i - 1);
        }

        if NUMBER_REGEX.is_match(&str) {
            match Lexer::lex_number(c, i, chars) {
                Some(t) => t,
                None => (Token::Unknown(str), i)
            }
        } else {
            (Token::Unknown(str), i)
        }
    }

    fn lex_number(c: char, i: usize, chars: &[u8]) -> Option<(Token, usize)> {
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
            return Some((Token::Number(num), new_i - 1));
        }

        None
    }

    pub fn lex(source: &str) -> Vec<(Token, usize)> {
        let mut tokens = Vec::<(Token, usize)>::new();
        let mut lines = source.lines();

        while let Some(line) = lines.next() {
            let mut i = 0;
            let chars = line.as_bytes();

            loop {
                let token = Lexer::next_token(chars, i);

                if token.is_none() {
                    break;
                }

                let token = token.unwrap();

                i = token.1 + 1;
                tokens.push(token);
            }

            tokens.push((Token::Newline, i));
        }

        tokens
    }
}