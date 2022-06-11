// aron-parse (c) Nikolas Wipper 2022

mod parser;
mod lexer;
mod tests;

use crate::instructions::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedLB,
    InvalidInstruction,
    InvalidOperand,
    ExtraneousTokenBeforeLabel,
}

pub enum Line {
    Directive(Vec<String>),
    Label(String),
    Instruction(Instruction)
}
