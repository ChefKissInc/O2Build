use MolecularRearranger::{token::*, tokeniser::Tokeniser};

#[test]
pub fn simple() {
    assert_eq!(
        "Func simple() { a += b; }".to_string().tokenise(),
        (
            vec![
                Token::Keyword(TokenPos { row: 0, column: 1 }, Keyword::Function),
                Token::Identifier(TokenPos { row: 0, column: 6 }, "simple".to_string()),
                Token::LeftParen(TokenPos { row: 0, column: 12 }),
                Token::RightParen(TokenPos { row: 0, column: 13 }),
                Token::LeftBracket(TokenPos { row: 0, column: 15 }),
                Token::Identifier(TokenPos { row: 0, column: 17 }, "a".to_string()),
                Token::PlusEquals(TokenPos { row: 0, column: 19 }),
                Token::Identifier(TokenPos { row: 0, column: 22 }, "b".to_string()),
                Token::Semicolon(TokenPos { row: 0, column: 23 }),
                Token::RightBracket(TokenPos { row: 0, column: 25 }),
            ],
            vec![]
        )
    );
}

#[test]
pub fn simple_with_digit() {
    assert_eq!(
        "Func simpleWithDigit() -> QWord { 1 + 2 }"
            .to_string()
            .tokenise(),
        (
            vec![
                Token::Keyword(TokenPos { row: 0, column: 1 }, Keyword::Function),
                Token::Identifier(
                    TokenPos { row: 0, column: 6 },
                    "simpleWithDigit".to_string()
                ),
                Token::LeftParen(TokenPos { row: 0, column: 21 }),
                Token::RightParen(TokenPos { row: 0, column: 22 }),
                Token::Arrow(TokenPos { row: 0, column: 24 }),
                Token::Identifier(TokenPos { row: 0, column: 27 }, "QWord".to_string()),
                Token::LeftBracket(TokenPos { row: 0, column: 33 }),
                Token::Integer(TokenPos { row: 0, column: 35 }, "1".to_string()),
                Token::Plus(TokenPos { row: 0, column: 37 }),
                Token::Integer(TokenPos { row: 0, column: 39 }, "2".to_string()),
                Token::RightBracket(TokenPos { row: 0, column: 41 }),
            ],
            vec![]
        )
    );
}

#[test]
pub fn simple_with_string() {
    assert_eq!(
        "Func simpleWithString() -> String { \"Hello \" + \"Oxygen\" }"
            .to_string()
            .tokenise(),
        (
            vec![
                Token::Keyword(TokenPos { row: 0, column: 1 }, Keyword::Function),
                Token::Identifier(
                    TokenPos { row: 0, column: 6 },
                    "simpleWithString".to_string()
                ),
                Token::LeftParen(TokenPos { row: 0, column: 22 }),
                Token::RightParen(TokenPos { row: 0, column: 23 }),
                Token::Arrow(TokenPos { row: 0, column: 25 }),
                Token::Identifier(TokenPos { row: 0, column: 28 }, "String".to_string()),
                Token::LeftBracket(TokenPos { row: 0, column: 35 }),
                Token::String(TokenPos { row: 0, column: 37 }, "Hello ".to_string()),
                Token::Plus(TokenPos { row: 0, column: 46 }),
                Token::String(TokenPos { row: 0, column: 48 }, "Oxygen".to_string()),
                Token::RightBracket(TokenPos { row: 0, column: 57 }),
            ],
            vec![]
        )
    );
}
