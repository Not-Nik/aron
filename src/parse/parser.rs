// aron (c) Nikolas Wipper 2022

use crate::parse::encodings::matches;
use crate::parse::lexer::Lexer;
use crate::parse::{BuildVersion, Directive, Line, ParseError};

fn parse_directive(mut tokens: Vec<String>) -> Result<Line, ParseError> {
    tokens.remove(0);

    if tokens.is_empty() {
        Err(ParseError::UnexpectedLB)
    } else {
        let mut iter = tokens.into_iter();
        let first = iter.next();
        if first.is_none() {
            return Err(ParseError::UnexpectedLB);
        }

        match first.unwrap().as_str() {
            "globl" => {
                let name = iter.next();

                if let Some(name) = name {
                    Ok(Line::Directive(Directive::Global(name)))
                } else {
                    Err(ParseError::UnexpectedLB)
                }
            }
            "build_version" => {
                let os = iter.next();

                if let Some(os) = os {
                    match os.as_str() {
                        "macos" => {
                            if iter.next().ok_or(ParseError::UnexpectedLB)? != "," {
                                return Err(ParseError::InvalidDirective);
                            }

                            let major = iter.next().ok_or(ParseError::UnexpectedLB)?.parse::<u16>().map_err(|_| ParseError::InvalidDirective)?;

                            if iter.next().ok_or(ParseError::UnexpectedLB)? != "," {
                                return Err(ParseError::InvalidDirective);
                            }

                            let minor = iter.next().ok_or(ParseError::UnexpectedLB)?.parse::<u16>().map_err(|_| ParseError::InvalidDirective)?;

                            Ok(Line::Directive(Directive::BuildVersion(BuildVersion::MacOS { major, minor })))
                        }
                        // Todo: other operating systems
                        //  This is an easy fix; compile test.c on other OS's and look what
                        //  .build_version says there
                        _ => Ok(Line::Directive(Directive::BuildVersion(BuildVersion::Unknown(iter.collect())))),
                    }
                } else {
                    Err(ParseError::UnexpectedLB)
                }
            }
            // Todo: parse other important directives like section and alignment indicators
            _ => Ok(Line::Directive(Directive::Unknown(iter.collect()))),
        }
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
    Ok(Line::Instruction(matches(&tokens)?))
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

pub fn parse_lines(code: String) -> Vec<Line> {
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
            res.push(parse_line(tokens).unwrap());
        }
    }

    res
}
