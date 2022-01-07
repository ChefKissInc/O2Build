use Oxygen::token::*;

#[test]
pub fn simple() {
    assert_eq!(
        tokenise("func simple() { a += b; }".to_string()),
        (
            vec![
                Token::Keyword(TokenPos { row: 1, column: 1 }, Keyword::Function),
                Token::Identifier(TokenPos { row: 1, column: 6 }, "simple".to_string()),
                Token::LeftParen(TokenPos { row: 1, column: 12 }),
                Token::RightParen(TokenPos { row: 1, column: 13 }),
                Token::LeftBracket(TokenPos { row: 1, column: 15 }),
                Token::Identifier(TokenPos { row: 1, column: 17 }, "a".to_string()),
                Token::PlusEquals(TokenPos { row: 1, column: 19 }),
                Token::Identifier(TokenPos { row: 1, column: 22 }, "b".to_string()),
                Token::Semicolon(TokenPos { row: 1, column: 23 }),
                Token::RightBracket(TokenPos { row: 1, column: 25 }),
            ],
            vec![]
        )
    );
}

#[test]
pub fn simple_with_digit() {
    assert_eq!(
        tokenise("func simpleWithDigit() -> QWord { 1 + 2 }".to_string()),
        (
            vec![
                Token::Keyword(TokenPos { row: 1, column: 1 }, Keyword::Function),
                Token::Identifier(
                    TokenPos { row: 1, column: 6 },
                    "simpleWithDigit".to_string()
                ),
                Token::LeftParen(TokenPos { row: 1, column: 21 }),
                Token::RightParen(TokenPos { row: 1, column: 22 }),
                Token::Arrow(TokenPos { row: 1, column: 24 }),
                Token::Identifier(TokenPos { row: 1, column: 27 }, "QWord".to_string()),
                Token::LeftBracket(TokenPos { row: 1, column: 33 }),
                Token::Integer(TokenPos { row: 1, column: 35 }, "1".to_string()),
                Token::Plus(TokenPos { row: 1, column: 37 }),
                Token::Integer(TokenPos { row: 1, column: 39 }, "2".to_string()),
                Token::RightBracket(TokenPos { row: 1, column: 41 }),
            ],
            vec![]
        )
    );
}

#[test]
pub fn simple_with_string() {
    assert_eq!(
        tokenise("func simpleWithString() -> Str { \"Hello \" + \"Oxygen\" }".to_string()),
        (
            vec![
                Token::Keyword(TokenPos { row: 1, column: 1 }, Keyword::Function),
                Token::Identifier(
                    TokenPos { row: 1, column: 6 },
                    "simpleWithString".to_string()
                ),
                Token::LeftParen(TokenPos { row: 1, column: 22 }),
                Token::RightParen(TokenPos { row: 1, column: 23 }),
                Token::Arrow(TokenPos { row: 1, column: 25 }),
                Token::Identifier(TokenPos { row: 1, column: 28 }, "Str".to_string()),
                Token::LeftBracket(TokenPos { row: 1, column: 32 }),
                Token::String(TokenPos { row: 1, column: 34 }, "Hello ".to_string()),
                Token::Plus(TokenPos { row: 1, column: 43 }),
                Token::String(TokenPos { row: 1, column: 45 }, "Oxygen".to_string()),
                Token::RightBracket(TokenPos { row: 1, column: 54 }),
            ],
            vec![]
        )
    );
}
