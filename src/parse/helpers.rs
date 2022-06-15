// aron (c) Nikolas Wipper 2022

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::instructions::{Mod, Register};
use crate::parse::lexer::Token;
use crate::parse::ParseError;
use std::mem::size_of;
use std::slice::Iter;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Relativity {
    Absolute,
    Relative,
    RipRelative,
}

pub enum ImmediateType {
    Integer(i32),
    Reference(String),
}

pub struct Immediate {
    pub rel: Relativity,
    pub typ: ImmediateType,
}

use ImmediateType::*;
use Relativity::*;
use crate::instructions::Register::Rip;

impl Immediate {
    pub fn integer(int: i32, rel: Relativity) -> Immediate {
        Immediate { rel, typ: Integer(int) }
    }

    pub fn reference(to: String, rel: Relativity) -> Immediate {
        Immediate { rel, typ: Reference(to) }
    }
}

pub fn get_next<'a>(iter: &'a mut Iter<Token>) -> Result<&'a Token, (usize, ParseError)> {
    let next = iter.next();
    if next.is_none() {
        return Err((iter.count(), ParseError::UnexpectedLB));
    }
    Ok(next.unwrap())
}

pub fn get_mod_from_rm(rm: &(Register, Mod, Option<Immediate>)) -> Mod {
    if let Some(off) = &rm.2 {
        if off.rel == RipRelative {
            Mod::NoOffset
        } else {
            match off.typ {
                Integer(i) => {
                    if i < 128 && i > -128 {
                        Mod::Offset8Bit
                    } else {
                        Mod::Offset32Bit
                    }
                }
                Reference(_) => Mod::Offset32Bit,
            }
        }
    } else {
        rm.1
    }
}

pub fn is_imm_of_size(iter: &mut Iter<Token>, size: usize) -> Result<Immediate, (usize, ParseError)> {
    if is_reg_of_size(&mut iter.clone(), 0).is_ok() { return Err((iter.count() - 1, ParseError::InvalidOperand)); }

    let next = get_next(iter)?;
    let (neg, num) = if next == "-" {
        let next = get_next(iter)?;

        (-1, next.parse::<isize>())
    } else {
        let r = next.parse::<isize>();

        if r.is_err() {
            return Ok(Immediate::reference(next.clone_string(), Absolute));
        }

        (1, r)
    };
    let num = num.unwrap();

    if (size_of::<usize>() * 8 - num.leading_zeros() as usize) <= size {
        Ok(Immediate::integer(num as i32 * neg, Absolute))
    } else {
        Err((iter.count(), ParseError::InvalidOperand))
    }
}

pub fn is_rel_of_size(iter: &mut Iter<Token>, size: usize) -> Result<Immediate, (usize, ParseError)> {
    // Todo: rel16/rel8
    if ![32usize, 64usize].contains(&size) {
        return Err((iter.count(), ParseError::InvalidOperand));
    }

    let next = get_next(iter)?;

    return Ok(Immediate::reference(next.clone_string(), Relative));
}

const REGS_8_BIT: [&str; 20] = [
    "al", "ah", "bl", "bh", "cl", "ch", "dl", "dh", "sil", "dil", "spl", "bpl", "r8b", "r9b", "r10b", "r11b", "r12b",
    "r13b", "r14b", "r15b",
];

const REGS_16_BIT: [&str; 17] = [
    "ax", "bx", "cx", "dx", "si", "di", "sp", "bp", "r8w", "r9w", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w", "ip",
];

const REGS_32_BIT: [&str; 17] = [
    "eax", "ebx", "ecx", "edx", "esi", "edi", "esp", "ebp", "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d",
    "r15d", "eip",
];

const REGS_64_BIT: [&str; 17] = [
    "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rsp", "rbp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "rip",
];

pub fn is_reg_of_size(iter: &mut Iter<Token>, size: usize) -> Result<Register, (usize, ParseError)> {
    let reg = get_next(iter)?;
    let works = match size {
        0 => {
            REGS_8_BIT.contains(&reg.as_str())
                || REGS_16_BIT.contains(&reg.as_str())
                || REGS_32_BIT.contains(&reg.as_str())
                || REGS_64_BIT.contains(&reg.as_str())
        }
        8 => REGS_8_BIT.contains(&reg.as_str()),
        16 => REGS_16_BIT.contains(&reg.as_str()),
        32 => REGS_32_BIT.contains(&reg.as_str()),
        64 => REGS_64_BIT.contains(&reg.as_str()),
        _ => panic!("Invalid size"),
    };
    if works {
        Ok(Register::from_str(reg.as_str()).unwrap())
    } else {
        Err((iter.count(), ParseError::InvalidOperand))
    }
}

pub fn is_rm_of_size(
    iter: &mut Iter<Token>,
    size: usize,
) -> Result<(Register, Mod, Option<Immediate>), (usize, ParseError)> {
    let reg_res = is_reg_of_size(&mut iter.clone(), size);
    if let Ok(reg_res) = reg_res {
        iter.next();
        Ok((reg_res, Mod::NoDereference, None))
    } else {
        is_m_of_size(iter, size)
    }
}

pub fn is_m_of_size(iter: &mut Iter<Token>,
                    size: usize,
) -> Result<(Register, Mod, Option<Immediate>), (usize, ParseError)> {
    let next = get_next(iter)?;
    if next != "[" {
        if match size {
            8 => next != "byte",
            16 => next != "word",
            32 => next != "dword",
            64 => next != "qword",
            0 => ["bytes", "word", "dword", "qword"].contains(&next.as_str()),
            _ => panic!("Invalid size"),
        } {
            return Err((iter.count(), ParseError::InvalidOperand));
        }
        if get_next(iter)? != "ptr" {
            return Err((iter.count(), ParseError::InvalidOperand));
        }
        if get_next(iter)? != "[" {
            return Err((iter.count(), ParseError::InvalidOperand));
        }
    };
    let reg_res = is_reg_of_size(iter, 0)?;

    let rel = if reg_res == Rip {
        RipRelative
    } else {
        Relative
    };

    let next = get_next(iter)?;

    let mod_byte: Mod;
    let mut off: Option<Immediate> = None;

    if ["+", "-"].contains(&next.as_str()) {
        let neg = if next.as_str() == "-" { -1 } else { 1 };
        let off_res = is_imm_of_size(iter, 32)?;
        off = match off_res.typ {
            Integer(i) => Some(Immediate::integer(i * neg, rel)),
            Reference(s) => Some(Immediate::reference(s, rel)),
        };

        if get_next(iter)? != "]" {
            return Err((iter.count(), ParseError::InvalidOperand));
        }

        mod_byte = Mod::Offset32Bit;
    } else if next != "]" {
        return Err((iter.count(), ParseError::InvalidOperand));
    } else {
        mod_byte = Mod::NoOffset;
    }
    return Ok((reg_res, mod_byte, off));
}
