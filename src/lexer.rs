use crate::token;
use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<&'a str>,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(&self.input[self.read_position..self.read_position + 1]);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let t: Token;

        if let Some(ch) = self.ch {
            t = match ch {
                "=" => Token::new(token::ASSIGN, ch),
                ";" => Token::new(token::SEMICOLON, ch),
                "(" => Token::new(token::LPAREN, ch),
                ")" => Token::new(token::RPAREN, ch),
                "," => Token::new(token::COMMA, ch),
                "+" => Token::new(token::PLUS, ch),
                "{" => Token::new(token::LBRACE, ch),
                "}" => Token::new(token::RBRACE, ch),
                _ => Token::new(token::EOF, ""),
            }
        } else {
            t = Token::new(token::EOF, "");
        }
        self.read_char();
        t
    }
}

