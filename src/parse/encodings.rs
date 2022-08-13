// aron (c) Nikolas Wipper 2022

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::instructions::{Instruction, Opcode, Mod, Register};
use crate::parse::lexer::Token;
use crate::parse::ParseError;
use crate::parse::helpers::*;

fn matches_aaa1(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aaa" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("aaa".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x37)], None, None, None, None, None))
}

fn matches_aad2(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aad" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("aad".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD5), Opcode::Byte(0x0A)], None, None, None, None, None))
}

fn matches_aad3(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aad" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("aad".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD5)], None, None, None, Some(imm1), None))
}

fn matches_aam4(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aam" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("aam".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD4), Opcode::Byte(0x0A)], None, None, None, None, None))
}

fn matches_aam5(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aam" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("aam".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD4)], None, None, None, Some(imm1), None))
}

fn matches_aas6(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aas" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("aas".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x3F)], None, None, None, None, None))
}

fn matches_adc7(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x14)], None, None, None, Some(imm1), None))
}

fn matches_adc8(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x15)], None, None, None, Some(imm1), None))
}

fn matches_adc9(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x15)], None, None, None, Some(imm1), None))
}

fn matches_adc10(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x15)], None, None, None, Some(imm1), None))
}

fn matches_adc11(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x80)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_adc12(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x80)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_adc13(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_adc14(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_adc15(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x81)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_adc16(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_adc17(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_adc18(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x83)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_adc19(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x10)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adc20(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x10)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adc21(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x11)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adc22(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x11)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adc23(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x11)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adc24(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x12)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adc25(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x12)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adc26(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x13)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adc27(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x13)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adc28(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x13)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adcx29(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adcx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adcx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x66), Opcode::Byte(0x0F), Opcode::Byte(0x38), Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adcx30(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adcx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adcx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x66), Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x38), Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_add31(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x04)], None, None, None, Some(imm1), None))
}

fn matches_add32(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x05)], None, None, None, Some(imm1), None))
}

fn matches_add33(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x05)], None, None, None, Some(imm1), None))
}

fn matches_add34(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x05)], None, None, None, Some(imm1), None))
}

fn matches_add35(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x80)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_add36(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x80)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_add37(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_add38(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_add39(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x81)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_add40(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_add41(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_add42(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x83)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_add43(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x00)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_add44(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x00)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_add45(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x01)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_add46(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x01)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_add47(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x01)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_add48(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x02)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_add49(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x02)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_add50(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x03)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_add51(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x03)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_add52(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("add".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x03)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adox53(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adox" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adox".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0x38), Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_adox54(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adox" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("adox".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x38), Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_and55(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x24)], None, None, None, Some(imm1), None))
}

fn matches_and56(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x25)], None, None, None, Some(imm1), None))
}

fn matches_and57(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x25)], None, None, None, Some(imm1), None))
}

fn matches_and58(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x25)], None, None, None, Some(imm1), None))
}

fn matches_and59(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x80)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_and60(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x80)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_and61(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_and62(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_and63(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x81)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_and64(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_and65(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_and66(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x83)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_and67(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x20)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_and68(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x20)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_and69(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x21)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_and70(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x21)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_and71(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x21)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_and72(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x22)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_and73(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x22)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_and74(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x23)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_and75(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x23)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_and76(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("and".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x23)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_arpl77(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "arpl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("arpl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x63)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bsf78(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bsf".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBC)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bsf79(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bsf".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBC)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bsf80(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bsf".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xBC)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bsr81(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bsr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBD)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bsr82(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bsr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBD)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bsr83(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bsr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xBD)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bswap84(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bswap" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bswap".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xC8+reg as u8)], None, None, None, None, None))
}

fn matches_bswap85(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bswap" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bswap".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xC8+reg as u8)], None, None, None, None, None))
}

fn matches_bt86(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xA3)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bt87(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xA3)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bt88(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xA3)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bt89(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_bt90(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_bt91(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_btc92(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBB)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_btc93(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBB)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_btc94(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xBB)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_btc95(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_btc96(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_btc97(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_btr98(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xB3)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_btr99(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xB3)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_btr100(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xB3)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_btr101(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_btr102(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_btr103(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("btr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_bts104(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bts".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xAB)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bts105(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bts".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xAB)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bts106(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bts".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xAB)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_bts107(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bts".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_bts108(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bts".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_bts109(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("bts".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xBA)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_call110(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "call" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("call".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE8)], None, None, None, Some(rel), None))
}

fn matches_call111(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "call" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("call".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE8)], None, None, None, Some(rel), None))
}

fn matches_call112(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "call" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("call".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_call113(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "call" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("call".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_call114(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "call" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("call".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cbw115(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cbw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cbw".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x98)], None, None, None, None, None))
}

fn matches_cwde116(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cwde" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cwde".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x98)], None, None, None, None, None))
}

fn matches_cdqe117(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cdqe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cdqe".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x98)], None, None, None, None, None))
}

fn matches_clc118(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "clc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("clc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF8)], None, None, None, None, None))
}

fn matches_cld119(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cld" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cld".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xFC)], None, None, None, None, None))
}

fn matches_cli120(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cli" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cli".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xFA)], None, None, None, None, None))
}

fn matches_clts121(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "clts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("clts".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x06)], None, None, None, None, None))
}

fn matches_cmc122(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF5)], None, None, None, None, None))
}

fn matches_cmova123(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmova" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmova".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x47)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmova124(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmova" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmova".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x47)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmova125(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmova" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmova".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x47)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovae126(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovae".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x43)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovae127(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovae".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x43)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovae128(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovae".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x43)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovb129(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x42)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovb130(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x42)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovb131(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x42)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovbe132(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovbe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x46)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovbe133(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovbe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x46)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovbe134(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovbe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x46)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovc135(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x42)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovc136(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x42)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovc137(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x42)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmove138(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmove" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmove".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x44)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmove139(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmove" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmove".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x44)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmove140(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmove" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmove".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x44)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovg141(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4F)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovg142(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4F)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovg143(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4F)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovge144(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovge".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4D)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovge145(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovge".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4D)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovge146(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovge".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4D)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovl147(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4C)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovl148(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4C)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovl149(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4C)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovle150(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovle".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4E)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovle151(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovle".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4E)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovle152(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovle".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4E)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovna153(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovna".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x46)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovna154(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovna".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x46)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovna155(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovna".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x46)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnae156(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnae".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x42)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnae157(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnae".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x42)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnae158(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnae".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x42)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnb159(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x43)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnb160(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x43)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnb161(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x43)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnbe162(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnbe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x47)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnbe163(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnbe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x47)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnbe164(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnbe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x47)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnc165(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x43)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnc166(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x43)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnc167(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x43)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovne168(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovne".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x45)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovne169(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovne".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x45)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovne170(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovne".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x45)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovng171(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovng".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4E)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovng172(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovng".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4E)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovng173(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovng".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4E)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnge174(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnge".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4C)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnge175(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnge".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4C)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnge176(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnge".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4C)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnl177(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4D)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnl178(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4D)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnl179(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4D)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnle180(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnle".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4F)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnle181(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnle".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4F)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnle182(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnle".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4F)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovno183(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovno" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovno".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x41)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovno184(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovno" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovno".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x41)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovno185(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovno" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovno".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x41)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnp186(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnp187(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnp188(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovns189(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovns" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovns".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x49)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovns190(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovns" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovns".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x49)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovns191(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovns" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovns".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x49)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnz192(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnz".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x45)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnz193(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnz".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x45)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovnz194(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovnz".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x45)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovo195(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovo".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x40)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovo196(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovo".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x40)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovo197(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovo".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x40)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovp198(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovp199(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovp200(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovpe201(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovpe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovpe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovpe202(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovpe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovpe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x4A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmovpe203(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovpe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmovpe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x4A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmp204(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x3C)], None, None, None, Some(imm1), None))
}

fn matches_cmp205(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x3D)], None, None, None, Some(imm1), None))
}

fn matches_cmp206(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x3D)], None, None, None, Some(imm1), None))
}

fn matches_cmp207(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x3D)], None, None, None, Some(imm1), None))
}

