// aron-parse (c) Nikolas Wipper 2022

mod lexer;
pub mod parser;
mod tests;
mod encodings;
mod helpers;

use crate::instructions::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedLB,
    InvalidInstruction,
    InvalidOperand,
    InvalidDirective,
    ExtraneousTokenBeforeLabel,
    ExtraneousTokenAfterInstruction,
}

#[derive(Debug)]
pub enum Line {
    Directive(Directive),
    Label(String),
    Instruction(Instruction),
}

#[derive(Debug)]
pub enum Directive {
    Global(String),
    BuildVersion(BuildVersion),
    Unknown(Vec<String>)
}

#[derive(Debug)]
pub enum BuildVersion {
    MacOS{major: u16, minor: u16},
    Unknown(Vec<String>)
}
