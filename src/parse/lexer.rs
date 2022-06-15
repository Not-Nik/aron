// aron (c) Nikolas Wipper 2022

use std::ops::Range;
use std::str::FromStr;
use crate::parse::ParseError;

#[derive(Copy, Clone, Debug)]
pub struct LexPosition {
    pub line: usize,
    pub char: usize,
    pub pos: usize
}

impl LexPosition {
    pub fn new() -> LexPosition {
        LexPosition { line: 1, char: 0, pos: 0 }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    raw: String,
    pos: LexPosition
}

pub(crate) const OPERATORS: &[&str] = &[":", ",", "+", "-", "[", "]"];

pub struct Lexer {
    code: String,
    c: Result<char, ParseError>,
    pos: LexPosition,
}

fn starts_operator(c: &str) -> bool {
    for op in OPERATORS {
        if op.starts_with(c) {
            return true;
        }
    }
    false
}

impl Token {
    pub fn new(s: &str) -> Self {
        Token {
            raw: s.to_string(),
            pos: LexPosition { line: 0, char: 0, pos: 0 }
        }
    }

    pub fn as_str(&self) -> &str {
        self.raw.as_str()
    }

    #[inline]
    pub fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
        FromStr::from_str(self.raw.as_str())
    }

    pub fn clone_string(&self) -> String {
        self.raw.clone()
    }

    pub fn get_pos(&self) -> &LexPosition {
        &self.pos
    }

    pub fn get_range(&self) -> Range<usize> {
        (self.pos.pos - self.raw.len())..self.pos.pos
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        self.raw
    }
}

impl PartialEq<String> for Token {
    fn eq(&self, other: &String) -> bool {
        self.raw.eq(other)
    }

    fn ne(&self, other: &String) -> bool {
        self.raw.ne(other)
    }
}

impl PartialEq<str> for Token {
    fn eq(&self, other: &str) -> bool {
        self.raw.eq(other)
    }

    fn ne(&self, other: &str) -> bool {
        self.raw.ne(other)
    }
}

impl PartialEq<&str> for Token {
    fn eq(&self, other: &&str) -> bool {
        self.raw.eq(other)
    }

    fn ne(&self, other: &&str) -> bool {
        self.raw.ne(other)
    }
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        let char = code.chars().next();
        let c = if char.is_none() {
            Err(ParseError::UnexpectedEOF)
        } else {
            Ok(char.unwrap())
        };
        Lexer { code, c, pos: LexPosition::new() }
    }

    fn read_char(&mut self) -> &Result<char, ParseError> {
        self.c = self.peek_char();
        if self.c.is_ok() {
            self.pos.pos += 1;
            if self.c.as_ref().unwrap() == &'\n' {
                self.pos.line += 1;
                self.pos.char = 0;
            } else {
                self.pos.char += 1;
            }
        }
        &self.c
    }

    fn peek_char(&mut self) -> Result<char, ParseError> {
        if self.pos.pos < self.code.len() {
            let char = self.code[self.pos.pos+1..].chars().next();
            if char.is_none() {
                Err(ParseError::UnexpectedEOF)
            } else {
                Ok(char.unwrap())
            }
        } else {
            Err(ParseError::UnexpectedEOF)
        }
    }

    fn get_last_char(&self) -> Result<&char, &ParseError> {
        self.c.as_ref()
    }

    fn get_char_or(&self, or: char) -> char {
        *self.get_last_char().unwrap_or(&or)
    }

    fn get_char(&self) -> char {
        *self.get_last_char().unwrap()
    }

    pub fn read(&mut self) -> Result<Token, ParseError> {
        let mut token = String::new();
        const STOPPERS: &str = " \t\n#";

        let skippable = |c: char| -> bool { (c.is_whitespace() && c != '\n') || c == '\t' || c == '#' };

        while skippable(self.get_char_or('\0')) {
            if self.get_char_or('\0') == '#' {
                self.read_line();
            } else {
                self.read_char();
            }
        }

        if self.get_char_or('\0') == '\0' {
            return Err(ParseError::UnexpectedLB);
        } else if self.get_char() == '"' || self.get_char() == '\'' {
            token.push('"');
            loop {
                self.read_char();
                if self.get_char_or('\0') == '\0' || self.get_char_or('\0') == '\n' {
                    break;
                }
                token.push(self.get_char_or('\0'));
                if token.chars().last().unwrap() == '\'' {
                    token.pop();
                    token.push('"');
                }
                if token.chars().last().unwrap() == '"' {
                    self.read_char();
                    break;
                }
            }
        } else {
            let mut op = false;
            loop {
                let char = self.get_char();
                if STOPPERS.contains(char) {
                    if token.is_empty() {
                        token = char.to_string();
                        self.read_char();
                    }
                    break;
                } else if starts_operator(char.to_string().as_str()) {
                    if token.is_empty() {
                        token = char.to_string();
                        op = true;
                    } else if op {
                        token.push(char);
                        if !starts_operator(token.as_str()) {
                            token.pop();
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    if op {
                        break;
                    }
                    token.push(char);
                }
                if self.read_char().is_err() {
                    break;
                }
            }
        }

        Ok(Token {
            raw: token,
            pos: self.pos
        })
    }

    fn read_line_until(&mut self, chars: Vec<char>) -> (String, bool, char) {
        let mut res = String::with_capacity(10);
        let mut read_c = false;
        let mut ch = '\0';
        'read_loop: while self.get_char_or('\0') != '\0' {
            let peek = self.peek_char();
            if peek.is_ok() && peek.unwrap() == '#' {
                break;
            }
            for c in &chars {
                if self.get_char() == *c {
                    read_c = true;
                    ch = *c;
                    break 'read_loop;
                }
            }
            if self.get_char() == '\\' {
                let peek = self.peek_char();
                if peek.is_ok() && peek.unwrap() == '\n' {
                    // Ignore line break
                    self.read().unwrap();
                }
            } else {
                if self.peek_char().unwrap_or('\0') == '\n' {
                    break;
                }
                res.push(self.get_char());
            }
            if self.read_char().is_err() {
                break;
            }
        }
        // Clear c
        self.read_char();
        res.shrink_to_fit();

        (res, read_c, ch)
    }

    fn read_line(&mut self) -> String {
        self.read_line_until(Vec::new()).0
    }
}
