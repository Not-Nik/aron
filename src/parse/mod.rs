// aron-parse (c) Nikolas Wipper 2022

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod encodings;
pub mod helpers;
mod lexer;
pub mod parser;
mod tests;

use crate::instructions::Instruction;
use crate::parse::lexer::Token;

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedLB,
    InvalidInstruction,
    InvalidOperand,
    InvalidDirective,
    ExtraneousTokenBeforeLabel(Token),
    ExtraneousTokenAfterInstruction,
}

impl ParseError {
    pub fn to_code(&self) -> usize {
        match self {
            ParseError::UnexpectedEOF => 1,
            ParseError::UnexpectedLB => 2,
            ParseError::InvalidInstruction => 3,
            ParseError::InvalidOperand => 4,
            ParseError::InvalidDirective => 5,
            ParseError::ExtraneousTokenBeforeLabel(_) => 6,
            ParseError::ExtraneousTokenAfterInstruction => 7,
        }
    }
}

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match self {
            ParseError::UnexpectedEOF => "Unexpected end of file".to_string(),
            ParseError::UnexpectedLB => "Unexpected end of line".to_string(),
            ParseError::InvalidInstruction => "Invalid instruction".to_string(),
            ParseError::InvalidOperand => "Invalid operand".to_string(),
            ParseError::InvalidDirective => "Invalid directive".to_string(),
            ParseError::ExtraneousTokenBeforeLabel(_) => "Extraneous token before label".to_string(),
            ParseError::ExtraneousTokenAfterInstruction => "Extraneous token after instruction".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Line {
    Directive(Directive),
    Label(String),
    Instruction(Instruction),
}

#[derive(Debug)]
pub enum Directive {
    Asciz(String),
    BuildVersion(BuildVersion),
    Global(String),
    Section(String),
    Unknown,
}

#[derive(Debug)]
pub enum BuildVersion {
    MacOS { major: u16, minor: u16 },
    Unknown,
}
