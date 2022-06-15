// aron (c) Nikolas Wipper 2022

use crate::parse::encodings::matches;
use crate::parse::helpers::get_next;
use crate::parse::lexer::{Lexer, Token};
use crate::parse::{BuildVersion, Directive, Line, ParseError};
use ariadne::{Label, Report, ReportKind, Source};

fn parse_directive(tokens: &Vec<Token>) -> Result<Line, (usize, ParseError)> {
    if tokens.is_empty() {
        Err((0, ParseError::UnexpectedLB))
    } else {
        let mut iter = tokens.iter();
        let first = iter.next();
        if first.is_none() {
            return Err((iter.count(), ParseError::UnexpectedLB));
        }

        match first.unwrap().as_str() {
            ".globl" => {
                let name = iter.next();

                if let Some(name) = name {
                    Ok(Line::Directive(Directive::Global(name.clone_string())))
                } else {
                    Err((iter.count(), ParseError::UnexpectedLB))
                }
            }
            ".build_version" => {
                let os = iter.next();

                if let Some(os) = os {
                    match os.as_str() {
                        "macos" => {
                            if get_next(&mut iter)? != "," {
                                return Err((iter.count(), ParseError::InvalidDirective));
                            }

                            let major = get_next(&mut iter)?
                                .parse::<u16>()
                                .map_err(|_| (iter.clone().count(), ParseError::InvalidDirective))?;

                            if get_next(&mut iter)? != "," {
                                return Err((iter.count(), ParseError::InvalidDirective));
                            }

                            let minor = get_next(&mut iter)?
                                .parse::<u16>()
                                .map_err(|_| (iter.clone().count(), ParseError::InvalidDirective))?;

                            Ok(Line::Directive(Directive::BuildVersion(BuildVersion::MacOS { major, minor })))
                        }
                        // Todo: other operating systems
                        //  This is an easy fix; compile test.c on other OS's and look what
                        //  .build_version says there
                        _ => Ok(Line::Directive(Directive::BuildVersion(BuildVersion::Unknown))),
                    }
                } else {
                    Err((iter.count(), ParseError::UnexpectedLB))
                }
            }
            // Todo: parse other important directives like section and alignment indicators
            _ => Ok(Line::Directive(Directive::Unknown)),
        }
    }
}

fn parse_label(tokens: &Vec<Token>) -> Result<Line, (usize, ParseError)> {
    if tokens.len() == 1 {
        Err((0, ParseError::UnexpectedLB))
    } else {
        let label = tokens.get(tokens.len() - 2).unwrap().clone();
        if tokens.len() > 2 {
            Err((tokens.len() - 1, ParseError::ExtraneousTokenBeforeLabel(label)))
        } else {
            Ok(Line::Label(tokens.first().unwrap().clone_string()))
        }
    }
}

fn parse_instruction(tokens: &Vec<Token>) -> Result<Line, (usize, ParseError)> {
    // todo: instruction prefix (rep, lock)
    Ok(Line::Instruction(matches(tokens)?))
}

fn parse_line(tokens: &Vec<Token>) -> Result<Line, (usize, ParseError)> {
    if tokens.first().unwrap().as_str().starts_with('.') {
        parse_directive(tokens)
    } else if tokens.last().unwrap() == ":" {
        parse_label(tokens)
    } else {
        parse_instruction(tokens)
    }
}

pub fn parse_lines(file_name: String, code: String) -> Result<Vec<Line>, ()> {
    let mut lexer = Lexer::new(code.clone());

    let mut vec = Vec::new();
    let mut is_ok = true;

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
            let res = parse_line(&tokens);

            if let Ok(res) = res {
                vec.push(res);
            } else if let Err((i, e)) = res {
                let tok = tokens.remove(tokens.len() - i - 1);

                let mut builder = Report::build(ReportKind::Error, file_name.clone(), tok.get_pos().pos)
                    .with_code(e.to_code())
                    .with_message(e.to_string())
                    .with_label(
                        Label::new((file_name.clone(), tok.get_range()))
                            .with_message(format!("'{}' here", tok.as_str())),
                    );

                match e {
                    ParseError::ExtraneousTokenBeforeLabel(label) => {
                        builder = builder.with_label(
                            Label::new((file_name.clone(), label.get_range()))
                                .with_message(format!("To label '{}' here", label.as_str())),
                        );
                    }
                    _ => {}
                }

                builder.finish().eprint((file_name.clone(), Source::from(code.clone()))).unwrap();

                is_ok = false;
            }
        }
    }

    if is_ok {
        Ok(vec)
    } else {
        Err(())
    }
}