fn matches_cmp208(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x80)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_cmp209(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x80)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_cmp210(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_cmp211(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_cmp212(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x81)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_cmp213(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_cmp214(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_cmp215(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x83)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_cmp216(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x38)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmp217(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x38)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmp218(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x39)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmp219(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x39)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmp220(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x39)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmp221(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x3A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmp222(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x3A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmp223(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x3B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmp224(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x3B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmp225(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x3B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmpsb226(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpsb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmpsb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xA6)], None, None, None, None, None))
}

fn matches_cmpsw227(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmpsw".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xA7)], None, None, None, None, None))
}

fn matches_cmpsd228(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpsd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmpsd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xA7)], None, None, None, None, None))
}

fn matches_cmpsq229(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpsq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmpsq".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0xA7)], None, None, None, None, None))
}

fn matches_cmpxchg230(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpxchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmpxchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xB0)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmpxchg231(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpxchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmpxchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0xB0)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmpxchg232(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpxchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmpxchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xB1)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmpxchg233(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpxchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmpxchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xB1)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cmpxchg234(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpxchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cmpxchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xB1)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cpuid235(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cpuid" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cpuid".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xA2)], None, None, None, None, None))
}

fn matches_crc32236(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("crc32".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF2), Opcode::Byte(0x0F), Opcode::Byte(0x38), Opcode::Byte(0xF0)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_crc32237(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("crc32".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF2), Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x38), Opcode::Byte(0xF0)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_crc32238(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("crc32".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF2), Opcode::Byte(0x0F), Opcode::Byte(0x38), Opcode::Byte(0xF1)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_crc32239(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("crc32".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF2), Opcode::Byte(0x0F), Opcode::Byte(0x38), Opcode::Byte(0xF1)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_crc32240(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("crc32".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF2), Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x38), Opcode::Byte(0xF0)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_crc32241(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("crc32".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF2), Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x38), Opcode::Byte(0xF1)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_cwd242(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cwd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cwd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x99)], None, None, None, None, None))
}

fn matches_cdq243(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cdq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cdq".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x99)], None, None, None, None, None))
}

fn matches_cqo244(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cqo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("cqo".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x99)], None, None, None, None, None))
}

fn matches_daa245(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "daa" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("daa".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x27)], None, None, None, None, None))
}

fn matches_das246(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "das" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("das".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x2F)], None, None, None, None, None))
}

fn matches_dec247(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("dec".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFE)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_dec248(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("dec".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xFE)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_dec249(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("dec".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_dec250(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("dec".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_dec251(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("dec".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_dec252(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("dec".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x48+reg as u8)], None, None, None, None, None))
}

fn matches_dec253(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("dec".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x48+reg as u8)], None, None, None, None, None))
}

fn matches_div254(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "div" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("div".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_div255(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "div" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("div".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_div256(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "div" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("div".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_div257(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "div" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("div".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_div258(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "div" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("div".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_enter259(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "enter" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "0" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("enter".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC8), Opcode::Byte(0x00)], None, None, None, Some(imm1), None))
}

fn matches_enter260(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "enter" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("enter".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC8), Opcode::Byte(0x01)], None, None, None, Some(imm1), None))
}

fn matches_enter261(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "enter" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm2 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("enter".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC8)], None, None, None, Some(imm1), Some(imm2)))
}

fn matches_f2xm1262(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "f2xm1" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("f2xm1".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xF0)], None, None, None, None, None))
}

fn matches_fabs263(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fabs" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fabs".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xE1)], None, None, None, None, None))
}

fn matches_faddp264(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "faddp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("faddp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDE), Opcode::Byte(0xC1)], None, None, None, None, None))
}

fn matches_fchs265(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fchs" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fchs".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xE0)], None, None, None, None, None))
}

fn matches_fclex266(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fclex" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fclex".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9B), Opcode::Byte(0xDB), Opcode::Byte(0xE2)], None, None, None, None, None))
}

fn matches_fnclex267(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fnclex" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fnclex".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDB), Opcode::Byte(0xE2)], None, None, None, None, None))
}

fn matches_fcom268(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fcom" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fcom".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD8), Opcode::Byte(0xD1)], None, None, None, None, None))
}

fn matches_fcomp269(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fcomp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fcomp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD8), Opcode::Byte(0xD9)], None, None, None, None, None))
}

fn matches_fcompp270(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fcompp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fcompp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDE), Opcode::Byte(0xD9)], None, None, None, None, None))
}

fn matches_fcos271(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fcos" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fcos".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xFF)], None, None, None, None, None))
}

fn matches_fdecstp272(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fdecstp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fdecstp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xF6)], None, None, None, None, None))
}

fn matches_fdivp273(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fdivp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fdivp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDE), Opcode::Byte(0xF9)], None, None, None, None, None))
}

fn matches_fdivrp274(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fdivrp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fdivrp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDE), Opcode::Byte(0xF1)], None, None, None, None, None))
}

fn matches_fincstp275(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fincstp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fincstp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xF7)], None, None, None, None, None))
}

fn matches_finit276(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "finit" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("finit".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9B), Opcode::Byte(0xDB), Opcode::Byte(0xE3)], None, None, None, None, None))
}

fn matches_fninit277(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fninit" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fninit".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDB), Opcode::Byte(0xE3)], None, None, None, None, None))
}

fn matches_fld1278(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fld1" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fld1".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xE8)], None, None, None, None, None))
}

fn matches_fldl2t279(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldl2t" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fldl2t".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xE9)], None, None, None, None, None))
}

fn matches_fldl2e280(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldl2e" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fldl2e".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xEA)], None, None, None, None, None))
}

fn matches_fldpi281(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldpi" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fldpi".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xEB)], None, None, None, None, None))
}

fn matches_fldlg2282(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldlg2" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fldlg2".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xEC)], None, None, None, None, None))
}

fn matches_fldln2283(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldln2" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fldln2".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xED)], None, None, None, None, None))
}

fn matches_fldz284(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fldz".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xEE)], None, None, None, None, None))
}

fn matches_fmulp285(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fmulp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fmulp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDE), Opcode::Byte(0xC9)], None, None, None, None, None))
}

fn matches_fnop286(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fnop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fnop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xD0)], None, None, None, None, None))
}

fn matches_fpatan287(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fpatan" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fpatan".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xF3)], None, None, None, None, None))
}

fn matches_fprem288(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fprem" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fprem".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xF8)], None, None, None, None, None))
}

fn matches_fprem1289(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fprem1" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fprem1".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xF5)], None, None, None, None, None))
}

fn matches_fptan290(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fptan" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fptan".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xF2)], None, None, None, None, None))
}

fn matches_frndint291(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "frndint" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("frndint".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xFC)], None, None, None, None, None))
}

fn matches_fscale292(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fscale" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fscale".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xFD)], None, None, None, None, None))
}

fn matches_fsin293(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fsin" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fsin".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xFE)], None, None, None, None, None))
}

fn matches_fsincos294(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fsincos" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fsincos".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xFB)], None, None, None, None, None))
}

fn matches_fsqrt295(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fsqrt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fsqrt".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xFA)], None, None, None, None, None))
}

fn matches_fstsw296(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fstsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fstsw".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9B), Opcode::Byte(0xDF), Opcode::Byte(0xE0)], None, None, None, None, None))
}

fn matches_fnstsw297(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fnstsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fnstsw".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDF), Opcode::Byte(0xE0)], None, None, None, None, None))
}

fn matches_fsubp298(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fsubp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fsubp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDE), Opcode::Byte(0xE9)], None, None, None, None, None))
}

fn matches_fsubrp299(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fsubrp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fsubrp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDE), Opcode::Byte(0xE1)], None, None, None, None, None))
}

fn matches_ftst300(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ftst" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ftst".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xE4)], None, None, None, None, None))
}

fn matches_fucom301(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fucom" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fucom".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDD), Opcode::Byte(0xE1)], None, None, None, None, None))
}

fn matches_fucomp302(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fucomp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fucomp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDD), Opcode::Byte(0xE9)], None, None, None, None, None))
}

fn matches_fucompp303(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fucompp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fucompp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xDA), Opcode::Byte(0xE9)], None, None, None, None, None))
}

fn matches_fxam304(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fxam" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fxam".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xE5)], None, None, None, None, None))
}

fn matches_fxch305(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fxch" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fxch".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xC9)], None, None, None, None, None))
}

fn matches_fxtract306(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fxtract" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fxtract".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xF4)], None, None, None, None, None))
}

fn matches_fyl2x307(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fyl2x" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fyl2x".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xF1)], None, None, None, None, None))
}

fn matches_fyl2xp1308(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fyl2xp1" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fyl2xp1".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD9), Opcode::Byte(0xF9)], None, None, None, None, None))
}

fn matches_hlt309(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "hlt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("hlt".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF4)], None, None, None, None, None))
}

