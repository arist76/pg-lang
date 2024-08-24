
#[derive(Debug)]
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
    PRINT, RETURN, SUPER,
    THIS, TRUE, LET, WHILE,
    DO, EOF
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u64
}

impl Token {
    pub fn new(token_type: TokenType, line: u64) -> Token {
        Token { token_type, line }
    }
}
