pub mod token;

use token::{Symbol, Token};
use toyjs_common::SourceLoc;

#[derive(Clone, Debug, PartialEq)]
pub enum LexerError {
    ExpectedEOF,
    UnexpectedEOF(String),
}

#[derive(Clone, Debug)]
pub struct Lexer {
    code: String,
    start: SourceLoc,
    current: SourceLoc,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            code,
            start: SourceLoc::default(),
            current: SourceLoc::default(),
        }
    }

    pub fn next(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        self.start = self.current;
        if self.is_at_end() {
            return Err(LexerError::ExpectedEOF);
        }

        match self.advance() {
            b'(' => self.make_symbol(Symbol::OpeningParen),
            b')' => self.make_symbol(Symbol::ClosingParen),
            b'{' => self.make_symbol(Symbol::OpeningBrace),
            b'}' => self.make_symbol(Symbol::ClosingBrace),
            b';' => self.make_symbol(Symbol::Semicolon),
            b',' => self.make_symbol(Symbol::Comma),
            b'.' => self.make_symbol(Symbol::Point),
            b'-' => self.make_symbol(Symbol::Sub),
            b'+' => self.make_symbol(Symbol::Add),
            b'/' => self.make_symbol(Symbol::Div),
            b'*' => self.make_symbol(Symbol::Asterisk),
            b'!' if self.matches(b'=') => {
                if self.matches(b'=') {
                    self.make_symbol(Symbol::SNe)
                } else {
                    self.make_symbol(Symbol::Ne)
                }
            }
            b'!' => self.make_symbol(Symbol::Not),
            b'=' if self.matches(b'=') => {
                if self.matches(b'=') {
                    self.make_symbol(Symbol::SEq)
                } else {
                    self.make_symbol(Symbol::Eq)
                }
            }
            b'=' => self.make_symbol(Symbol::Assign),
            b'<' if self.matches(b'=') => self.make_symbol(Symbol::Le),
            b'<' => self.make_symbol(Symbol::Lt),
            b'>' if self.matches(b'=') => self.make_symbol(Symbol::Ge),
            b'>' => self.make_symbol(Symbol::Gt),
            b'"' => self.string(),
            _ => todo!(),
        }
    }

    fn string(&mut self) -> Result<Token, LexerError> {
        while !self.is_at_end() && self.peek() != b'"' {
            if self.peek() == b'\n' {
                // TODO: increment line number?
            }
            self.advance();
        }

        if self.is_at_end() {
            Err(LexerError::UnexpectedEOF("Unterminated string".to_string()))
        } else {
            Ok(Token::new_string(self.lexeme(), self.start))
        }
    }

    fn make_symbol(&self, sym: Symbol) -> Result<Token, LexerError> {
        Ok(Token::new_symbol(sym, self.start))
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }
                b'\n' => {
                    // TODO: Update line number here if we need it?
                    self.advance();
                }
                b'/' if self.peek_next() == b'/' => {
                    // Do we also want to emit comments?
                    while !self.is_at_end() && self.peek() != b'\n' {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn advance(&mut self) -> u8 {
        let c = self.peek();
        // TODO: Also update line and column values.
        self.current.pos += 1;
        c
    }

    fn matches(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        } else {
            self.advance();
            true
        }
    }

    fn lexeme(&self) -> String {
        self.code[self.start.pos..self.current.pos].to_string()
    }

    fn peek_next(&self) -> u8 {
        if self.current.pos + 1 < self.code.len() {
            return self.char_at(self.current.pos + 1);
        } else {
            return b'\0';
        }
    }

    fn peek(&self) -> u8 {
        self.char_at(self.current.pos)
    }

    fn char_at(&self, index: usize) -> u8 {
        self.code.as_bytes()[index]
    }

    fn is_at_end(&self) -> bool {
        self.current.pos == self.code.len()
    }
}