fn matches_idiv310(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "idiv" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("idiv".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_idiv311(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "idiv" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("idiv".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_idiv312(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "idiv" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("idiv".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_idiv313(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "idiv" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("idiv".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_idiv314(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "idiv" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("idiv".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_imul315(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("imul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_imul316(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("imul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_imul317(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("imul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_imul318(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("imul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_imul319(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("imul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xAF)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_imul320(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("imul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xAF)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_imul321(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("imul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xAF)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_in322(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("in".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE4)], None, None, None, Some(imm1), None))
}

fn matches_in323(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("in".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE5)], None, None, None, Some(imm1), None))
}

fn matches_in324(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("in".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE5)], None, None, None, Some(imm1), None))
}

fn matches_in325(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("in".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xEC)], None, None, None, None, None))
}

fn matches_in326(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("in".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xED)], None, None, None, None, None))
}

fn matches_in327(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("in".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xED)], None, None, None, None, None))
}

fn matches_inc328(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("inc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFE)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_inc329(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("inc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xFE)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_inc330(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("inc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_inc331(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("inc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_inc332(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("inc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_inc333(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("inc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x40+reg as u8)], None, None, None, None, None))
}

fn matches_inc334(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("inc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x40+reg as u8)], None, None, None, None, None))
}

fn matches_insb335(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "insb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("insb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x6C)], None, None, None, None, None))
}

fn matches_insw336(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "insw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("insw".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x6D)], None, None, None, None, None))
}

fn matches_insd337(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "insd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("insd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x6D)], None, None, None, None, None))
}

fn matches_int338(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "int" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "3" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("int".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xCC)], None, None, None, None, None))
}

fn matches_into339(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "into" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("into".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xCE)], None, None, None, None, None))
}

fn matches_invd340(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "invd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("invd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x08)], None, None, None, None, None))
}

fn matches_invlpg341(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "invlpg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_m_of_size(&mut iter, 0)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("invlpg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x01)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_iret342(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "iret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("iret".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xCF)], None, None, None, None, None))
}

fn matches_iretd343(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "iretd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("iretd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xCF)], None, None, None, None, None))
}

fn matches_iretq344(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "iretq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("iretq".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0xCF)], None, None, None, None, None))
}

fn matches_ja345(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ja" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ja".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x77)], None, None, None, Some(rel), None))
}

fn matches_jae346(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jae".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x73)], None, None, None, Some(rel), None))
}

fn matches_jb347(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x72)], None, None, None, Some(rel), None))
}

fn matches_jbe348(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jbe".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x76)], None, None, None, Some(rel), None))
}

fn matches_jc349(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x72)], None, None, None, Some(rel), None))
}

fn matches_jcxz350(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jcxz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jcxz".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE3)], None, None, None, Some(rel), None))
}

fn matches_jecxz351(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jecxz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jecxz".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE3)], None, None, None, Some(rel), None))
}

fn matches_jrcxz352(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jrcxz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jrcxz".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE3)], None, None, None, Some(rel), None))
}

fn matches_je353(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "je" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("je".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x74)], None, None, None, Some(rel), None))
}

fn matches_jg354(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jg".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7F)], None, None, None, Some(rel), None))
}

fn matches_jge355(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jge".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7D)], None, None, None, Some(rel), None))
}

fn matches_jl356(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jl".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7C)], None, None, None, Some(rel), None))
}

fn matches_jle357(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jle".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7E)], None, None, None, Some(rel), None))
}

fn matches_jna358(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jna".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x76)], None, None, None, Some(rel), None))
}

fn matches_jnae359(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnae".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x72)], None, None, None, Some(rel), None))
}

fn matches_jnb360(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x73)], None, None, None, Some(rel), None))
}

fn matches_jnbe361(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnbe".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x77)], None, None, None, Some(rel), None))
}

fn matches_jnc362(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x73)], None, None, None, Some(rel), None))
}

fn matches_jne363(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jne".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x75)], None, None, None, Some(rel), None))
}

fn matches_jng364(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jng".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7E)], None, None, None, Some(rel), None))
}

fn matches_jnge365(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnge".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7C)], None, None, None, Some(rel), None))
}

fn matches_jnl366(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnl".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7D)], None, None, None, Some(rel), None))
}

fn matches_jnle367(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnle".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7F)], None, None, None, Some(rel), None))
}

fn matches_jno368(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jno" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jno".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x71)], None, None, None, Some(rel), None))
}

fn matches_jnp369(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7B)], None, None, None, Some(rel), None))
}

fn matches_jns370(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jns" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jns".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x79)], None, None, None, Some(rel), None))
}

fn matches_jnz371(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnz".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x75)], None, None, None, Some(rel), None))
}

fn matches_jo372(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jo".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x70)], None, None, None, Some(rel), None))
}

fn matches_jp373(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7A)], None, None, None, Some(rel), None))
}

fn matches_jpe374(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jpe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jpe".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7A)], None, None, None, Some(rel), None))
}

fn matches_jpo375(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jpo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jpo".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x7B)], None, None, None, Some(rel), None))
}

fn matches_js376(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "js" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("js".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x78)], None, None, None, Some(rel), None))
}

fn matches_jz377(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jz".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x74)], None, None, None, Some(rel), None))
}

fn matches_ja378(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ja" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ja".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x87)], None, None, None, Some(rel), None))
}

fn matches_ja379(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ja" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ja".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x87)], None, None, None, Some(rel), None))
}

fn matches_jae380(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jae".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x83)], None, None, None, Some(rel), None))
}

fn matches_jae381(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jae".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x83)], None, None, None, Some(rel), None))
}

fn matches_jb382(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x82)], None, None, None, Some(rel), None))
}

fn matches_jb383(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x82)], None, None, None, Some(rel), None))
}

fn matches_jbe384(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jbe".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x86)], None, None, None, Some(rel), None))
}

fn matches_jbe385(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jbe".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x86)], None, None, None, Some(rel), None))
}

fn matches_jc386(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x82)], None, None, None, Some(rel), None))
}

fn matches_jc387(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x82)], None, None, None, Some(rel), None))
}

fn matches_je388(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "je" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("je".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x84)], None, None, None, Some(rel), None))
}

fn matches_je389(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "je" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("je".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x84)], None, None, None, Some(rel), None))
}

fn matches_jz390(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jz".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x84)], None, None, None, Some(rel), None))
}

fn matches_jz391(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jz".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x84)], None, None, None, Some(rel), None))
}

fn matches_jg392(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jg".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8F)], None, None, None, Some(rel), None))
}

fn matches_jg393(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jg".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8F)], None, None, None, Some(rel), None))
}

fn matches_jge394(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jge".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8D)], None, None, None, Some(rel), None))
}

fn matches_jge395(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jge".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8D)], None, None, None, Some(rel), None))
}

fn matches_jl396(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jl".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8C)], None, None, None, Some(rel), None))
}

fn matches_jl397(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jl".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8C)], None, None, None, Some(rel), None))
}

fn matches_jle398(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jle".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8E)], None, None, None, Some(rel), None))
}

fn matches_jle399(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jle".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8E)], None, None, None, Some(rel), None))
}

fn matches_jna400(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jna".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x86)], None, None, None, Some(rel), None))
}

fn matches_jna401(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jna".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x86)], None, None, None, Some(rel), None))
}

fn matches_jnae402(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnae".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x82)], None, None, None, Some(rel), None))
}

fn matches_jnae403(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnae".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x82)], None, None, None, Some(rel), None))
}

fn matches_jnb404(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x83)], None, None, None, Some(rel), None))
}

fn matches_jnb405(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x83)], None, None, None, Some(rel), None))
}

fn matches_jnbe406(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnbe".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x87)], None, None, None, Some(rel), None))
}

fn matches_jnbe407(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnbe".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x87)], None, None, None, Some(rel), None))
}

fn matches_jnc408(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x83)], None, None, None, Some(rel), None))
}

fn matches_jnc409(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x83)], None, None, None, Some(rel), None))
}

fn matches_jne410(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jne".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x85)], None, None, None, Some(rel), None))
}

fn matches_jne411(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jne".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x85)], None, None, None, Some(rel), None))
}

fn matches_jng412(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jng".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8E)], None, None, None, Some(rel), None))
}

fn matches_jng413(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jng".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8E)], None, None, None, Some(rel), None))
}

fn matches_jnge414(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnge".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8C)], None, None, None, Some(rel), None))
}

fn matches_jnge415(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnge".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8C)], None, None, None, Some(rel), None))
}

fn matches_jnl416(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnl".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8D)], None, None, None, Some(rel), None))
}

fn matches_jnl417(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnl".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8D)], None, None, None, Some(rel), None))
}

fn matches_jnle418(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnle".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8F)], None, None, None, Some(rel), None))
}

fn matches_jnle419(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnle".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8F)], None, None, None, Some(rel), None))
}

fn matches_jno420(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jno" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jno".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x81)], None, None, None, Some(rel), None))
}

fn matches_jno421(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jno" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jno".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x81)], None, None, None, Some(rel), None))
}

fn matches_jnp422(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8B)], None, None, None, Some(rel), None))
}

