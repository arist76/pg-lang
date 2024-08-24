use crate::token::{TokenType, Token};
use crate::exceptions::{BaseException, SyntaxException};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: u64,
    pub current: u64,
    pub line: u64,
    pub exceptions : Vec<SyntaxException> 
}

impl Scanner {
    pub fn new(source: String, line : u64) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line,
            exceptions: Vec::new()
        }
    }

    pub fn next(&mut self) -> Option<char> {
        // TODO: handle overflow on implicit conversion u64 -> usize
        let r = self.source.chars().nth(self.current as usize);
        self.current+=1;
        r
    }

    pub fn next_peek(&mut self) -> Option<char> {
        self.source.chars().nth(self.current as usize + 1)
    }

    pub fn scan(&mut self) -> Option<TokenType> {
        let option_c = self.next();
        
        let c = match option_c {
            Some(cc) => cc,
            None => return Some(TokenType::EOF)
        };

        match c {
            '(' => Some(TokenType::LEFT_PAREN),
            ')' => Some(TokenType::RIGHT_PAREN),
            '{' => Some(TokenType::LEFT_BRACE),
            '}' => Some(TokenType::RIGHT_BRACE),
            ';' => Some(TokenType::SEMI_COLON),
            ',' => Some(TokenType::COMMA),
            '.' => Some(TokenType::DOT),
            '-' => Some(TokenType::MINUS),
            '+' => Some(TokenType::PLUS),
            '/' => Some(TokenType::SLASH),
            '*' => Some(TokenType::STAR),
            '#' => {  // comment
                while let Some(c) = self.next_peek() {
                    if c == '\n' {
                        break;
                    }
                    self.next();
                }
                self.next();  // move one more
                None
            },
            '!' => {
                if let Some('=') = self.next_peek() {
                    self.next();  // skip the next character
                    Some(TokenType::BANG_EQUAL)
                } else  {
                    Some(TokenType::BANG)
                }
            },
            '=' => {
                if let Some('=') = self.next_peek() {
                    self.next();  // skip the next character
                    Some(TokenType::EQUAL_EQUAL)
                } else {
                    Some(TokenType::EQUAL)
                } 
            },
            '>' => {
                if let Some('=') = self.next_peek() {
                    self.next();  // slip the next character
                    Some(TokenType::GREATER_EQUAL)
                } else {
                    Some(TokenType::GREATER)
                }
            },
            '<' => {
                if let Some('=') = self.next_peek() {
                    self.next();
                    Some(TokenType::LESS_EQUAL)
                } else {
                    Some(TokenType::LESS)
                }
            },
            // pass whitespace
            ' ' | '\r' | '\t' => None,
            // append all other characters as exceptions
           '\0'..='\'' | ')'..='\u{d7ff}' | '\u{e000}'..='\u{10ffff}' => {
                self.exceptions.push(
                    SyntaxException::new(
                        format!("Unexpected character {}", c),
                        self.line.into(),
                        self.current
                    )
                );
                None
            }
        }
    }

    pub fn start(&mut self) -> () {
        loop {
            let token_type_option = self.scan();

            let token_type = match token_type_option {
                Some(t) => t,
                None => continue
            };

            match token_type {
                TokenType::EOF => break,
                _ => self.tokens.push(Token::new(token_type, self.line)),
            }
        }
    }
}


