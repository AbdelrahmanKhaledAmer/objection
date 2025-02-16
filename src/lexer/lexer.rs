use std::iter::Peekable;
use std::str::Chars;

use super::tokens::Token;
use super::tokens::TokenType;
use super::tokens::get_keywords;
use super::tokens::get_symbols;

pub fn get_tokens(source_code: String) -> Vec<Token> {
    let mut chars = source_code.chars().peekable();
    let mut line = 1;
    let mut column = 1;

    let symbols = get_symbols();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(c) = chars.next() {
        // Whitespace and comments are skipped completely
        if c == '\n' {
            line += 1;
            column = 1;
        } else if c.is_whitespace() {
            column += 1;
        } else if c == '/' && chars.clone().next() == Some('/') {
            let mut found_newline = false;
            while let Some(c) = chars.next() {
                if c == '\n' {
                    line += 1;
                    column = 1;
                    found_newline = true;
                    break;
                }
            }
            if !found_newline {
                break;
            }
        } else if c == '/' && chars.clone().next() == Some('*') {
            let mut found_end = false;
            while let Some(c) = chars.next() {
                if c == '*' && chars.clone().next() == Some('/') {
                    chars.next();
                    column += 2;
                    found_end = true;
                    break;
                } else if c == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
            }
            if !found_end {
                break;
            }

        // Keywords and identifiers
        } else if c.is_alphabetic() || c == '_' {
            tokens.push(keyword_or_id(&mut chars, &line, &mut column, c));

        // Integer Literals
        } else if c.is_numeric() {
            tokens.push(int_lit(&mut chars, &line, &mut column, c));

        // Symbols
        } else if symbols.keys().any(|symbol| symbol == &c.to_string()) {
            let value = c.to_string();
            let token_type = *symbols.get(value.as_str()).unwrap();
            tokens.push(Token{token_type, value, line, column});
            column += 1;

        // Unknown token found
        } else {
            tokens.push(Token {
                token_type: TokenType::UNKNOWN,
                value: c.to_string(),
                line,
                column,
            });
        }
    }

    tokens.push(Token {
        token_type: TokenType::EOF,
        value: String::from("<EOF>"),
        line,
        column,
    });
    return tokens;
}

fn keyword_or_id(chars: &mut Peekable<Chars>, line_start: &u32, column_start: &mut u32, first_char: char) -> Token {
    let mut value = first_char.to_string();
    let line = *line_start;
    let column = *column_start;
    while let Some(c) = chars.peek() {
        *column_start += 1;
        if c.is_alphanumeric() || c == &'_' {
            value.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    let keywords = get_keywords();
    let token_type = match keywords.get(value.as_str()) {
        Some(token_type) => *token_type,
        None => TokenType::Id,
    };
    Token{token_type, value, line, column}
}

fn int_lit(chars: &mut Peekable<Chars>, line_start: &u32, column_start: &mut u32, first_char: char) -> Token {
    let mut value = first_char.to_string();
    let line = *line_start;
    let column = *column_start;
    while let Some(c) = chars.peek() {
        *column_start += 1;
        if c.is_numeric() {
            value.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    Token{token_type: TokenType::IntLit, value, line, column}
}