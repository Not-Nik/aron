// aron (c) Nikolas Wipper 2022

use crate::instructions::{Instruction, Opcode};
use crate::instructions::AddressingMode::Qword;
use crate::parse::{Line, ParseError};
use crate::parse::lexer::Lexer;

fn parse_directive(mut tokens: Vec<String>) -> Result<Line, ParseError> {
    tokens.remove(0);

    if tokens.is_empty() {
        Err(ParseError::UnexpectedLB)
    } else {
        Ok(Line::Directive(tokens))
    }
}

fn parse_label(mut tokens: Vec<String>) -> Result<Line, ParseError> {
    tokens.pop();

    if tokens.is_empty() {
        Err(ParseError::UnexpectedLB)
    } else if tokens.len() > 1 {
        Err(ParseError::ExtraneousTokenBeforeLabel)
    } else {
        Ok(Line::Label(tokens.first().unwrap().clone()))
    }
}

fn parse_instruction(tokens: Vec<String>) -> Result<Line, ParseError> {
    // todo: instruction prefix (rep, lock)
    let op = tokens.first().unwrap();

    let opcode = op.parse::<Opcode>();

    if opcode.is_err() {
        return Err(ParseError::InvalidInstruction);
    }
    let opcode = opcode.unwrap();

    // todo: parse the rest
    Ok(Line::Instruction(Instruction::new_ocl(opcode, Qword/*todo hardcoded*/)))
}

fn parse_line(tokens: Vec<String>) -> Result<Line, ParseError> {
    if tokens.first().unwrap() == "." {
        parse_directive(tokens)
    } else if tokens.last().unwrap() == ":" {
        parse_label(tokens)
    } else {
        parse_instruction(tokens)
    }
}

fn parse_lines(code: String) -> Vec<Line> {
    let mut lexer = Lexer::new(code);

    let mut res = Vec::new();

    'outer_parser: loop {
        let mut tokens = Vec::new();

        let mut token = lexer.read();
        'inner_parser: loop {
            if token.is_err() {
                break 'outer_parser;
            }

            let token_s = token.unwrap();
            if token_s == "\n" {
                break 'inner_parser;
            }
            tokens.push(token_s);
            token = lexer.read()
        }

        if !tokens.is_empty() {
            res.push(parse_line(tokens));
        }
    }

    todo!()
}
