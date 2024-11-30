use regex::Regex;
use crate::lexer::TokenType::{Number, OpenParenthesis, Operator, ClosedParenthesis};

pub enum TokenType {
    Number,
    Operator,
    OpenParenthesis,
    ClosedParenthesis,
}

pub struct Token {
    pub content: String,
    pub _type: TokenType,
}
pub struct Lexer {
    operators: Vec<char>,
}


impl Lexer {
    pub fn new() -> Lexer {
        Lexer { operators: vec!['+', '-', '*', '/', '^'] }
    }

    fn create_token(&self, string: &str, token_type: TokenType) -> Token {
        Token { content: String::from(string), _type: token_type }
    }

    pub fn get_tokens(&self, string: &str) -> Vec<Token> {
        let mut tokens = vec![];

        let decimal_number_regex = Regex::new(r"^(0|[1-9]\d*)?(\.\d+)?").unwrap();

        let mut idx: usize = 0;
        while idx < string.len() {
            let current_option = string.chars().nth(idx);
            let current = match current_option {
                Some(current_option) => current_option,
                _ => panic!()
            };

            if current.is_digit(10) {
                let match_delimiters = decimal_number_regex.find(&string[idx..]);
                let extracted_number = match match_delimiters {
                    Some(match_delimiters) => {
                        let start = idx;
                        let end = idx + match_delimiters.end();
                        idx = end - 1;
                        &string[start..end]
                    }
                    _ => panic!("No match!")
                };
                tokens.push(Token { content: String::from(extracted_number), _type: Number })
            } else {
                if self.operators.contains(&current) {
                    tokens.push(Token { content: String::from(current), _type: Operator })
                } else if current == '(' {
                    tokens.push(Token { content: String::from(current), _type: OpenParenthesis })
                } else if current == ')' {
                    tokens.push(Token { content: String::from(current), _type: ClosedParenthesis })
                }
            }
            idx = idx + 1;
        }

        tokens
    }
}
