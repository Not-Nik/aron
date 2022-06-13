// aron (c) Nikolas Wipper 2022

use std::mem::size_of;
use std::slice::Iter;
use std::str::FromStr;
use crate::instructions::{Mod, Register};
use crate::parse::lexer::Token;
use crate::parse::ParseError;

pub fn get_next<'a>(iter: &'a mut Iter<Token>) -> Result<&'a Token, (usize, ParseError)> {
    let next = iter.next();
    if next.is_none() {
        return Err((iter.count(), ParseError::UnexpectedLB));
    }
    Ok(next.unwrap())
}

pub fn get_mod_from_rm(rm: &(Register, Mod, Option<i32>)) -> Mod {
    if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Mod::Offset8Bit
        } else {
            Mod::Offset32Bit
        }
    } else {
        rm.1
    }
}

pub fn is_imm_of_size(iter: &mut Iter<Token>, size: usize) -> Result<i32, (usize, ParseError)> {
    let next = get_next(iter)?;
    let neg = if next == "-" { -1 } else { 1 };

    let num = if next == "-" {
        let next = get_next(iter)?;

        next.parse::<isize>()
    } else {
        next.parse::<isize>()
    };

    if num.is_err() {
        return Err((iter.count(), ParseError::InvalidOperand));
    }
    let num = num.unwrap();

    if (size_of::<usize>() * 8 - num.leading_zeros() as usize) <= size {
        Ok(num as i32 * neg)
    } else {
        Err((iter.count(), ParseError::InvalidOperand))
    }
}

const REGS_8_BIT: [&str; 20] = [
    "al", "ah", "bl", "bh", "cl", "ch", "dl", "dh", "sil", "dil", "spl", "bpl", "r8b", "r9b", "r10b", "r11b", "r12b",
    "r13b", "r14b", "r15b",
];

const REGS_16_BIT: [&str; 16] =
    ["ax", "bx", "cx", "dx", "si", "di", "sp", "bp", "r8w", "r9w", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w"];

const REGS_32_BIT: [&str; 16] = [
    "eax", "ebx", "ecx", "edx", "esi", "edi", "esp", "ebp", "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d",
    "r15d",
];

const REGS_64_BIT: [&str; 16] =
    ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rsp", "rbp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"];

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

pub fn is_rm_of_size(iter: &mut Iter<Token>, size: usize) -> Result<(Register, Mod, Option<i32>), (usize, ParseError)> {
    let reg_res = is_reg_of_size(&mut iter.clone(), size);
    if let Ok(reg_res) = reg_res {
        iter.next();
        return Ok((reg_res, Mod::NoDereference, None));
    }

    let next = get_next(iter)?;
    if match size {
        8 => next != "byte",
        16 => next != "word",
        32 => next != "dword",
        64 => next != "qword",
        _ => panic!("Invalid size"),
    } { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().unwrap() != "ptr" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().unwrap() != "[" { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg_res = is_reg_of_size(iter, 0)?;

    let next = get_next(iter)?;

    let mod_byte: Mod;
    let mut off: Option<i32> = None;

    if ["+", "-"].contains(&next.as_str()) {
        let neg = if next.as_str() == "-" { -1 } else { 1 };
        let off_res = is_imm_of_size(iter, 32)?;
        off = Some(off_res * neg);

        if get_next(iter)? != "]" { return Err((iter.count(), ParseError::InvalidOperand)); }

        mod_byte = Mod::Offset32Bit;
    } else if next != "]" {
        return Err((iter.count(), ParseError::InvalidOperand));
    } else {
        mod_byte = Mod::NoOffset;
    }
    return Ok((reg_res, mod_byte, off));
}
