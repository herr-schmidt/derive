pub enum TokenType {
    NUMBER
}

pub struct Token {
    pub content: String,
    pub _type: TokenType,
}
pub struct Lexer {}


impl Lexer {
    pub fn new() -> Lexer {
        Lexer {}
    }

    fn create_token(&self, string: &str, token_type: TokenType) -> Token {
        Token { content: String::from(string), _type: token_type }
    }

    pub fn get_tokens(&self, string: &str) -> Vec<Token> {
        let mut tokens = vec![];

        let mut scan = 0;
        let mut token_start = 0;
        let mut state = 0;
        for char in string.chars() {
            match (state, char) {
                (0, '.') => { state = 1; }
                (0, '0'..='9') => { state = 2 }
                (1, '0'..='9') => { state = 3 }
                (2, '0'..='9') => { state = 2 }
                (2, '.') => { state = 4 }
                (2, _) => { state = 5 }
                (3, '0'..='9') => { state = 3 }
                (3, _) => { state = 5 }
                (4, '.') => { state = 5 }
                (4, _) => { state = 5 }
                (5, _) => { tokens.push(self.create_token(&string[token_start..scan], TokenType::NUMBER)) }
                _ => { panic!("Unexpected input!") }
            }
            scan = scan + 1;
        }

        tokens
    }
}