fn matches_jnp423(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8B)], None, None, None, Some(rel), None))
}

fn matches_jns424(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jns" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jns".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x89)], None, None, None, Some(rel), None))
}

fn matches_jns425(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jns" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jns".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x89)], None, None, None, Some(rel), None))
}

fn matches_jnz426(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnz".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x85)], None, None, None, Some(rel), None))
}

fn matches_jnz427(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jnz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jnz".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x85)], None, None, None, Some(rel), None))
}

fn matches_jo428(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jo".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x80)], None, None, None, Some(rel), None))
}

fn matches_jo429(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jo".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x80)], None, None, None, Some(rel), None))
}

fn matches_jp430(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8A)], None, None, None, Some(rel), None))
}

fn matches_jp431(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8A)], None, None, None, Some(rel), None))
}

fn matches_jpe432(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jpe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jpe".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8A)], None, None, None, Some(rel), None))
}

fn matches_jpe433(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jpe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jpe".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8A)], None, None, None, Some(rel), None))
}

fn matches_jpo434(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jpo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jpo".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8B)], None, None, None, Some(rel), None))
}

fn matches_jpo435(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jpo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jpo".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x8B)], None, None, None, Some(rel), None))
}

fn matches_js436(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "js" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("js".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x88)], None, None, None, Some(rel), None))
}

fn matches_jmp437(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jmp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xEB)], None, None, None, Some(rel), None))
}

fn matches_jmp438(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jmp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE9)], None, None, None, Some(rel), None))
}

fn matches_jmp439(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jmp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE9)], None, None, None, Some(rel), None))
}

fn matches_jmp440(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_jmp441(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_jmp442(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("jmp".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_lahf443(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lahf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lahf".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9F)], None, None, None, None, None))
}

fn matches_lea444(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lea" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_m_of_size(&mut iter, 0)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lea".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x8D)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_lea445(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lea" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_m_of_size(&mut iter, 0)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lea".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x8D)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_lea446(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lea" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_m_of_size(&mut iter, 0)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lea".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x8D)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_leave447(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "leave" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("leave".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC9)], None, None, None, None, None))
}

fn matches_leave448(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "leave" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("leave".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC9)], None, None, None, None, None))
}

fn matches_leave449(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "leave" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("leave".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC9)], None, None, None, None, None))
}

fn matches_lldt450(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lldt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lldt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x00)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_lmsw451(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lmsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lmsw".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x01)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_lock452(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lock" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lock".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF0)], None, None, None, None, None))
}

fn matches_lodsb453(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lodsb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lodsb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xAC)], None, None, None, None, None))
}

fn matches_lodsw454(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lodsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lodsw".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xAD)], None, None, None, None, None))
}

fn matches_lodsd455(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lodsd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lodsd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xAD)], None, None, None, None, None))
}

fn matches_lodsq456(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lodsq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lodsq".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0xAD)], None, None, None, None, None))
}

fn matches_loop457(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "loop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("loop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE2)], None, None, None, Some(rel), None))
}

fn matches_loope458(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "loope" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("loope".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE1)], None, None, None, Some(rel), None))
}

fn matches_loopne459(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "loopne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("loopne".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE0)], None, None, None, Some(rel), None))
}

fn matches_ltr460(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ltr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ltr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x00)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_lzcnt461(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lzcnt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xBD)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_lzcnt462(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lzcnt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xBD)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_lzcnt463(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("lzcnt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xBD)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_monitor464(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "monitor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("monitor".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x01), Opcode::Byte(0xC8)], None, None, None, None, None))
}

fn matches_mov465(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x88)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mov466(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x88)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mov467(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x89)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mov468(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x89)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mov469(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x89)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mov470(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x8A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mov471(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x8A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mov472(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x8B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mov473(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x8B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mov474(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x8B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mov475(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xB0+reg as u8)], None, None, None, Some(imm1), None))
}

fn matches_mov476(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: false}, Opcode::Byte(0xB0+reg as u8)], None, None, None, Some(imm1), None))
}

fn matches_mov477(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xB8+reg as u8)], None, None, None, Some(imm1), None))
}

fn matches_mov478(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xB8+reg as u8)], None, None, None, Some(imm1), None))
}

fn matches_mov479(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0xB8+reg as u8)], None, None, None, Some(imm1), None))
}

fn matches_mov480(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC6)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_mov481(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xC6)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_mov482(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC7)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_mov483(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC7)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_mov484(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mov".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xC7)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_movsb485(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movsb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xA4)], None, None, None, None, None))
}

fn matches_movsw486(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movsw".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xA5)], None, None, None, None, None))
}

fn matches_movsd487(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movsd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xA5)], None, None, None, None, None))
}

fn matches_movsq488(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movsq".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0xA5)], None, None, None, None, None))
}

fn matches_movsx489(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movsx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBE)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_movsx490(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movsx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBE)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_movsx491(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movsx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0xBE)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_movsx492(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movsx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xBF)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_movsx493(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movsx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xBF)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_movsxd494(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsxd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movsxd".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x63)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_movzx495(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movzx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movzx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xB6)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_movzx496(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movzx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movzx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xB6)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_movzx497(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movzx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movzx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xB6)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_movzx498(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movzx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movzx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xB7)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_movzx499(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movzx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("movzx".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xB7)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mul500(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mul501(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mul502(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mul503(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mul504(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mul".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_mwait505(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mwait" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("mwait".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x01), Opcode::Byte(0xC9)], None, None, None, None, None))
}

fn matches_neg506(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "neg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("neg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_neg507(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "neg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("neg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_neg508(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "neg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("neg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_neg509(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "neg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("neg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_neg510(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "neg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("neg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_not511(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "not" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("not".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_not512(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "not" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("not".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_not513(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "not" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("not".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_not514(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "not" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("not".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_not515(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "not" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("not".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_or516(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0C)], None, None, None, Some(imm1), None))
}

fn matches_or517(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0D)], None, None, None, Some(imm1), None))
}

fn matches_or518(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0D)], None, None, None, Some(imm1), None))
}

fn matches_or519(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0D)], None, None, None, Some(imm1), None))
}

fn matches_or520(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x80)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_or521(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x80)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_or522(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_or523(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_or524(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x81)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_or525(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_or526(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_or527(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x83)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_or528(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x08)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_or529(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x08)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_or530(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x09)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_or531(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x09)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_or532(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x09)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_or533(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_or534(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_or535(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_or536(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_or537(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("or".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_out538(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("out".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE6)], None, None, None, Some(imm1), None))
}

fn matches_out539(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("out".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE7)], None, None, None, Some(imm1), None))
}

fn matches_out540(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("out".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xE7)], None, None, None, Some(imm1), None))
}

fn matches_out541(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("out".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xEE)], None, None, None, None, None))
}

fn matches_out542(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("out".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xEF)], None, None, None, None, None))
}

fn matches_out543(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("out".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xEF)], None, None, None, None, None))
}

fn matches_outsb544(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "outsb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("outsb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x6E)], None, None, None, None, None))
}

fn matches_outsw545(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "outsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("outsw".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x6F)], None, None, None, None, None))
}

fn matches_outsd546(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "outsd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("outsd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x6F)], None, None, None, None, None))
}

fn matches_pause547(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pause" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pause".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Byte(0x90)], None, None, None, None, None))
}

fn matches_pop548(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x8F)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_pop549(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x8F)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_pop550(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x8F)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_pop551(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x58+reg as u8)], None, None, None, None, None))
}

fn matches_pop552(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x58+reg as u8)], None, None, None, None, None))
}

fn matches_pop553(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x58+reg as u8)], None, None, None, None, None))
}

fn matches_pop554(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ds" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x1F)], None, None, None, None, None))
}

fn matches_pop555(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "es" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x07)], None, None, None, None, None))
}

fn matches_pop556(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ss" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x17)], None, None, None, None, None))
}

fn matches_pop557(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "fs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xA1)], None, None, None, None, None))
}

fn matches_pop558(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "fs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xA1)], None, None, None, None, None))
}

fn matches_pop559(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "fs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xA1)], None, None, None, None, None))
}

fn matches_pop560(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "gs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xA9)], None, None, None, None, None))
}

fn matches_pop561(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "gs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xA9)], None, None, None, None, None))
}

fn matches_pop562(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "gs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pop".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xA9)], None, None, None, None, None))
}

fn matches_popa563(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popa" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("popa".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x61)], None, None, None, None, None))
}

fn matches_popad564(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popad" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("popad".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x61)], None, None, None, None, None))
}

fn matches_popcnt565(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("popcnt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xB8)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_popcnt566(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("popcnt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xB8)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_popcnt567(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("popcnt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xB8)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_popf568(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("popf".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9D)], None, None, None, None, None))
}

fn matches_popfd569(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popfd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("popfd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9D)], None, None, None, None, None))
}

fn matches_popfq570(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popfq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("popfq".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9D)], None, None, None, None, None))
}

