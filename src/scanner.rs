use crate::exceptions::{BaseException, SyntaxException};
use crate::token::{keywords, Token, TokenType};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub exceptions: Vec<SyntaxException>,
}

impl Scanner {
    pub fn new(source: String, line: usize) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line,
            exceptions: Vec::new(),
        }
    }

    pub fn next(&mut self) -> Option<char> {
        // TODO: handle overflow on implicit conversion u64 -> usize
        let r = self.source.chars().nth(self.current);
        self.current += 1;
        r
    }

    pub fn next_peek(&mut self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    pub fn scan(&mut self) -> Option<TokenType> {
        let option_c = self.next();

        let c = match option_c {
            Some(cc) => cc,
            None => return Some(TokenType::EOF),
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
            '#' => self.scan_comment(), // comment
            '!' => self.scan_double_character('=', TokenType::BANG, TokenType::BANG_EQUAL),
            '=' => self.scan_double_character('=', TokenType::EQUAL, TokenType::EQUAL_EQUAL),
            '>' => self.scan_double_character('=', TokenType::GREATER, TokenType::GREATER_EQUAL),
            '<' => self.scan_double_character('=', TokenType::LESS, TokenType::LESS_EQUAL),
            '"' => self.scan_string(),
            '0'..='9' => self.scan_number(), // floating number
            // pass whitespace
            ' ' | '\r' | '\t' => None,
            // append all other characters as exceptions
            _ => self.scan_symbol(),
        }
    }

    pub fn scan_string(&mut self) -> Option<TokenType> {
        let unterminated_string_exception =
            SyntaxException::new("Unterminated string".to_string(), self.line, self.current);
        let mut scanned_string = String::new();
        let mut s = match self.next() {
            Some(c) => c,
            None => {
                self.exceptions.push(unterminated_string_exception);
                return None;
            }
        };

        while '"' != s {
            if '\n' == s {
                self.exceptions.push(unterminated_string_exception);
                break;
            } else if s == '"' {
                return Some(TokenType::STRING(scanned_string));
            } else {
                scanned_string.push(s);
            };

            s = match self.next() {
                Some(c) => c,
                None => {
                    self.exceptions.push(unterminated_string_exception);
                    return None;
                }
            };
        }
        Some(TokenType::STRING(scanned_string))
    }

    pub fn scan_comment(&mut self) -> Option<TokenType> {
        while let Some(c) = self.next_peek() {
            if c == '\n' {
                break;
            }
            self.next();
        }
        self.next(); // move one more
        None
    }

    pub fn scan_double_character(
        &mut self,
        next_char: char,
        initial_token: TokenType,
        double_token: TokenType,
    ) -> Option<TokenType> {
        if Some(next_char) == self.next_peek() {
            self.next(); // skip the next character
            Some(double_token)
        } else {
            Some(initial_token)
        }
    }

    pub fn scan_number(&mut self) -> Option<TokenType> {
        let mut number_str = String::new();
        let mut c = self.source.chars().nth(self.current - 1).unwrap(); // unwrap-is-safe
                                                                        // and is always a number
        while c.is_ascii_digit() || c == '.' {
            number_str.push(c);
            c = match self.next_peek() {
                Some(c) => c,
                None => break,
            };

            //if !c.is_ascii_digit() && c != '.' {
            //    if !c.is_whitespace() {
            //        self.exceptions.push(SyntaxException::new(
            //            format!("Unexpected character {}", c),
            //            self.line,
            //            self.current,
            //        ));
            //    }
            //    break;
            //}
            self.next();
        }
        Some(TokenType::NUMBER(number_str.parse().unwrap()))
    }

    pub fn scan_symbol(&mut self) -> Option<TokenType> {
        let kw = keywords();
        let mut symbol_str = String::new();
        let mut c = match self.source.chars().nth(self.current - 1) {
            Some(cc) => cc,
            None => return None,
        };

        if !c.is_alphabetic() && c != '_' {  // variables/keywords only start with alphabets and _
            self.exceptions.push(SyntaxException::new(
                format!("Unexpected character {}", c),
                self.line,
                self.current,
            ));
        }

        while c.is_alphanumeric() || c == '_' {
            // finish scanning the symbol if the character is a whitespace
            symbol_str.push(c);

            c = match self.next() {
                Some(cc) => cc,
                None => break,
            };
        }

        match kw.get(symbol_str.as_str()) {
            Some(token_type) => Some(token_type.clone()),
            None => Some(TokenType::IDENTIFIER(symbol_str)),
        }
    }

    pub fn start(&mut self) -> () {
        loop {
            let token_type_option = self.scan();

            let token_type = match token_type_option {
                Some(t) => t,
                None => continue,
            };

            match token_type {
                TokenType::EOF => break,
                _ => self.tokens.push(Token::new(token_type, self.line)),
            }
        }
    }
}
