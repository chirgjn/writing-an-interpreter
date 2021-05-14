type TokenType<'a> = &'a str;

pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub literal: &'a str,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Operators
pub const ASSIGN: &str = "ASSIGN";
pub const PLUS: &str = "PLUS";

// Delimiters
pub const COMMA: &str = "COMMA";
pub const SEMICOLON: &str = "SEMICOLON";

pub const LPAREN: &str = "LPAREN";
pub const RPAREN: &str = "RPAREN";
pub const LBRACE: &str = "LBRACE";
pub const RBRACE: &str = "RBRACE";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";

impl<'a> Token<'_> {
    pub fn new(token_type: &'a str, literal: &'a str) -> Token<'a> {
        Token {
            token_type,
            literal,
        }
    }
}