fn matches_push571(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_push572(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_push573(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xFF)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_push574(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x50+reg as u8)], None, None, None, None, None))
}

fn matches_push575(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x50+reg as u8)], None, None, None, None, None))
}

fn matches_push576(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x50+reg as u8)], None, None, None, None, None))
}

fn matches_push577(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x6A)], None, None, None, Some(imm1), None))
}

fn matches_push578(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x68)], None, None, None, Some(imm1), None))
}

fn matches_push579(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x68)], None, None, None, Some(imm1), None))
}

fn matches_push580(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "cs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0E)], None, None, None, None, None))
}

fn matches_push581(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ss" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x16)], None, None, None, None, None))
}

fn matches_push582(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ds" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x1E)], None, None, None, None, None))
}

fn matches_push583(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "es" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x06)], None, None, None, None, None))
}

fn matches_push584(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "fs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xA0)], None, None, None, None, None))
}

fn matches_push585(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "gs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("push".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xA8)], None, None, None, None, None))
}

fn matches_pusha586(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pusha" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pusha".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x60)], None, None, None, None, None))
}

fn matches_pushad587(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pushad" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pushad".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x60)], None, None, None, None, None))
}

fn matches_pushf588(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pushf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pushf".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9C)], None, None, None, None, None))
}

fn matches_pushfd589(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pushfd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pushfd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9C)], None, None, None, None, None))
}

fn matches_pushfq590(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pushfq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("pushfq".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9C)], None, None, None, None, None))
}

fn matches_rcl591(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcl592(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcl593(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcl594(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcl595(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rcl596(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rcl597(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcl598(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcl599(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rcl600(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcl601(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcl602(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcl603(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcl604(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rcl605(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(2 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rcr606(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcr607(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcr608(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcr609(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcr610(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rcr611(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rcr612(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcr613(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcr614(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rcr615(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcr616(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcr617(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcr618(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rcr619(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rcr620(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rcr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rol621(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rol622(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rol623(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rol624(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rol625(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rol626(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rol627(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rol628(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rol629(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rol630(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rol631(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rol632(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rol633(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_rol634(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rol635(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rol".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_ror636(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ror637(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ror638(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ror639(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ror640(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_ror641(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_ror642(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ror643(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ror644(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_ror645(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ror646(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ror647(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ror648(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ror649(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_ror650(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ror".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_rdfsbase651(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdfsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdfsbase".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xAE)], None, Some(Register::try_from(0).unwrap()), Some(reg), None, None))
}

fn matches_rdfsbase652(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdfsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdfsbase".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xAE)], None, Some(Register::try_from(0).unwrap()), Some(reg), None, None))
}

fn matches_rdgsbase653(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdgsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdgsbase".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xAE)], None, Some(Register::try_from(1).unwrap()), Some(reg), None, None))
}

fn matches_rdgsbase654(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdgsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdgsbase".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xAE)], None, Some(Register::try_from(1).unwrap()), Some(reg), None, None))
}

fn matches_rdmsr655(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdmsr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdmsr".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x32)], None, None, None, None, None))
}

fn matches_rdpid656(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdpid" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdpid".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xC7)], None, Some(Register::try_from(7).unwrap()), Some(reg), None, None))
}

fn matches_rdpid657(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdpid" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdpid".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xC7)], None, Some(Register::try_from(7).unwrap()), Some(reg), None, None))
}

fn matches_rdpmc658(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdpmc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdpmc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x33)], None, None, None, None, None))
}

fn matches_rdrand659(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdrand" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdrand".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xC7)], None, Some(Register::try_from(6).unwrap()), Some(reg), None, None))
}

fn matches_rdrand660(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdrand" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdrand".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xC7)], None, Some(Register::try_from(6).unwrap()), Some(reg), None, None))
}

fn matches_rdrand661(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdrand" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdrand".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xC7)], None, Some(Register::try_from(6).unwrap()), Some(reg), None, None))
}

fn matches_rdseed662(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdseed" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdseed".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xC7)], None, Some(Register::try_from(7).unwrap()), Some(reg), None, None))
}

fn matches_rdseed663(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdseed" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdseed".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xC7)], None, Some(Register::try_from(7).unwrap()), Some(reg), None, None))
}

fn matches_rdseed664(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdseed" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdseed".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xC7)], None, Some(Register::try_from(7).unwrap()), Some(reg), None, None))
}

fn matches_rdtsc665(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdtsc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdtsc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x31)], None, None, None, None, None))
}

fn matches_rdtscp666(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdtscp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rdtscp".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x01), Opcode::Byte(0xF9)], None, None, None, None, None))
}

fn matches_ret667(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ret".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC3)], None, None, None, None, None))
}

fn matches_retf668(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "retf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("retf".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xCB)], None, None, None, None, None))
}

fn matches_ret669(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ret".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC2)], None, None, None, Some(imm1), None))
}

fn matches_ret670(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ret".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xCA)], None, None, None, Some(imm1), None))
}

fn matches_rsm671(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rsm" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("rsm".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xAA)], None, None, None, None, None))
}

fn matches_sahf672(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sahf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sahf".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9E)], None, None, None, None, None))
}

fn matches_sal673(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sal674(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sal675(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sal676(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sal677(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sal678(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sal679(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sal680(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sal681(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sal682(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sal683(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sal684(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sal685(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sal686(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sal687(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sal".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sar688(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sar689(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sar690(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sar691(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sar692(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sar693(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sar694(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sar695(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sar696(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sar697(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sar698(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sar699(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sar700(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sar701(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sar702(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sar".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(7 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_shl703(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shl704(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shl705(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shl706(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shl707(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_shl708(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_shl709(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shl710(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shl711(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_shl712(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shl713(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shl714(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shl715(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shl716(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_shl717(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_shr718(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shr719(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD0)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shr720(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shr721(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xD2)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shr722(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_shr723(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_shr724(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shr725(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shr726(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_shr727(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shr728(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD1)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shr729(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shr730(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD3)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_shr731(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_shr732(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("shr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sbb733(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x1C)], None, None, None, Some(imm1), None))
}

fn matches_sbb734(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x1D)], None, None, None, Some(imm1), None))
}

fn matches_sbb735(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x1D)], None, None, None, Some(imm1), None))
}

fn matches_sbb736(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x1D)], None, None, None, Some(imm1), None))
}

fn matches_sbb737(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x80)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sbb738(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x80)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sbb739(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sbb740(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sbb741(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x81)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sbb742(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sbb743(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sbb744(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x83)], rm.2, Some(Register::try_from(3 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sbb745(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x18)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sbb746(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x18)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sbb747(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x19)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sbb748(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x19)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sbb749(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x19)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sbb750(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x1A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sbb751(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x1A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sbb752(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x1B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sbb753(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x1B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sbb754(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sbb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x1B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_scasb755(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "scasb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("scasb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xAE)], None, None, None, None, None))
}

fn matches_scasw756(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "scasw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("scasw".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xAF)], None, None, None, None, None))
}

fn matches_scasd757(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "scasd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("scasd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xAF)], None, None, None, None, None))
}

fn matches_scasq758(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "scasq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("scasq".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0xAF)], None, None, None, None, None))
}

fn matches_seta759(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "seta" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("seta".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x97)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_seta760(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "seta" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("seta".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x97)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setae761(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setae".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x93)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setae762(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setae".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x93)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setb763(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x92)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setb764(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x92)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setbe765(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setbe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x96)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setbe766(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setbe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x96)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setc767(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x92)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setc768(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x92)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sete769(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sete" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sete".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x94)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sete770(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sete" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sete".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x94)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setg771(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x9F)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setg772(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x9F)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setge773(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setge".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x9D)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setge774(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setge".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x9D)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setl775(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x9C)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setl776(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x9C)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setle777(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setle".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x9E)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setle778(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setle".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x9E)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setna779(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setna".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x96)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setna780(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setna".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x96)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnae781(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnae".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x92)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnae782(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnae".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x92)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnb783(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x93)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnb784(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnb".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x93)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnbe785(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnbe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x97)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnbe786(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnbe".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x97)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnc787(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x93)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnc788(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnc".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x93)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setne789(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setne".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x95)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setne790(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setne".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x95)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setng791(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setng".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x9E)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setng792(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setng".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x9E)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnge793(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnge".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x9C)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnge794(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnge".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x9C)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnl795(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x9D)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnl796(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnl".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0x9D)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_setnle797(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("setnle".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x9F)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sgdt798(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sgdt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_m_of_size(&mut iter, 0)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sgdt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x01)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sidt799(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sidt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_m_of_size(&mut iter, 0)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sidt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x01)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sldt800(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sldt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sldt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x00)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_smsw801(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "smsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("smsw".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x01)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_stc802(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "stc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("stc".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF9)], None, None, None, None, None))
}

fn matches_std803(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "std" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("std".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xFD)], None, None, None, None, None))
}

fn matches_sti804(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sti" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sti".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xFB)], None, None, None, None, None))
}

fn matches_stosb805(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "stosb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("stosb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xAA)], None, None, None, None, None))
}

fn matches_stosw806(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "stosw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("stosw".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xAB)], None, None, None, None, None))
}

fn matches_stosd807(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "stosd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("stosd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xAB)], None, None, None, None, None))
}

fn matches_stosq808(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "stosq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("stosq".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0xAB)], None, None, None, None, None))
}

fn matches_str809(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "str" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("str".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x00)], rm.2, Some(Register::try_from(1 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sub810(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x2C)], None, None, None, Some(imm1), None))
}

fn matches_sub811(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x2D)], None, None, None, Some(imm1), None))
}

fn matches_sub812(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x2D)], None, None, None, Some(imm1), None))
}

fn matches_sub813(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x2D)], None, None, None, Some(imm1), None))
}

fn matches_sub814(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x80)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sub815(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x80)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sub816(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sub817(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sub818(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x81)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sub819(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sub820(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sub821(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x83)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_sub822(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x28)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sub823(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x28)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sub824(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x29)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sub825(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x29)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sub826(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x29)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sub827(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x2A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sub828(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x2A)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sub829(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x2B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sub830(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x2B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_sub831(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sub".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x2B)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_swapgs832(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "swapgs" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("swapgs".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x01), Opcode::Byte(0xF8)], None, None, None, None, None))
}

fn matches_syscall833(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "syscall" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("syscall".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x05)], None, None, None, None, None))
}

fn matches_sysenter834(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sysenter" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sysenter".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x34)], None, None, None, None, None))
}

fn matches_sysexit835(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sysexit" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sysexit".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x35)], None, None, None, None, None))
}

fn matches_sysexit836(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sysexit" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sysexit".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x35)], None, None, None, None, None))
}

fn matches_sysret837(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sysret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sysret".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x07)], None, None, None, None, None))
}

fn matches_sysret838(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sysret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("sysret".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0x07)], None, None, None, None, None))
}

