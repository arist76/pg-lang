use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single characters
    LEFT_PAREN, RIGHT_PAREN,
    LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, 
    SEMI_COLON, SLASH, STAR,
    EQUAL, BANG, GREATER, LESS,
    POUND,
    // Double characters
    BANG_EQUAL,
    EQUAL_EQUAL,
    GREATER_EQUAL,
    LESS_EQUAL,
    // Literals
    IDENTIFIER(String), STRING(String),
    NUMBER(f64),
    // Keywords
    AND, ELSE, ELIF,
    FALSE, FN, FOR,
    IF, NONE, OR,
    RETURN, SUPER,
    THIS, TRUE, LET, WHILE,
    DO, EOF
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Token {
        Token { token_type, line }
    }
}

pub fn keywords() -> HashMap<&'static str, TokenType> {
    let mut kw = HashMap::new();
    kw.insert("and", TokenType::AND);
    kw.insert("else", TokenType::ELSE);
    kw.insert("elif", TokenType::ELIF);
    kw.insert("false", TokenType::FALSE);
    kw.insert("fn", TokenType::FN);
    kw.insert("for", TokenType::FOR);
    kw.insert("if", TokenType::IF);
    kw.insert("none", TokenType::NONE);
    kw.insert("or", TokenType::OR);
    kw.insert("return", TokenType::RETURN);
    kw.insert("super", TokenType::SUPER);
    kw.insert("this", TokenType::THIS);
    kw.insert("true", TokenType::TRUE);
    kw.insert("let", TokenType::LET);
    kw.insert("while", TokenType::WHILE);
    kw.insert("do", TokenType::DO);
    kw

}
