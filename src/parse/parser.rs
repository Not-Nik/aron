// aron (c) Nikolas Wipper 2022

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::parse::encodings::matches;
use crate::parse::helpers::get_next;
use crate::parse::lexer::{Lexer, Token};
use crate::parse::{BuildVersion, Directive, Line, ParseError};
use ariadne::{Label, Report, ReportKind, Source};

fn sanitize_string(mut string: String) -> String {
    string = string.replace("\\a", "\x07");
    string = string.replace("\\b", "\x08");
    string = string.replace("\\t", "\x09");
    string = string.replace("\\n", "\x0A");
    string = string.replace("\\v", "\x0B");
    string = string.replace("\\f", "\x0C");
    string = string.replace("\\r", "\x0D");
    string = string.replace("\\e", "\x1B");
    string = string.replace("\\?", "\x3F");

    string
}

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
            ".asciz" => {
                let string = get_next(&mut iter)?.clone_string();

                Ok(Line::Directive(Directive::Asciz(sanitize_string(string))))
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
            ".globl" => Ok(Line::Directive(Directive::Global(get_next(&mut iter)?.clone_string()))),
            ".section" => {
                let segment = get_next(&mut iter)?.clone_string();

                let next = iter.next();
                if let Some(next) = next {
                    if next != "," {
                        return Err((iter.count(), ParseError::InvalidDirective));
                    }

                    let section = get_next(&mut iter)?.clone_string();

                    Ok(Line::Directive(Directive::Section(format!("{},{}", segment, section))))
                } else {
                    Ok(Line::Directive(Directive::Section(segment)))
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