fn matches_test839(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xA8)], None, None, None, Some(imm1), None))
}

fn matches_test840(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xA9)], None, None, None, Some(imm1), None))
}

fn matches_test841(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xA9)], None, None, None, Some(imm1), None))
}

fn matches_test842(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0xA9)], None, None, None, Some(imm1), None))
}

fn matches_test843(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_test844(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0xF6)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_test845(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_test846(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_test847(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0xF7)], rm.2, Some(Register::try_from(0 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_test848(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x84)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_test849(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x84)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_test850(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x85)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_test851(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x85)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_test852(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("test".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x85)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_tzcnt853(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "tzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("tzcnt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xBC)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_tzcnt854(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "tzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("tzcnt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xBC)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_tzcnt855(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "tzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("tzcnt".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0xF3), Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xBC)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ud0856(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ud0" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ud0".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0xFF)], None, None, None, None, None))
}

fn matches_ud1857(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ud1" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ud1".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xB9)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_ud2858(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ud2" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("ud2".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x0B)], None, None, None, None, None))
}

fn matches_verr859(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "verr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("verr".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x00)], rm.2, Some(Register::try_from(4 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_verw860(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "verw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("verw".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0x00)], rm.2, Some(Register::try_from(5 as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_wait861(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wait" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("wait".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9B)], None, None, None, None, None))
}

fn matches_fwait862(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fwait" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("fwait".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x9B)], None, None, None, None, None))
}

fn matches_wbinvd863(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wbinvd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("wbinvd".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x09)], None, None, None, None, None))
}

fn matches_wrfsbase864(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wrfsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("wrfsbase".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xAE)], None, Some(Register::try_from(2).unwrap()), Some(reg), None, None))
}

fn matches_wrfsbase865(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wrfsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("wrfsbase".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xAE)], None, Some(Register::try_from(2).unwrap()), Some(reg), None, None))
}

fn matches_wrgsbase866(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wrgsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("wrgsbase".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Byte(0x0F), Opcode::Byte(0xAE)], None, Some(Register::try_from(3).unwrap()), Some(reg), None, None))
}

fn matches_wrgsbase867(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wrgsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("wrgsbase".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3), Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xAE)], None, Some(Register::try_from(3).unwrap()), Some(reg), None, None))
}

fn matches_wrmsr868(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wrmsr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("wrmsr".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x0F), Opcode::Byte(0x30)], None, None, None, None, None))
}

fn matches_xabort869(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xabort" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xabort".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC6), Opcode::Byte(0xF8)], None, None, None, Some(imm1), None))
}

fn matches_xacquire870(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xacquire" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xacquire".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF2)], None, None, None, None, None))
}

fn matches_xrelease871(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xrelease" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xrelease".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xF3)], None, None, None, None, None))
}

fn matches_xadd872(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xadd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xadd".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xadd873(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xadd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xadd".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x0F), Opcode::Byte(0xC0)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xadd874(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xadd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xadd".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xadd875(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xadd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xadd".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x0F), Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xadd876(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xadd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xadd".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x0F), Opcode::Byte(0xC1)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xbegin877(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xbegin" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xbegin".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC7), Opcode::Byte(0xF8)], None, None, None, Some(rel), None))
}

fn matches_xbegin878(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xbegin" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rel = is_rel_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xbegin".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xC7), Opcode::Byte(0xF8)], None, None, None, Some(rel), None))
}

fn matches_xchg879(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x90+reg as u8)], None, None, None, None, None))
}

fn matches_xchg880(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x90+reg as u8)], None, None, None, None, None))
}

fn matches_xchg881(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x90+reg as u8)], None, None, None, None, None))
}

fn matches_xchg882(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x90+reg as u8)], None, None, None, None, None))
}

fn matches_xchg883(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x90+reg as u8)], None, None, None, None, None))
}

fn matches_xchg884(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x90+reg as u8)], None, None, None, None, None))
}

fn matches_xchg885(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x86)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xchg886(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x86)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xchg887(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x86)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xchg888(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x86)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xchg889(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x87)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xchg890(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x87)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xchg891(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x87)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xchg892(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x87)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xchg893(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x87)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xchg894(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xchg".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x87)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xlatb895(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xlatb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xlatb".to_string(), Mod::NoDereference, vec![Opcode::Byte(0xD7)], None, None, None, None, None))
}

fn matches_xlatb896(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xlatb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xlatb".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0xD7)], None, None, None, None, None))
}

fn matches_xor897(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x34)], None, None, None, Some(imm1), None))
}

fn matches_xor898(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x35)], None, None, None, Some(imm1), None))
}

fn matches_xor899(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), Mod::NoDereference, vec![Opcode::Byte(0x35)], None, None, None, Some(imm1), None))
}

fn matches_xor900(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), Mod::NoDereference, vec![Opcode::Rex{wide: true}, Opcode::Byte(0x35)], None, None, None, Some(imm1), None))
}

fn matches_xor901(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x80)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_xor902(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x80)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_xor903(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_xor904(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x81)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_xor905(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x81)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_xor906(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_xor907(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x83)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_xor908(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm1 = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x83)], rm.2, Some(Register::try_from(6 as i32).unwrap()), Some(rm.0), Some(imm1), None))
}

fn matches_xor909(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x30)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xor910(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x30)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xor911(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x31)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xor912(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x31)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xor913(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x31)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xor914(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x32)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xor915(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: false}, Opcode::Byte(0x32)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xor916(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x33)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xor917(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Byte(0x33)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

fn matches_xor918(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }

    Ok(Instruction::new("xor".to_string(), get_mod_from_rm(&rm), vec![Opcode::Rex{wide: true}, Opcode::Byte(0x33)], rm.2, Some(Register::try_from(reg as i32).unwrap()), Some(rm.0), None, None))
}

