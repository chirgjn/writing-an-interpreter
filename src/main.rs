mod token;
mod lexer;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use crate::lexer::Lexer;
    use crate::token;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let expected_output = [
            (token::ASSIGN, "="),
            (token::PLUS, "+"),
            (token::LPAREN, "("),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::RBRACE, "}"),
            (token::COMMA, ","),
            (token::SEMICOLON, ";"),
            (token::EOF, ""),
        ];

        let mut l = Lexer::new(input);

        for e in expected_output.iter() {
            let t = l.next_token();
            assert_eq!(t.token_type, e.0);
            assert_eq!(t.literal, e.1);
        }
    }
}
