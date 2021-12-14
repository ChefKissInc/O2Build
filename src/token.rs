#[derive(Debug, PartialEq)]
pub enum TokenisationError {
    UnexpectedEndOfFile,
    UnknownToken(char),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Public,   // public
    Extern,   // extern
    Function, // func
    Variable, // let
    Mutable,  // mutable
    Abi,      // abi
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct TokenPos {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(TokenPos, Keyword),
    Identifier(TokenPos, String),
    Integer(TokenPos, String),
    String(TokenPos, String),
    Char(TokenPos, char),
    LeftParen(TokenPos),
    RightParen(TokenPos),
    Colon(TokenPos),
    Comma(TokenPos),
    Minus(TokenPos),
    Arrow(TokenPos),
    FatArrow(TokenPos),
    MinusEquals(TokenPos),
    Plus(TokenPos),
    PlusEquals(TokenPos),
    LeftBracket(TokenPos),
    RightBracket(TokenPos),
    Times(TokenPos),
    Divide(TokenPos),
    Eq(TokenPos),
    EqEq(TokenPos),
    Less(TokenPos),
    LessOrEqual(TokenPos),
    Greater(TokenPos),
    GreaterOrEqual(TokenPos),
    Semicolon(TokenPos),
}
