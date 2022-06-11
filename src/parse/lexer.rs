// aron (c) Nikolas Wipper 2022

use crate::parse::ParseError;

#[derive(Copy, Clone)]
pub struct LexPosition {
    pub line: u32,
    pub char: u32,
}

impl LexPosition {
    pub fn new() -> LexPosition {
        LexPosition { line: 1, char: 0 }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Operator,
    LineBreak,
    String,
    Name,
}

pub(crate) const OPERATORS: &[&str] = &[".", ":", ",", "[", "]"];

fn starts_operator(c: &str) -> bool {
    for op in OPERATORS {
        if op.starts_with(c) {
            return true;
        }
    }
    false
}

impl TokenType {
    pub fn from_token(token: &String) -> TokenType {
        if token.starts_with('"') {
            return TokenType::String;
        }
        for op in OPERATORS {
            if *op == token.as_str() {
                return TokenType::Operator;
            }
        }

        // Todo: parse number
        match token.as_str() {
            "\n" => TokenType::LineBreak,
            _ => TokenType::Name,
        }
    }
}

pub struct Lexer {
    code: String,
    c: Result<char, ParseError>,
    pub pos: usize,
    pretty_pos: LexPosition,
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer { code, c: Ok(' '), pos: 0, pretty_pos: LexPosition::new() }
    }

    fn read_char(&mut self) -> &Result<char, ParseError> {
        self.c = self.peek_char();
        if self.c.is_ok() {
            self.pos += 1;
            if self.c.as_ref().unwrap() == &'\n' {
                self.pretty_pos.line += 1;
                self.pretty_pos.char = 0;
            } else {
                self.pretty_pos.char += 1;
            }
        }
        &self.c
    }

    fn peek_char(&mut self) -> Result<char, ParseError> {
        if self.pos < self.code.len() {
            let char = self.code[self.pos..].chars().next();
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

    pub fn read(&mut self) -> Result<String, ParseError> {
        let mut token = String::new();
        const STOPPERS: &str = " \n#";

        let skippable = |c: char| -> bool { c.is_whitespace() && c != '\n' || c == '#' };

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
                    if char == '$' && self.peek_char().is_ok() && self.peek_char().unwrap() == '(' {
                        self.read_char();
                        self.read_char();
                        token = String::from("$(");
                        let name = self.read();
                        if name.is_err() {
                            break;
                        }
                        token.push_str(name.unwrap().as_str());
                        if self.get_char_or('\0') == ')' {
                            token.push(')');
                        }
                        break;
                    }

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

        Ok(token)
    }

    fn read_line_until(&mut self, chars: Vec<char>) -> (String, bool, char) {
        let mut res = String::with_capacity(10);
        let mut read_c = false;
        let mut ch = '\0';
        'readLoop: while self.get_char_or('\0') != '\0' {
            let peek = self.peek_char();
            if peek.is_ok() && peek.unwrap() == '#' {
                break;
            }
            for c in &chars {
                if self.get_char() == *c {
                    read_c = true;
                    ch = *c;
                    break 'readLoop;
                }
            }
            if self.get_char() == '\\' {
                let peek = self.peek_char();
                if peek.is_ok() && peek.unwrap() == '\n' {
                    // Ignore line break
                    self.read().unwrap();
                }
            } else {
                if self.peek_char() == Ok('\n') {
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