const MATCH_FUNCTIONS: [fn(&Vec<Token>) -> Result<Instruction, (usize, ParseError)>; 918] = [matches_aaa1, matches_aad2, matches_aad3, matches_aam4, matches_aam5, matches_aas6, matches_adc7, matches_adc8, matches_adc9, matches_adc10, matches_adc11, matches_adc12, matches_adc13, matches_adc14, matches_adc15, matches_adc16, matches_adc17, matches_adc18, matches_adc19, matches_adc20, matches_adc21, matches_adc22, matches_adc23, matches_adc24, matches_adc25, matches_adc26, matches_adc27, matches_adc28, matches_adcx29, matches_adcx30, matches_add31, matches_add32, matches_add33, matches_add34, matches_add35, matches_add36, matches_add37, matches_add38, matches_add39, matches_add40, matches_add41, matches_add42, matches_add43, matches_add44, matches_add45, matches_add46, matches_add47, matches_add48, matches_add49, matches_add50, matches_add51, matches_add52, matches_adox53, matches_adox54, matches_and55, matches_and56, matches_and57, matches_and58, matches_and59, matches_and60, matches_and61, matches_and62, matches_and63, matches_and64, matches_and65, matches_and66, matches_and67, matches_and68, matches_and69, matches_and70, matches_and71, matches_and72, matches_and73, matches_and74, matches_and75, matches_and76, matches_arpl77, matches_bsf78, matches_bsf79, matches_bsf80, matches_bsr81, matches_bsr82, matches_bsr83, matches_bswap84, matches_bswap85, matches_bt86, matches_bt87, matches_bt88, matches_bt89, matches_bt90, matches_bt91, matches_btc92, matches_btc93, matches_btc94, matches_btc95, matches_btc96, matches_btc97, matches_btr98, matches_btr99, matches_btr100, matches_btr101, matches_btr102, matches_btr103, matches_bts104, matches_bts105, matches_bts106, matches_bts107, matches_bts108, matches_bts109, matches_call110, matches_call111, matches_call112, matches_call113, matches_call114, matches_cbw115, matches_cwde116, matches_cdqe117, matches_clc118, matches_cld119, matches_cli120, matches_clts121, matches_cmc122, matches_cmova123, matches_cmova124, matches_cmova125, matches_cmovae126, matches_cmovae127, matches_cmovae128, matches_cmovb129, matches_cmovb130, matches_cmovb131, matches_cmovbe132, matches_cmovbe133, matches_cmovbe134, matches_cmovc135, matches_cmovc136, matches_cmovc137, matches_cmove138, matches_cmove139, matches_cmove140, matches_cmovg141, matches_cmovg142, matches_cmovg143, matches_cmovge144, matches_cmovge145, matches_cmovge146, matches_cmovl147, matches_cmovl148, matches_cmovl149, matches_cmovle150, matches_cmovle151, matches_cmovle152, matches_cmovna153, matches_cmovna154, matches_cmovna155, matches_cmovnae156, matches_cmovnae157, matches_cmovnae158, matches_cmovnb159, matches_cmovnb160, matches_cmovnb161, matches_cmovnbe162, matches_cmovnbe163, matches_cmovnbe164, matches_cmovnc165, matches_cmovnc166, matches_cmovnc167, matches_cmovne168, matches_cmovne169, matches_cmovne170, matches_cmovng171, matches_cmovng172, matches_cmovng173, matches_cmovnge174, matches_cmovnge175, matches_cmovnge176, matches_cmovnl177, matches_cmovnl178, matches_cmovnl179, matches_cmovnle180, matches_cmovnle181, matches_cmovnle182, matches_cmovno183, matches_cmovno184, matches_cmovno185, matches_cmovnp186, matches_cmovnp187, matches_cmovnp188, matches_cmovns189, matches_cmovns190, matches_cmovns191, matches_cmovnz192, matches_cmovnz193, matches_cmovnz194, matches_cmovo195, matches_cmovo196, matches_cmovo197, matches_cmovp198, matches_cmovp199, matches_cmovp200, matches_cmovpe201, matches_cmovpe202, matches_cmovpe203, matches_cmp204, matches_cmp205, matches_cmp206, matches_cmp207, matches_cmp208, matches_cmp209, matches_cmp210, matches_cmp211, matches_cmp212, matches_cmp213, matches_cmp214, matches_cmp215, matches_cmp216, matches_cmp217, matches_cmp218, matches_cmp219, matches_cmp220, matches_cmp221, matches_cmp222, matches_cmp223, matches_cmp224, matches_cmp225, matches_cmpsb226, matches_cmpsw227, matches_cmpsd228, matches_cmpsq229, matches_cmpxchg230, matches_cmpxchg231, matches_cmpxchg232, matches_cmpxchg233, matches_cmpxchg234, matches_cpuid235, matches_crc32236, matches_crc32237, matches_crc32238, matches_crc32239, matches_crc32240, matches_crc32241, matches_cwd242, matches_cdq243, matches_cqo244, matches_daa245, matches_das246, matches_dec247, matches_dec248, matches_dec249, matches_dec250, matches_dec251, matches_dec252, matches_dec253, matches_div254, matches_div255, matches_div256, matches_div257, matches_div258, matches_enter259, matches_enter260, matches_enter261, matches_f2xm1262, matches_fabs263, matches_faddp264, matches_fchs265, matches_fclex266, matches_fnclex267, matches_fcom268, matches_fcomp269, matches_fcompp270, matches_fcos271, matches_fdecstp272, matches_fdivp273, matches_fdivrp274, matches_fincstp275, matches_finit276, matches_fninit277, matches_fld1278, matches_fldl2t279, matches_fldl2e280, matches_fldpi281, matches_fldlg2282, matches_fldln2283, matches_fldz284, matches_fmulp285, matches_fnop286, matches_fpatan287, matches_fprem288, matches_fprem1289, matches_fptan290, matches_frndint291, matches_fscale292, matches_fsin293, matches_fsincos294, matches_fsqrt295, matches_fstsw296, matches_fnstsw297, matches_fsubp298, matches_fsubrp299, matches_ftst300, matches_fucom301, matches_fucomp302, matches_fucompp303, matches_fxam304, matches_fxch305, matches_fxtract306, matches_fyl2x307, matches_fyl2xp1308, matches_hlt309, matches_idiv310, matches_idiv311, matches_idiv312, matches_idiv313, matches_idiv314, matches_imul315, matches_imul316, matches_imul317, matches_imul318, matches_imul319, matches_imul320, matches_imul321, matches_in322, matches_in323, matches_in324, matches_in325, matches_in326, matches_in327, matches_inc328, matches_inc329, matches_inc330, matches_inc331, matches_inc332, matches_inc333, matches_inc334, matches_insb335, matches_insw336, matches_insd337, matches_int338, matches_into339, matches_invd340, matches_invlpg341, matches_iret342, matches_iretd343, matches_iretq344, matches_ja345, matches_jae346, matches_jb347, matches_jbe348, matches_jc349, matches_jcxz350, matches_jecxz351, matches_jrcxz352, matches_je353, matches_jg354, matches_jge355, matches_jl356, matches_jle357, matches_jna358, matches_jnae359, matches_jnb360, matches_jnbe361, matches_jnc362, matches_jne363, matches_jng364, matches_jnge365, matches_jnl366, matches_jnle367, matches_jno368, matches_jnp369, matches_jns370, matches_jnz371, matches_jo372, matches_jp373, matches_jpe374, matches_jpo375, matches_js376, matches_jz377, matches_ja378, matches_ja379, matches_jae380, matches_jae381, matches_jb382, matches_jb383, matches_jbe384, matches_jbe385, matches_jc386, matches_jc387, matches_je388, matches_je389, matches_jz390, matches_jz391, matches_jg392, matches_jg393, matches_jge394, matches_jge395, matches_jl396, matches_jl397, matches_jle398, matches_jle399, matches_jna400, matches_jna401, matches_jnae402, matches_jnae403, matches_jnb404, matches_jnb405, matches_jnbe406, matches_jnbe407, matches_jnc408, matches_jnc409, matches_jne410, matches_jne411, matches_jng412, matches_jng413, matches_jnge414, matches_jnge415, matches_jnl416, matches_jnl417, matches_jnle418, matches_jnle419, matches_jno420, matches_jno421, matches_jnp422, matches_jnp423, matches_jns424, matches_jns425, matches_jnz426, matches_jnz427, matches_jo428, matches_jo429, matches_jp430, matches_jp431, matches_jpe432, matches_jpe433, matches_jpo434, matches_jpo435, matches_js436, matches_jmp437, matches_jmp438, matches_jmp439, matches_jmp440, matches_jmp441, matches_jmp442, matches_lahf443, matches_lea444, matches_lea445, matches_lea446, matches_leave447, matches_leave448, matches_leave449, matches_lldt450, matches_lmsw451, matches_lock452, matches_lodsb453, matches_lodsw454, matches_lodsd455, matches_lodsq456, matches_loop457, matches_loope458, matches_loopne459, matches_ltr460, matches_lzcnt461, matches_lzcnt462, matches_lzcnt463, matches_monitor464, matches_mov465, matches_mov466, matches_mov467, matches_mov468, matches_mov469, matches_mov470, matches_mov471, matches_mov472, matches_mov473, matches_mov474, matches_mov475, matches_mov476, matches_mov477, matches_mov478, matches_mov479, matches_mov480, matches_mov481, matches_mov482, matches_mov483, matches_mov484, matches_movsb485, matches_movsw486, matches_movsd487, matches_movsq488, matches_movsx489, matches_movsx490, matches_movsx491, matches_movsx492, matches_movsx493, matches_movsxd494, matches_movzx495, matches_movzx496, matches_movzx497, matches_movzx498, matches_movzx499, matches_mul500, matches_mul501, matches_mul502, matches_mul503, matches_mul504, matches_mwait505, matches_neg506, matches_neg507, matches_neg508, matches_neg509, matches_neg510, matches_not511, matches_not512, matches_not513, matches_not514, matches_not515, matches_or516, matches_or517, matches_or518, matches_or519, matches_or520, matches_or521, matches_or522, matches_or523, matches_or524, matches_or525, matches_or526, matches_or527, matches_or528, matches_or529, matches_or530, matches_or531, matches_or532, matches_or533, matches_or534, matches_or535, matches_or536, matches_or537, matches_out538, matches_out539, matches_out540, matches_out541, matches_out542, matches_out543, matches_outsb544, matches_outsw545, matches_outsd546, matches_pause547, matches_pop548, matches_pop549, matches_pop550, matches_pop551, matches_pop552, matches_pop553, matches_pop554, matches_pop555, matches_pop556, matches_pop557, matches_pop558, matches_pop559, matches_pop560, matches_pop561, matches_pop562, matches_popa563, matches_popad564, matches_popcnt565, matches_popcnt566, matches_popcnt567, matches_popf568, matches_popfd569, matches_popfq570, matches_push571, matches_push572, matches_push573, matches_push574, matches_push575, matches_push576, matches_push577, matches_push578, matches_push579, matches_push580, matches_push581, matches_push582, matches_push583, matches_push584, matches_push585, matches_pusha586, matches_pushad587, matches_pushf588, matches_pushfd589, matches_pushfq590, matches_rcl591, matches_rcl592, matches_rcl593, matches_rcl594, matches_rcl595, matches_rcl596, matches_rcl597, matches_rcl598, matches_rcl599, matches_rcl600, matches_rcl601, matches_rcl602, matches_rcl603, matches_rcl604, matches_rcl605, matches_rcr606, matches_rcr607, matches_rcr608, matches_rcr609, matches_rcr610, matches_rcr611, matches_rcr612, matches_rcr613, matches_rcr614, matches_rcr615, matches_rcr616, matches_rcr617, matches_rcr618, matches_rcr619, matches_rcr620, matches_rol621, matches_rol622, matches_rol623, matches_rol624, matches_rol625, matches_rol626, matches_rol627, matches_rol628, matches_rol629, matches_rol630, matches_rol631, matches_rol632, matches_rol633, matches_rol634, matches_rol635, matches_ror636, matches_ror637, matches_ror638, matches_ror639, matches_ror640, matches_ror641, matches_ror642, matches_ror643, matches_ror644, matches_ror645, matches_ror646, matches_ror647, matches_ror648, matches_ror649, matches_ror650, matches_rdfsbase651, matches_rdfsbase652, matches_rdgsbase653, matches_rdgsbase654, matches_rdmsr655, matches_rdpid656, matches_rdpid657, matches_rdpmc658, matches_rdrand659, matches_rdrand660, matches_rdrand661, matches_rdseed662, matches_rdseed663, matches_rdseed664, matches_rdtsc665, matches_rdtscp666, matches_ret667, matches_retf668, matches_ret669, matches_ret670, matches_rsm671, matches_sahf672, matches_sal673, matches_sal674, matches_sal675, matches_sal676, matches_sal677, matches_sal678, matches_sal679, matches_sal680, matches_sal681, matches_sal682, matches_sal683, matches_sal684, matches_sal685, matches_sal686, matches_sal687, matches_sar688, matches_sar689, matches_sar690, matches_sar691, matches_sar692, matches_sar693, matches_sar694, matches_sar695, matches_sar696, matches_sar697, matches_sar698, matches_sar699, matches_sar700, matches_sar701, matches_sar702, matches_shl703, matches_shl704, matches_shl705, matches_shl706, matches_shl707, matches_shl708, matches_shl709, matches_shl710, matches_shl711, matches_shl712, matches_shl713, matches_shl714, matches_shl715, matches_shl716, matches_shl717, matches_shr718, matches_shr719, matches_shr720, matches_shr721, matches_shr722, matches_shr723, matches_shr724, matches_shr725, matches_shr726, matches_shr727, matches_shr728, matches_shr729, matches_shr730, matches_shr731, matches_shr732, matches_sbb733, matches_sbb734, matches_sbb735, matches_sbb736, matches_sbb737, matches_sbb738, matches_sbb739, matches_sbb740, matches_sbb741, matches_sbb742, matches_sbb743, matches_sbb744, matches_sbb745, matches_sbb746, matches_sbb747, matches_sbb748, matches_sbb749, matches_sbb750, matches_sbb751, matches_sbb752, matches_sbb753, matches_sbb754, matches_scasb755, matches_scasw756, matches_scasd757, matches_scasq758, matches_seta759, matches_seta760, matches_setae761, matches_setae762, matches_setb763, matches_setb764, matches_setbe765, matches_setbe766, matches_setc767, matches_setc768, matches_sete769, matches_sete770, matches_setg771, matches_setg772, matches_setge773, matches_setge774, matches_setl775, matches_setl776, matches_setle777, matches_setle778, matches_setna779, matches_setna780, matches_setnae781, matches_setnae782, matches_setnb783, matches_setnb784, matches_setnbe785, matches_setnbe786, matches_setnc787, matches_setnc788, matches_setne789, matches_setne790, matches_setng791, matches_setng792, matches_setnge793, matches_setnge794, matches_setnl795, matches_setnl796, matches_setnle797, matches_sgdt798, matches_sidt799, matches_sldt800, matches_smsw801, matches_stc802, matches_std803, matches_sti804, matches_stosb805, matches_stosw806, matches_stosd807, matches_stosq808, matches_str809, matches_sub810, matches_sub811, matches_sub812, matches_sub813, matches_sub814, matches_sub815, matches_sub816, matches_sub817, matches_sub818, matches_sub819, matches_sub820, matches_sub821, matches_sub822, matches_sub823, matches_sub824, matches_sub825, matches_sub826, matches_sub827, matches_sub828, matches_sub829, matches_sub830, matches_sub831, matches_swapgs832, matches_syscall833, matches_sysenter834, matches_sysexit835, matches_sysexit836, matches_sysret837, matches_sysret838, matches_test839, matches_test840, matches_test841, matches_test842, matches_test843, matches_test844, matches_test845, matches_test846, matches_test847, matches_test848, matches_test849, matches_test850, matches_test851, matches_test852, matches_tzcnt853, matches_tzcnt854, matches_tzcnt855, matches_ud0856, matches_ud1857, matches_ud2858, matches_verr859, matches_verw860, matches_wait861, matches_fwait862, matches_wbinvd863, matches_wrfsbase864, matches_wrfsbase865, matches_wrgsbase866, matches_wrgsbase867, matches_wrmsr868, matches_xabort869, matches_xacquire870, matches_xrelease871, matches_xadd872, matches_xadd873, matches_xadd874, matches_xadd875, matches_xadd876, matches_xbegin877, matches_xbegin878, matches_xchg879, matches_xchg880, matches_xchg881, matches_xchg882, matches_xchg883, matches_xchg884, matches_xchg885, matches_xchg886, matches_xchg887, matches_xchg888, matches_xchg889, matches_xchg890, matches_xchg891, matches_xchg892, matches_xchg893, matches_xchg894, matches_xlatb895, matches_xlatb896, matches_xor897, matches_xor898, matches_xor899, matches_xor900, matches_xor901, matches_xor902, matches_xor903, matches_xor904, matches_xor905, matches_xor906, matches_xor907, matches_xor908, matches_xor909, matches_xor910, matches_xor911, matches_xor912, matches_xor913, matches_xor914, matches_xor915, matches_xor916, matches_xor917, matches_xor918];

pub fn matches(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {
    let mut i: Option<Instruction> = None;
    let mut l: usize = usize::MAX;
    let mut err: (usize, ParseError) = (tokens.len() - 1, ParseError::InvalidInstruction);
    
    for func in MATCH_FUNCTIONS {
        let instr = func(tokens);
        if let Ok(instr) = instr {
            let le = instr.encode().get_bytes().len();
            if i.is_none() || le < l {
                i = Some(instr);
                l = le;
            }
        } else {
            let instr_err = instr.unwrap_err();
            if instr_err.0 < err.0 {
                err = instr_err;
            }
        }
    }
    
    if let Some(i) = i {
        Ok(i)
    } else {
        Err(err)
    }
}
