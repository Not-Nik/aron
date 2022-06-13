// aron (c) Nikolas Wipper 2022

use crate::instructions::Instruction;
use crate::parse::ParseError;
use crate::parse::helpers::*;

fn matches_aaa1(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aaa" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("aaa".to_string());
    instr.write_byte(0x37);

    Ok(instr)
}

fn matches_aad2(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aad" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("aad".to_string());
    instr.write_byte(0xD5);
    instr.write_byte(0x0A);

    Ok(instr)
}

fn matches_aad3(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aad" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("aad".to_string());
    instr.write_byte(0xD5);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_aam4(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aam" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("aam".to_string());
    instr.write_byte(0xD4);
    instr.write_byte(0x0A);

    Ok(instr)
}

fn matches_aam5(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aam" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("aam".to_string());
    instr.write_byte(0xD4);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_aas6(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "aas" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("aas".to_string());
    instr.write_byte(0x3F);

    Ok(instr)
}

fn matches_adc7(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());
    instr.write_byte(0x14);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_adc8(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());
    instr.write_byte(0x15);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_adc9(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());
    instr.write_byte(0x15);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_adc10(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x15);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_adc11(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_adc12(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 2 as u8);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_adc13(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_adc14(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_adc15(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_adc16(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_adc17(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_adc18(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_adc19(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x10);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adc20(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x10);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adc21(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x11);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adc22(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x11);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adc23(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x11);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adc24(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x12);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adc25(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x12);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adc26(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x13);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adc27(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x13);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adc28(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x13);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adcx29(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adcx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adcx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x66);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adcx30(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adcx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adcx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x66);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_add31(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());
    instr.write_byte(0x04);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_add32(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());
    instr.write_byte(0x05);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_add33(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());
    instr.write_byte(0x05);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_add34(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x05);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_add35(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_add36(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_add37(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_add38(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_add39(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_add40(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_add41(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_add42(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_add43(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x00);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_add44(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x00);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_add45(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x01);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_add46(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x01);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_add47(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x01);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_add48(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x02);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_add49(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x02);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_add50(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x03);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_add51(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x03);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_add52(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "add" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("add".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x03);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adox53(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adox" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adox".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_adox54(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "adox" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("adox".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_and55(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());
    instr.write_byte(0x24);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_and56(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());
    instr.write_byte(0x25);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_and57(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());
    instr.write_byte(0x25);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_and58(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x25);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_and59(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_and60(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_and61(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_and62(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_and63(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_and64(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_and65(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_and66(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_and67(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x20);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_and68(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x20);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_and69(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x21);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_and70(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x21);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_and71(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x21);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_and72(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x22);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_and73(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x22);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_and74(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x23);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_and75(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x23);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_and76(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "and" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("and".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x23);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_arpl77(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "arpl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("arpl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x63);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bsf78(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bsf".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bsf79(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bsf".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bsf80(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bsf".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bsr81(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bsr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bsr82(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bsr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bsr83(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bsr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bsr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bswap84(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bswap" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bswap".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xC8+reg as u8);

    Ok(instr)
}

fn matches_bswap85(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bswap" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bswap".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xC8+reg as u8);

    Ok(instr)
}

fn matches_bt86(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xA3);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bt87(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xA3);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bt88(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xA3);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bt89(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_bt90(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_bt91(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_btc92(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBB);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_btc93(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBB);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_btc94(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBB);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_btc95(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_btc96(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_btc97(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_btr98(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xB3);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_btr99(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xB3);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_btr100(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB3);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_btr101(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_btr102(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_btr103(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "btr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("btr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 6 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_bts104(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bts".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xAB);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bts105(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bts".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xAB);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bts106(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bts".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAB);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_bts107(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bts".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_bts108(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bts".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_bts109(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "bts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("bts".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_call110(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "call" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("call".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_call111(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "call" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("call".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_call112(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "call" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("call".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_cbw113(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cbw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cbw".to_string());
    instr.write_byte(0x98);

    Ok(instr)
}

fn matches_cwde114(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cwde" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cwde".to_string());
    instr.write_byte(0x98);

    Ok(instr)
}

fn matches_cdqe115(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cdqe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cdqe".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x98);

    Ok(instr)
}

fn matches_clc116(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "clc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("clc".to_string());
    instr.write_byte(0xF8);

    Ok(instr)
}

fn matches_cld117(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cld" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cld".to_string());
    instr.write_byte(0xFC);

    Ok(instr)
}

fn matches_cli118(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cli" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cli".to_string());
    instr.write_byte(0xFA);

    Ok(instr)
}

fn matches_clts119(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "clts" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("clts".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x06);

    Ok(instr)
}

fn matches_cmc120(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmc".to_string());
    instr.write_byte(0xF5);

    Ok(instr)
}

fn matches_cmova121(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmova" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmova".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmova122(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmova" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmova".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmova123(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmova" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmova".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovae124(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovae".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovae125(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovae".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovae126(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovae".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovb127(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovb128(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovb129(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovbe130(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovbe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovbe131(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovbe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovbe132(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovbe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovc133(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovc134(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovc135(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmove136(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmove" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmove".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x44);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmove137(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmove" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmove".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x44);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmove138(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmove" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmove".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x44);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovg139(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovg140(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovg141(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovge142(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovge".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovge143(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovge".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovge144(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovge".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovl145(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovl146(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovl147(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovle148(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovle".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovle149(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovle".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovle150(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovle".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovna151(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovna".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovna152(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovna".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovna153(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovna".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnae154(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnae".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnae155(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnae".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnae156(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnae".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnb157(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnb158(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnb159(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnbe160(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnbe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnbe161(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnbe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnbe162(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnbe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnc163(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnc164(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnc165(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovne166(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovne".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovne167(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovne".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovne168(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovne".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovng169(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovng".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovng170(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovng".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovng171(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovng".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnge172(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnge".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnge173(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnge".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnge174(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnge".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnl175(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnl176(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnl177(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnle178(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnle".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnle179(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnle".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnle180(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnle".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovno181(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovno" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovno".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x41);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovno182(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovno" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovno".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x41);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovno183(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovno" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovno".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x41);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnp184(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnp185(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnp186(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovns187(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovns" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovns".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x49);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovns188(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovns" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovns".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x49);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovns189(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovns" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovns".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x49);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnz190(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnz".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnz191(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnz".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovnz192(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovnz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovnz".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovo193(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovo".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x40);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovo194(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovo".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x40);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovo195(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovo".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x40);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovp196(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovp197(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovp198(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovpe199(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovpe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovpe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovpe200(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovpe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovpe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmovpe201(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmovpe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmovpe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmp202(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());
    instr.write_byte(0x3C);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_cmp203(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());
    instr.write_byte(0x3D);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_cmp204(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());
    instr.write_byte(0x3D);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_cmp205(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x3D);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_cmp206(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_cmp207(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 7 as u8);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_cmp208(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_cmp209(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_cmp210(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_cmp211(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_cmp212(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_cmp213(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_cmp214(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x38);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmp215(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x38);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmp216(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x39);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmp217(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x39);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmp218(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x39);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmp219(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x3A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmp220(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x3A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmp221(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x3B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmp222(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x3B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmp223(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x3B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmpsb224(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpsb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmpsb".to_string());
    instr.write_byte(0xA6);

    Ok(instr)
}

fn matches_cmpsw225(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmpsw".to_string());
    instr.write_byte(0xA7);

    Ok(instr)
}

fn matches_cmpsd226(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpsd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmpsd".to_string());
    instr.write_byte(0xA7);

    Ok(instr)
}

fn matches_cmpsq227(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpsq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmpsq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xA7);

    Ok(instr)
}

fn matches_cmpxchg228(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpxchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmpxchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xB0);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmpxchg229(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpxchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmpxchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB0);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmpxchg230(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpxchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmpxchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xB1);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmpxchg231(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpxchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmpxchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xB1);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cmpxchg232(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cmpxchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cmpxchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB1);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cpuid233(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cpuid" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cpuid".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA2);

    Ok(instr)
}

fn matches_crc32234(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("crc32".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF2);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF0);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_crc32235(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("crc32".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF2);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF0);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_crc32236(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("crc32".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF2);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF1);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_crc32237(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("crc32".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF2);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF1);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_crc32238(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("crc32".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF2);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF0);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_crc32239(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "crc32" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("crc32".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF2);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF1);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_cwd240(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cwd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cwd".to_string());
    instr.write_byte(0x99);

    Ok(instr)
}

fn matches_cdq241(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cdq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cdq".to_string());
    instr.write_byte(0x99);

    Ok(instr)
}

fn matches_cqo242(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "cqo" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("cqo".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x99);

    Ok(instr)
}

fn matches_daa243(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "daa" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("daa".to_string());
    instr.write_byte(0x27);

    Ok(instr)
}

fn matches_das244(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "das" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("das".to_string());
    instr.write_byte(0x2F);

    Ok(instr)
}

fn matches_dec245(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("dec".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFE);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_dec246(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("dec".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 1 as u8);
    instr.write_byte(0xFE);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_dec247(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("dec".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_dec248(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("dec".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_dec249(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("dec".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_dec250(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("dec".to_string());
    instr.write_byte(0x48+reg as u8);

    Ok(instr)
}

fn matches_dec251(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "dec" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("dec".to_string());
    instr.write_byte(0x48+reg as u8);

    Ok(instr)
}

fn matches_div252(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "div" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("div".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);

    Ok(instr)
}

fn matches_div253(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "div" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("div".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 6 as u8);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);

    Ok(instr)
}

fn matches_div254(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "div" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("div".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);

    Ok(instr)
}

fn matches_div255(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "div" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("div".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);

    Ok(instr)
}

fn matches_div256(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "div" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("div".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 6 as u8);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);

    Ok(instr)
}

fn matches_enter257(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "enter" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "0" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("enter".to_string());
    instr.write_byte(0xC8);
    instr.write_byte(0x00);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_enter258(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "enter" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("enter".to_string());
    instr.write_byte(0xC8);
    instr.write_byte(0x01);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_enter259(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "enter" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("enter".to_string());
    instr.write_byte(0xC8);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_f2xm1260(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "f2xm1" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("f2xm1".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF0);

    Ok(instr)
}

fn matches_fabs261(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fabs" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fabs".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE1);

    Ok(instr)
}

fn matches_faddp262(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "faddp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("faddp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xC1);

    Ok(instr)
}

fn matches_fchs263(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fchs" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fchs".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE0);

    Ok(instr)
}

fn matches_fclex264(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fclex" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fclex".to_string());
    instr.write_byte(0x9B);
    instr.write_byte(0xDB);
    instr.write_byte(0xE2);

    Ok(instr)
}

fn matches_fnclex265(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fnclex" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fnclex".to_string());
    instr.write_byte(0xDB);
    instr.write_byte(0xE2);

    Ok(instr)
}

fn matches_fcom266(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fcom" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fcom".to_string());
    instr.write_byte(0xD8);
    instr.write_byte(0xD1);

    Ok(instr)
}

fn matches_fcomp267(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fcomp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fcomp".to_string());
    instr.write_byte(0xD8);
    instr.write_byte(0xD9);

    Ok(instr)
}

fn matches_fcompp268(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fcompp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fcompp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xD9);

    Ok(instr)
}

fn matches_fcos269(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fcos" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fcos".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFF);

    Ok(instr)
}

fn matches_fdecstp270(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fdecstp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fdecstp".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF6);

    Ok(instr)
}

fn matches_fdivp271(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fdivp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fdivp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xF9);

    Ok(instr)
}

fn matches_fdivrp272(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fdivrp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fdivrp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xF1);

    Ok(instr)
}

fn matches_fincstp273(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fincstp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fincstp".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF7);

    Ok(instr)
}

fn matches_finit274(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "finit" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("finit".to_string());
    instr.write_byte(0x9B);
    instr.write_byte(0xDB);
    instr.write_byte(0xE3);

    Ok(instr)
}

fn matches_fninit275(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fninit" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fninit".to_string());
    instr.write_byte(0xDB);
    instr.write_byte(0xE3);

    Ok(instr)
}

fn matches_fld1276(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fld1" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fld1".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE8);

    Ok(instr)
}

fn matches_fldl2t277(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldl2t" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fldl2t".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE9);

    Ok(instr)
}

fn matches_fldl2e278(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldl2e" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fldl2e".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xEA);

    Ok(instr)
}

fn matches_fldpi279(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldpi" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fldpi".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xEB);

    Ok(instr)
}

fn matches_fldlg2280(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldlg2" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fldlg2".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xEC);

    Ok(instr)
}

fn matches_fldln2281(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldln2" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fldln2".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xED);

    Ok(instr)
}

fn matches_fldz282(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fldz" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fldz".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xEE);

    Ok(instr)
}

fn matches_fmulp283(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fmulp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fmulp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xC9);

    Ok(instr)
}

fn matches_fnop284(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fnop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fnop".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xD0);

    Ok(instr)
}

fn matches_fpatan285(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fpatan" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fpatan".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF3);

    Ok(instr)
}

fn matches_fprem286(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fprem" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fprem".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF8);

    Ok(instr)
}

fn matches_fprem1287(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fprem1" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fprem1".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF5);

    Ok(instr)
}

fn matches_fptan288(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fptan" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fptan".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF2);

    Ok(instr)
}

fn matches_frndint289(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "frndint" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("frndint".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFC);

    Ok(instr)
}

fn matches_fscale290(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fscale" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fscale".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFD);

    Ok(instr)
}

fn matches_fsin291(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fsin" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fsin".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFE);

    Ok(instr)
}

fn matches_fsincos292(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fsincos" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fsincos".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFB);

    Ok(instr)
}

fn matches_fsqrt293(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fsqrt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fsqrt".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFA);

    Ok(instr)
}

fn matches_fstsw294(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fstsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fstsw".to_string());
    instr.write_byte(0x9B);
    instr.write_byte(0xDF);
    instr.write_byte(0xE0);

    Ok(instr)
}

fn matches_fnstsw295(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fnstsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fnstsw".to_string());
    instr.write_byte(0xDF);
    instr.write_byte(0xE0);

    Ok(instr)
}

fn matches_fsubp296(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fsubp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fsubp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xE9);

    Ok(instr)
}

fn matches_fsubrp297(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fsubrp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fsubrp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xE1);

    Ok(instr)
}

fn matches_ftst298(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ftst" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ftst".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE4);

    Ok(instr)
}

fn matches_fucom299(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fucom" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fucom".to_string());
    instr.write_byte(0xDD);
    instr.write_byte(0xE1);

    Ok(instr)
}

fn matches_fucomp300(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fucomp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fucomp".to_string());
    instr.write_byte(0xDD);
    instr.write_byte(0xE9);

    Ok(instr)
}

fn matches_fucompp301(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fucompp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fucompp".to_string());
    instr.write_byte(0xDA);
    instr.write_byte(0xE9);

    Ok(instr)
}

fn matches_fxam302(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fxam" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fxam".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE5);

    Ok(instr)
}

fn matches_fxch303(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fxch" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fxch".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xC9);

    Ok(instr)
}

fn matches_fxtract304(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fxtract" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fxtract".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF4);

    Ok(instr)
}

fn matches_fyl2x305(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fyl2x" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fyl2x".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF1);

    Ok(instr)
}

fn matches_fyl2xp1306(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fyl2xp1" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fyl2xp1".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF9);

    Ok(instr)
}

fn matches_hlt307(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "hlt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("hlt".to_string());
    instr.write_byte(0xF4);

    Ok(instr)
}

fn matches_idiv308(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "idiv" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("idiv".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_idiv309(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "idiv" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("idiv".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 7 as u8);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_idiv310(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "idiv" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("idiv".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_idiv311(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "idiv" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("idiv".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_idiv312(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "idiv" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("idiv".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_imul313(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("imul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_imul314(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("imul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_imul315(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("imul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_imul316(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("imul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_imul317(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("imul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xAF);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_imul318(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("imul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xAF);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_imul319(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "imul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("imul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAF);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_in320(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xE4);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_in321(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xE5);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_in322(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xE5);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_in323(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xEC);

    Ok(instr)
}

fn matches_in324(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xED);

    Ok(instr)
}

fn matches_in325(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "in" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xED);

    Ok(instr)
}

fn matches_inc326(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("inc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFE);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_inc327(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("inc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xFE);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_inc328(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("inc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_inc329(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("inc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_inc330(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("inc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_inc331(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("inc".to_string());
    instr.write_byte(0x40+reg as u8);

    Ok(instr)
}

fn matches_inc332(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "inc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("inc".to_string());
    instr.write_byte(0x40+reg as u8);

    Ok(instr)
}

fn matches_insb333(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "insb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("insb".to_string());
    instr.write_byte(0x6C);

    Ok(instr)
}

fn matches_insw334(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "insw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("insw".to_string());
    instr.write_byte(0x6D);

    Ok(instr)
}

fn matches_insd335(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "insd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("insd".to_string());
    instr.write_byte(0x6D);

    Ok(instr)
}

fn matches_int336(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "int" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "3" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("int".to_string());
    instr.write_byte(0xCC);

    Ok(instr)
}

fn matches_into337(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "into" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("into".to_string());
    instr.write_byte(0xCE);

    Ok(instr)
}

fn matches_invd338(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "invd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("invd".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x08);

    Ok(instr)
}

fn matches_iret339(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "iret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("iret".to_string());
    instr.write_byte(0xCF);

    Ok(instr)
}

fn matches_iretd340(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "iretd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("iretd".to_string());
    instr.write_byte(0xCF);

    Ok(instr)
}

fn matches_iretq341(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "iretq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("iretq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xCF);

    Ok(instr)
}

fn matches_jmp342(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("jmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_jmp343(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("jmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_jmp344(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "jmp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("jmp".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_lahf345(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lahf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lahf".to_string());
    instr.write_byte(0x9F);

    Ok(instr)
}

fn matches_leave346(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "leave" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("leave".to_string());
    instr.write_byte(0xC9);

    Ok(instr)
}

fn matches_leave347(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "leave" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("leave".to_string());
    instr.write_byte(0xC9);

    Ok(instr)
}

fn matches_leave348(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "leave" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("leave".to_string());
    instr.write_byte(0xC9);

    Ok(instr)
}

fn matches_lldt349(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lldt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lldt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_lmsw350(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lmsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lmsw".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);

    Ok(instr)
}

fn matches_lock351(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lock" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lock".to_string());
    instr.write_byte(0xF0);

    Ok(instr)
}

fn matches_lodsb352(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lodsb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lodsb".to_string());
    instr.write_byte(0xAC);

    Ok(instr)
}

fn matches_lodsw353(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lodsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lodsw".to_string());
    instr.write_byte(0xAD);

    Ok(instr)
}

fn matches_lodsd354(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lodsd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lodsd".to_string());
    instr.write_byte(0xAD);

    Ok(instr)
}

fn matches_lodsq355(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lodsq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lodsq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xAD);

    Ok(instr)
}

fn matches_ltr356(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ltr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ltr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_lzcnt357(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lzcnt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_lzcnt358(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lzcnt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_lzcnt359(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "lzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("lzcnt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_monitor360(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "monitor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("monitor".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_byte(0xC8);

    Ok(instr)
}

fn matches_mov361(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x88);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mov362(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x88);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mov363(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x89);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mov364(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x89);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mov365(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x89);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mov366(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x8A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mov367(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x8A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mov368(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x8B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mov369(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x8B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mov370(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x8B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mov371(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());
    instr.write_byte(0xB0+reg as u8);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_mov372(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());
    instr.write_rex(false, 0 as u8, reg as u8);
    instr.write_byte(0xB0+reg as u8);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_mov373(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());
    instr.write_byte(0xB8+reg as u8);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_mov374(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());
    instr.write_byte(0xB8+reg as u8);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_mov375(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0xB8+reg as u8);
    instr.write_num(imm as i64);

    Ok(instr)
}

fn matches_mov376(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC6);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_mov377(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xC6);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_mov378(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC7);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_mov379(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC7);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_mov380(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mov" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mov".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xC7);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_movsb381(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movsb".to_string());
    instr.write_byte(0xA4);

    Ok(instr)
}

fn matches_movsw382(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movsw".to_string());
    instr.write_byte(0xA5);

    Ok(instr)
}

fn matches_movsd383(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movsd".to_string());
    instr.write_byte(0xA5);

    Ok(instr)
}

fn matches_movsq384(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movsq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xA5);

    Ok(instr)
}

fn matches_movsx385(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movsx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBE);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_movsx386(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movsx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBE);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_movsx387(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movsx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBE);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_movsx388(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movsx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xBF);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_movsx389(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movsx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBF);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_movsxd390(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movsxd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movsxd".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x63);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_movzx391(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movzx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movzx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xB6);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_movzx392(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movzx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movzx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xB6);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_movzx393(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movzx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movzx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB6);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_movzx394(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movzx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movzx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xB7);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_movzx395(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "movzx" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("movzx".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB7);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_mul396(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_mul397(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_mul398(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_mul399(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_mul400(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mul" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mul".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_mwait401(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "mwait" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("mwait".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_byte(0xC9);

    Ok(instr)
}

fn matches_neg402(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "neg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("neg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_neg403(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "neg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("neg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 3 as u8);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_neg404(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "neg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("neg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_neg405(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "neg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("neg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_neg406(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "neg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("neg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_not407(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "not" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("not".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_not408(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "not" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("not".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 2 as u8);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_not409(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "not" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("not".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_not410(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "not" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("not".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_not411(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "not" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("not".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_or412(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());
    instr.write_byte(0x0C);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_or413(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());
    instr.write_byte(0x0D);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_or414(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());
    instr.write_byte(0x0D);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_or415(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x0D);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_or416(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_or417(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 1 as u8);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_or418(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_or419(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_or420(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_or421(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_or422(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_or423(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_or424(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x08);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_or425(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x08);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_or426(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x09);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_or427(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x09);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_or428(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x09);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_or429(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_or430(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_or431(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_or432(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_or433(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "or" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("or".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_out434(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xE6);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_out435(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xE7);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_out436(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xE7);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_out437(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xEE);

    Ok(instr)
}

fn matches_out438(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xEF);

    Ok(instr)
}

fn matches_out439(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "out" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "dx" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xEF);

    Ok(instr)
}

fn matches_outsb440(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "outsb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("outsb".to_string());
    instr.write_byte(0x6E);

    Ok(instr)
}

fn matches_outsw441(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "outsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("outsw".to_string());
    instr.write_byte(0x6F);

    Ok(instr)
}

fn matches_outsd442(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "outsd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("outsd".to_string());
    instr.write_byte(0x6F);

    Ok(instr)
}

fn matches_pause443(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pause" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pause".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x90);

    Ok(instr)
}

fn matches_pop444(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x8F);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_pop445(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x8F);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_pop446(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x8F);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_pop447(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x58+reg as u8);

    Ok(instr)
}

fn matches_pop448(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x58+reg as u8);

    Ok(instr)
}

fn matches_pop449(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x58+reg as u8);

    Ok(instr)
}

fn matches_pop450(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ds" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x1F);

    Ok(instr)
}

fn matches_pop451(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "es" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x07);

    Ok(instr)
}

fn matches_pop452(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ss" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x17);

    Ok(instr)
}

fn matches_pop453(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "fs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA1);

    Ok(instr)
}

fn matches_pop454(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "fs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA1);

    Ok(instr)
}

fn matches_pop455(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "fs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA1);

    Ok(instr)
}

fn matches_pop456(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "gs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA9);

    Ok(instr)
}

fn matches_pop457(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "gs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA9);

    Ok(instr)
}

fn matches_pop458(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pop" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "gs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA9);

    Ok(instr)
}

fn matches_popa459(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popa" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("popa".to_string());
    instr.write_byte(0x61);

    Ok(instr)
}

fn matches_popad460(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popad" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("popad".to_string());
    instr.write_byte(0x61);

    Ok(instr)
}

fn matches_popcnt461(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("popcnt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xB8);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_popcnt462(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("popcnt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xB8);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_popcnt463(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("popcnt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB8);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_popf464(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("popf".to_string());
    instr.write_byte(0x9D);

    Ok(instr)
}

fn matches_popfd465(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popfd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("popfd".to_string());
    instr.write_byte(0x9D);

    Ok(instr)
}

fn matches_popfq466(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "popfq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("popfq".to_string());
    instr.write_byte(0x9D);

    Ok(instr)
}

fn matches_push467(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);

    Ok(instr)
}

fn matches_push468(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);

    Ok(instr)
}

fn matches_push469(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xFF);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);

    Ok(instr)
}

fn matches_push470(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x50+reg as u8);

    Ok(instr)
}

fn matches_push471(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x50+reg as u8);

    Ok(instr)
}

fn matches_push472(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x50+reg as u8);

    Ok(instr)
}

fn matches_push473(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x6A);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_push474(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x68);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_push475(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x68);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_push476(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "cs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x0E);

    Ok(instr)
}

fn matches_push477(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ss" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x16);

    Ok(instr)
}

fn matches_push478(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ds" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x1E);

    Ok(instr)
}

fn matches_push479(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "es" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x06);

    Ok(instr)
}

fn matches_push480(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "fs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA0);

    Ok(instr)
}

fn matches_push481(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "push" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "gs" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA8);

    Ok(instr)
}

fn matches_pusha482(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pusha" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pusha".to_string());
    instr.write_byte(0x60);

    Ok(instr)
}

fn matches_pushad483(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pushad" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pushad".to_string());
    instr.write_byte(0x60);

    Ok(instr)
}

fn matches_pushf484(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pushf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pushf".to_string());
    instr.write_byte(0x9C);

    Ok(instr)
}

fn matches_pushfd485(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pushfd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pushfd".to_string());
    instr.write_byte(0x9C);

    Ok(instr)
}

fn matches_pushfq486(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "pushfq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("pushfq".to_string());
    instr.write_byte(0x9C);

    Ok(instr)
}

fn matches_rcl487(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_rcl488(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 2 as u8);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_rcl489(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_rcl490(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 2 as u8);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_rcl491(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rcl492(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 2 as u8);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rcl493(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_rcl494(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_rcl495(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rcl496(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_rcl497(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_rcl498(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_rcl499(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);

    Ok(instr)
}

fn matches_rcl500(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rcl501(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 2 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rcr502(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_rcr503(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 3 as u8);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_rcr504(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_rcr505(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 3 as u8);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_rcr506(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rcr507(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 3 as u8);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rcr508(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_rcr509(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_rcr510(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rcr511(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_rcr512(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_rcr513(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_rcr514(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);

    Ok(instr)
}

fn matches_rcr515(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rcr516(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rcr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rcr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rol517(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_rol518(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_rol519(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_rol520(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_rol521(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rol522(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rol523(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_rol524(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_rol525(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rol526(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_rol527(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_rol528(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_rol529(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_rol530(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rol531(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rol" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rol".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_ror532(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_ror533(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 1 as u8);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_ror534(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_ror535(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 1 as u8);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_ror536(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_ror537(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 1 as u8);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_ror538(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_ror539(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_ror540(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_ror541(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_ror542(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_ror543(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_ror544(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_ror545(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_ror546(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ror" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ror".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_rdfsbase547(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdfsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdfsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Ok(instr)
}

fn matches_rdfsbase548(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdfsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdfsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Ok(instr)
}

fn matches_rdgsbase549(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdgsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdgsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Ok(instr)
}

fn matches_rdgsbase550(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdgsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdgsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Ok(instr)
}

fn matches_rdmsr551(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdmsr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdmsr".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x32);

    Ok(instr)
}

fn matches_rdpid552(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdpid" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdpid".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Ok(instr)
}

fn matches_rdpid553(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdpid" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdpid".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Ok(instr)
}

fn matches_rdpmc554(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdpmc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdpmc".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x33);

    Ok(instr)
}

fn matches_rdrand555(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdrand" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdrand".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Ok(instr)
}

fn matches_rdrand556(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdrand" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdrand".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Ok(instr)
}

fn matches_rdrand557(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdrand" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdrand".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Ok(instr)
}

fn matches_rdseed558(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdseed" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdseed".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Ok(instr)
}

fn matches_rdseed559(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdseed" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdseed".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Ok(instr)
}

fn matches_rdseed560(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdseed" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdseed".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Ok(instr)
}

fn matches_rdtsc561(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdtsc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdtsc".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x31);

    Ok(instr)
}

fn matches_rdtscp562(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rdtscp" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rdtscp".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_byte(0xF9);

    Ok(instr)
}

fn matches_ret563(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ret".to_string());
    instr.write_byte(0xC3);

    Ok(instr)
}

fn matches_retf564(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "retf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("retf".to_string());
    instr.write_byte(0xCB);

    Ok(instr)
}

fn matches_ret565(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ret".to_string());
    instr.write_byte(0xC2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_ret566(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ret".to_string());
    instr.write_byte(0xCA);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_rsm567(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "rsm" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("rsm".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xAA);

    Ok(instr)
}

fn matches_sahf568(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sahf" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sahf".to_string());
    instr.write_byte(0x9E);

    Ok(instr)
}

fn matches_sal569(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_sal570(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_sal571(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_sal572(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_sal573(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sal574(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sal575(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_sal576(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_sal577(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sal578(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_sal579(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_sal580(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_sal581(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_sal582(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sal583(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sal" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sal".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sar584(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_sar585(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 7 as u8);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_sar586(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_sar587(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 7 as u8);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_sar588(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sar589(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 7 as u8);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sar590(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_sar591(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_sar592(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sar593(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_sar594(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_sar595(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_sar596(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);

    Ok(instr)
}

fn matches_sar597(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sar598(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sar" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sar".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 7 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_shl599(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_shl600(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_shl601(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_shl602(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_shl603(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_shl604(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_shl605(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_shl606(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_shl607(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_shl608(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_shl609(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_shl610(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_shl611(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_shl612(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_shl613(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_shr614(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_shr615(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 5 as u8);
    instr.write_byte(0xD0);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_shr616(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_shr617(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 5 as u8);
    instr.write_byte(0xD2);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_shr618(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_shr619(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 5 as u8);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_shr620(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_shr621(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_shr622(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_shr623(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_shr624(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "1" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0xD1);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_shr625(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_shr626(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "cl" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0xD3);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_shr627(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_shr628(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "shr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("shr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sbb629(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());
    instr.write_byte(0x1C);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sbb630(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());
    instr.write_byte(0x1D);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_sbb631(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());
    instr.write_byte(0x1D);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_sbb632(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x1D);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_sbb633(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sbb634(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 3 as u8);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sbb635(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_sbb636(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_sbb637(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_sbb638(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sbb639(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sbb640(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 3 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sbb641(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x18);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sbb642(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x18);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sbb643(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x19);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sbb644(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x19);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sbb645(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x19);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sbb646(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x1A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sbb647(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x1A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sbb648(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x1B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sbb649(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x1B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sbb650(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sbb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sbb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x1B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_scasb651(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "scasb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("scasb".to_string());
    instr.write_byte(0xAE);

    Ok(instr)
}

fn matches_scasw652(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "scasw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("scasw".to_string());
    instr.write_byte(0xAF);

    Ok(instr)
}

fn matches_scasd653(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "scasd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("scasd".to_string());
    instr.write_byte(0xAF);

    Ok(instr)
}

fn matches_scasq654(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "scasq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("scasq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xAF);

    Ok(instr)
}

fn matches_seta655(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "seta" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("seta".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x97);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_seta656(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "seta" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("seta".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x97);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setae657(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setae".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setae658(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setae".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setb659(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setb660(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setbe661(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setbe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x96);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setbe662(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setbe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x96);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setc663(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setc664(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_sete665(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sete" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sete".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x94);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_sete666(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sete" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sete".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x94);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setg667(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x9F);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setg668(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9F);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setge669(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setge".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x9D);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setge670(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setge".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9D);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setl671(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x9C);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setl672(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9C);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setle673(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setle".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x9E);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setle674(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setle".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9E);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setna675(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setna".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x96);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setna676(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setna" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setna".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x96);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnae677(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnae".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnae678(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnae" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnae".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnb679(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnb680(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnb".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnbe681(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnbe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x97);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnbe682(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnbe" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnbe".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x97);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnc683(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnc684(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnc".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setne685(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setne".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x95);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setne686(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setne" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setne".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x95);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setng687(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setng".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x9E);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setng688(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setng" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setng".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9E);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnge689(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnge".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x9C);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnge690(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnge" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnge".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9C);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnl691(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x9D);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnl692(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnl" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnl".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9D);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_setnle693(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "setnle" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("setnle".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x9F);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_sldt694(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sldt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sldt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);

    Ok(instr)
}

fn matches_smsw695(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "smsw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("smsw".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_stc696(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "stc" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("stc".to_string());
    instr.write_byte(0xF9);

    Ok(instr)
}

fn matches_std697(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "std" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("std".to_string());
    instr.write_byte(0xFD);

    Ok(instr)
}

fn matches_sti698(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sti" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sti".to_string());
    instr.write_byte(0xFB);

    Ok(instr)
}

fn matches_stosb699(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "stosb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("stosb".to_string());
    instr.write_byte(0xAA);

    Ok(instr)
}

fn matches_stosw700(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "stosw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("stosw".to_string());
    instr.write_byte(0xAB);

    Ok(instr)
}

fn matches_stosd701(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "stosd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("stosd".to_string());
    instr.write_byte(0xAB);

    Ok(instr)
}

fn matches_stosq702(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "stosq" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("stosq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xAB);

    Ok(instr)
}

fn matches_str703(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "str" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("str".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_offset(m, rm.0 as u8, 1 as u8, rm.2);

    Ok(instr)
}

fn matches_sub704(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());
    instr.write_byte(0x2C);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sub705(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());
    instr.write_byte(0x2D);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_sub706(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());
    instr.write_byte(0x2D);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_sub707(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x2D);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_sub708(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sub709(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 5 as u8);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sub710(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_sub711(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_sub712(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_sub713(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sub714(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sub715(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_sub716(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x28);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sub717(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x28);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sub718(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x29);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sub719(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x29);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sub720(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x29);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sub721(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x2A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sub722(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x2A);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sub723(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x2B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sub724(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x2B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_sub725(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sub" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sub".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x2B);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_swapgs726(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "swapgs" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("swapgs".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_byte(0xF8);

    Ok(instr)
}

fn matches_syscall727(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "syscall" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("syscall".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x05);

    Ok(instr)
}

fn matches_sysenter728(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sysenter" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sysenter".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x34);

    Ok(instr)
}

fn matches_sysexit729(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sysexit" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sysexit".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x35);

    Ok(instr)
}

fn matches_sysexit730(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sysexit" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sysexit".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x35);

    Ok(instr)
}

fn matches_sysret731(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sysret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sysret".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x07);

    Ok(instr)
}

fn matches_sysret732(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "sysret" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("sysret".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x07);

    Ok(instr)
}

fn matches_test733(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());
    instr.write_byte(0xA8);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_test734(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());
    instr.write_byte(0xA9);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_test735(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());
    instr.write_byte(0xA9);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_test736(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xA9);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_test737(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_test738(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xF6);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_test739(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_test740(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_test741(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xF7);
    instr.write_offset(m, rm.0 as u8, 0 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_test742(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x84);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_test743(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x84);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_test744(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x85);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_test745(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x85);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_test746(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "test" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("test".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x85);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_tzcnt747(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "tzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("tzcnt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_tzcnt748(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "tzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("tzcnt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_tzcnt749(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "tzcnt" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("tzcnt".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0xF3);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_ud0750(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ud0" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ud0".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xFF);

    Ok(instr)
}

fn matches_ud1751(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ud1" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ud1".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xB9);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_ud2752(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "ud2" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("ud2".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x0B);

    Ok(instr)
}

fn matches_verr753(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "verr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("verr".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_offset(m, rm.0 as u8, 4 as u8, rm.2);

    Ok(instr)
}

fn matches_verw754(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "verw" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("verw".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_offset(m, rm.0 as u8, 5 as u8, rm.2);

    Ok(instr)
}

fn matches_wait755(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wait" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("wait".to_string());
    instr.write_byte(0x9B);

    Ok(instr)
}

fn matches_fwait756(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "fwait" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("fwait".to_string());
    instr.write_byte(0x9B);

    Ok(instr)
}

fn matches_wbinvd757(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wbinvd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("wbinvd".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x09);

    Ok(instr)
}

fn matches_wrfsbase758(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wrfsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("wrfsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Ok(instr)
}

fn matches_wrfsbase759(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wrfsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("wrfsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Ok(instr)
}

fn matches_wrgsbase760(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wrgsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("wrgsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Ok(instr)
}

fn matches_wrgsbase761(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wrgsbase" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("wrgsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Ok(instr)
}

fn matches_wrmsr762(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "wrmsr" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("wrmsr".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x30);

    Ok(instr)
}

fn matches_xabort763(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xabort" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xabort".to_string());
    instr.write_byte(0xC6);
    instr.write_byte(0xF8);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_xacquire764(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xacquire" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xacquire".to_string());
    instr.write_byte(0xF2);

    Ok(instr)
}

fn matches_xrelease765(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xrelease" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xrelease".to_string());
    instr.write_byte(0xF3);

    Ok(instr)
}

fn matches_xadd766(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xadd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xadd".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xadd767(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xadd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xadd".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xC0);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xadd768(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xadd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xadd".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xadd769(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xadd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xadd".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x0F);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xadd770(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xadd" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xadd".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xC1);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xchg771(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_byte(0x90+reg as u8);

    Ok(instr)
}

fn matches_xchg772(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_byte(0x90+reg as u8);

    Ok(instr)
}

fn matches_xchg773(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_byte(0x90+reg as u8);

    Ok(instr)
}

fn matches_xchg774(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x90+reg as u8);

    Ok(instr)
}

fn matches_xchg775(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_byte(0x90+reg as u8);

    Ok(instr)
}

fn matches_xchg776(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x90+reg as u8);

    Ok(instr)
}

fn matches_xchg777(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x86);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xchg778(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x86);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xchg779(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x86);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xchg780(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x86);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xchg781(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x87);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xchg782(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x87);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xchg783(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x87);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xchg784(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x87);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xchg785(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x87);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xchg786(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xchg" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xchg".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x87);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xlatb787(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xlatb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xlatb".to_string());
    instr.write_byte(0xD7);

    Ok(instr)
}

fn matches_xlatb788(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xlatb" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xlatb".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xD7);

    Ok(instr)
}

fn matches_xor789(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "al" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());
    instr.write_byte(0x34);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_xor790(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "ax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());
    instr.write_byte(0x35);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_xor791(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "eax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());
    instr.write_byte(0x35);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_xor792(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    if get_next(&mut iter)? != "rax" { return Err((iter.count(), ParseError::InvalidOperand)); }
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x35);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_xor793(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_xor794(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, 6 as u8);
    instr.write_byte(0x80);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_xor795(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i16);

    Ok(instr)
}

fn matches_xor796(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_xor797(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 6 as u8);
    instr.write_byte(0x81);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i32);

    Ok(instr)
}

fn matches_xor798(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_xor799(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_xor800(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let imm = is_imm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, 6 as u8);
    instr.write_byte(0x83);
    instr.write_offset(m, rm.0 as u8, 6 as u8, rm.2);
    instr.write_num(imm as i8);

    Ok(instr)
}

fn matches_xor801(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x30);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xor802(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x30);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xor803(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x31);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xor804(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x31);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xor805(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x31);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xor806(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x32);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xor807(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 8)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 8)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x32);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xor808(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 16)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 16)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x33);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xor809(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 32)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 32)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_byte(0x33);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

fn matches_xor810(tokens: &Vec<String>) -> Result<Instruction, (usize, ParseError)> {
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != "xor" { return Err((iter.count(), ParseError::InvalidInstruction)); }
    let reg = is_reg_of_size(&mut iter, 64)?;
    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }
    let rm = is_rm_of_size(&mut iter, 64)?;
    if iter.next().is_some() { return Err((iter.count() + 1, ParseError::ExtraneousTokenAfterInstruction)); }
    let mut instr = Instruction::new("xor".to_string());

    let m = get_mod_from_rm(&rm);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x33);
    instr.write_offset(m, rm.0 as u8, reg as u8, rm.2);

    Ok(instr)
}

const MATCH_FUNCTIONS: [fn(&Vec<String>) -> Result<Instruction, (usize, ParseError)>; 810] = [matches_aaa1, matches_aad2, matches_aad3, matches_aam4, matches_aam5, matches_aas6, matches_adc7, matches_adc8, matches_adc9, matches_adc10, matches_adc11, matches_adc12, matches_adc13, matches_adc14, matches_adc15, matches_adc16, matches_adc17, matches_adc18, matches_adc19, matches_adc20, matches_adc21, matches_adc22, matches_adc23, matches_adc24, matches_adc25, matches_adc26, matches_adc27, matches_adc28, matches_adcx29, matches_adcx30, matches_add31, matches_add32, matches_add33, matches_add34, matches_add35, matches_add36, matches_add37, matches_add38, matches_add39, matches_add40, matches_add41, matches_add42, matches_add43, matches_add44, matches_add45, matches_add46, matches_add47, matches_add48, matches_add49, matches_add50, matches_add51, matches_add52, matches_adox53, matches_adox54, matches_and55, matches_and56, matches_and57, matches_and58, matches_and59, matches_and60, matches_and61, matches_and62, matches_and63, matches_and64, matches_and65, matches_and66, matches_and67, matches_and68, matches_and69, matches_and70, matches_and71, matches_and72, matches_and73, matches_and74, matches_and75, matches_and76, matches_arpl77, matches_bsf78, matches_bsf79, matches_bsf80, matches_bsr81, matches_bsr82, matches_bsr83, matches_bswap84, matches_bswap85, matches_bt86, matches_bt87, matches_bt88, matches_bt89, matches_bt90, matches_bt91, matches_btc92, matches_btc93, matches_btc94, matches_btc95, matches_btc96, matches_btc97, matches_btr98, matches_btr99, matches_btr100, matches_btr101, matches_btr102, matches_btr103, matches_bts104, matches_bts105, matches_bts106, matches_bts107, matches_bts108, matches_bts109, matches_call110, matches_call111, matches_call112, matches_cbw113, matches_cwde114, matches_cdqe115, matches_clc116, matches_cld117, matches_cli118, matches_clts119, matches_cmc120, matches_cmova121, matches_cmova122, matches_cmova123, matches_cmovae124, matches_cmovae125, matches_cmovae126, matches_cmovb127, matches_cmovb128, matches_cmovb129, matches_cmovbe130, matches_cmovbe131, matches_cmovbe132, matches_cmovc133, matches_cmovc134, matches_cmovc135, matches_cmove136, matches_cmove137, matches_cmove138, matches_cmovg139, matches_cmovg140, matches_cmovg141, matches_cmovge142, matches_cmovge143, matches_cmovge144, matches_cmovl145, matches_cmovl146, matches_cmovl147, matches_cmovle148, matches_cmovle149, matches_cmovle150, matches_cmovna151, matches_cmovna152, matches_cmovna153, matches_cmovnae154, matches_cmovnae155, matches_cmovnae156, matches_cmovnb157, matches_cmovnb158, matches_cmovnb159, matches_cmovnbe160, matches_cmovnbe161, matches_cmovnbe162, matches_cmovnc163, matches_cmovnc164, matches_cmovnc165, matches_cmovne166, matches_cmovne167, matches_cmovne168, matches_cmovng169, matches_cmovng170, matches_cmovng171, matches_cmovnge172, matches_cmovnge173, matches_cmovnge174, matches_cmovnl175, matches_cmovnl176, matches_cmovnl177, matches_cmovnle178, matches_cmovnle179, matches_cmovnle180, matches_cmovno181, matches_cmovno182, matches_cmovno183, matches_cmovnp184, matches_cmovnp185, matches_cmovnp186, matches_cmovns187, matches_cmovns188, matches_cmovns189, matches_cmovnz190, matches_cmovnz191, matches_cmovnz192, matches_cmovo193, matches_cmovo194, matches_cmovo195, matches_cmovp196, matches_cmovp197, matches_cmovp198, matches_cmovpe199, matches_cmovpe200, matches_cmovpe201, matches_cmp202, matches_cmp203, matches_cmp204, matches_cmp205, matches_cmp206, matches_cmp207, matches_cmp208, matches_cmp209, matches_cmp210, matches_cmp211, matches_cmp212, matches_cmp213, matches_cmp214, matches_cmp215, matches_cmp216, matches_cmp217, matches_cmp218, matches_cmp219, matches_cmp220, matches_cmp221, matches_cmp222, matches_cmp223, matches_cmpsb224, matches_cmpsw225, matches_cmpsd226, matches_cmpsq227, matches_cmpxchg228, matches_cmpxchg229, matches_cmpxchg230, matches_cmpxchg231, matches_cmpxchg232, matches_cpuid233, matches_crc32234, matches_crc32235, matches_crc32236, matches_crc32237, matches_crc32238, matches_crc32239, matches_cwd240, matches_cdq241, matches_cqo242, matches_daa243, matches_das244, matches_dec245, matches_dec246, matches_dec247, matches_dec248, matches_dec249, matches_dec250, matches_dec251, matches_div252, matches_div253, matches_div254, matches_div255, matches_div256, matches_enter257, matches_enter258, matches_enter259, matches_f2xm1260, matches_fabs261, matches_faddp262, matches_fchs263, matches_fclex264, matches_fnclex265, matches_fcom266, matches_fcomp267, matches_fcompp268, matches_fcos269, matches_fdecstp270, matches_fdivp271, matches_fdivrp272, matches_fincstp273, matches_finit274, matches_fninit275, matches_fld1276, matches_fldl2t277, matches_fldl2e278, matches_fldpi279, matches_fldlg2280, matches_fldln2281, matches_fldz282, matches_fmulp283, matches_fnop284, matches_fpatan285, matches_fprem286, matches_fprem1287, matches_fptan288, matches_frndint289, matches_fscale290, matches_fsin291, matches_fsincos292, matches_fsqrt293, matches_fstsw294, matches_fnstsw295, matches_fsubp296, matches_fsubrp297, matches_ftst298, matches_fucom299, matches_fucomp300, matches_fucompp301, matches_fxam302, matches_fxch303, matches_fxtract304, matches_fyl2x305, matches_fyl2xp1306, matches_hlt307, matches_idiv308, matches_idiv309, matches_idiv310, matches_idiv311, matches_idiv312, matches_imul313, matches_imul314, matches_imul315, matches_imul316, matches_imul317, matches_imul318, matches_imul319, matches_in320, matches_in321, matches_in322, matches_in323, matches_in324, matches_in325, matches_inc326, matches_inc327, matches_inc328, matches_inc329, matches_inc330, matches_inc331, matches_inc332, matches_insb333, matches_insw334, matches_insd335, matches_int336, matches_into337, matches_invd338, matches_iret339, matches_iretd340, matches_iretq341, matches_jmp342, matches_jmp343, matches_jmp344, matches_lahf345, matches_leave346, matches_leave347, matches_leave348, matches_lldt349, matches_lmsw350, matches_lock351, matches_lodsb352, matches_lodsw353, matches_lodsd354, matches_lodsq355, matches_ltr356, matches_lzcnt357, matches_lzcnt358, matches_lzcnt359, matches_monitor360, matches_mov361, matches_mov362, matches_mov363, matches_mov364, matches_mov365, matches_mov366, matches_mov367, matches_mov368, matches_mov369, matches_mov370, matches_mov371, matches_mov372, matches_mov373, matches_mov374, matches_mov375, matches_mov376, matches_mov377, matches_mov378, matches_mov379, matches_mov380, matches_movsb381, matches_movsw382, matches_movsd383, matches_movsq384, matches_movsx385, matches_movsx386, matches_movsx387, matches_movsx388, matches_movsx389, matches_movsxd390, matches_movzx391, matches_movzx392, matches_movzx393, matches_movzx394, matches_movzx395, matches_mul396, matches_mul397, matches_mul398, matches_mul399, matches_mul400, matches_mwait401, matches_neg402, matches_neg403, matches_neg404, matches_neg405, matches_neg406, matches_not407, matches_not408, matches_not409, matches_not410, matches_not411, matches_or412, matches_or413, matches_or414, matches_or415, matches_or416, matches_or417, matches_or418, matches_or419, matches_or420, matches_or421, matches_or422, matches_or423, matches_or424, matches_or425, matches_or426, matches_or427, matches_or428, matches_or429, matches_or430, matches_or431, matches_or432, matches_or433, matches_out434, matches_out435, matches_out436, matches_out437, matches_out438, matches_out439, matches_outsb440, matches_outsw441, matches_outsd442, matches_pause443, matches_pop444, matches_pop445, matches_pop446, matches_pop447, matches_pop448, matches_pop449, matches_pop450, matches_pop451, matches_pop452, matches_pop453, matches_pop454, matches_pop455, matches_pop456, matches_pop457, matches_pop458, matches_popa459, matches_popad460, matches_popcnt461, matches_popcnt462, matches_popcnt463, matches_popf464, matches_popfd465, matches_popfq466, matches_push467, matches_push468, matches_push469, matches_push470, matches_push471, matches_push472, matches_push473, matches_push474, matches_push475, matches_push476, matches_push477, matches_push478, matches_push479, matches_push480, matches_push481, matches_pusha482, matches_pushad483, matches_pushf484, matches_pushfd485, matches_pushfq486, matches_rcl487, matches_rcl488, matches_rcl489, matches_rcl490, matches_rcl491, matches_rcl492, matches_rcl493, matches_rcl494, matches_rcl495, matches_rcl496, matches_rcl497, matches_rcl498, matches_rcl499, matches_rcl500, matches_rcl501, matches_rcr502, matches_rcr503, matches_rcr504, matches_rcr505, matches_rcr506, matches_rcr507, matches_rcr508, matches_rcr509, matches_rcr510, matches_rcr511, matches_rcr512, matches_rcr513, matches_rcr514, matches_rcr515, matches_rcr516, matches_rol517, matches_rol518, matches_rol519, matches_rol520, matches_rol521, matches_rol522, matches_rol523, matches_rol524, matches_rol525, matches_rol526, matches_rol527, matches_rol528, matches_rol529, matches_rol530, matches_rol531, matches_ror532, matches_ror533, matches_ror534, matches_ror535, matches_ror536, matches_ror537, matches_ror538, matches_ror539, matches_ror540, matches_ror541, matches_ror542, matches_ror543, matches_ror544, matches_ror545, matches_ror546, matches_rdfsbase547, matches_rdfsbase548, matches_rdgsbase549, matches_rdgsbase550, matches_rdmsr551, matches_rdpid552, matches_rdpid553, matches_rdpmc554, matches_rdrand555, matches_rdrand556, matches_rdrand557, matches_rdseed558, matches_rdseed559, matches_rdseed560, matches_rdtsc561, matches_rdtscp562, matches_ret563, matches_retf564, matches_ret565, matches_ret566, matches_rsm567, matches_sahf568, matches_sal569, matches_sal570, matches_sal571, matches_sal572, matches_sal573, matches_sal574, matches_sal575, matches_sal576, matches_sal577, matches_sal578, matches_sal579, matches_sal580, matches_sal581, matches_sal582, matches_sal583, matches_sar584, matches_sar585, matches_sar586, matches_sar587, matches_sar588, matches_sar589, matches_sar590, matches_sar591, matches_sar592, matches_sar593, matches_sar594, matches_sar595, matches_sar596, matches_sar597, matches_sar598, matches_shl599, matches_shl600, matches_shl601, matches_shl602, matches_shl603, matches_shl604, matches_shl605, matches_shl606, matches_shl607, matches_shl608, matches_shl609, matches_shl610, matches_shl611, matches_shl612, matches_shl613, matches_shr614, matches_shr615, matches_shr616, matches_shr617, matches_shr618, matches_shr619, matches_shr620, matches_shr621, matches_shr622, matches_shr623, matches_shr624, matches_shr625, matches_shr626, matches_shr627, matches_shr628, matches_sbb629, matches_sbb630, matches_sbb631, matches_sbb632, matches_sbb633, matches_sbb634, matches_sbb635, matches_sbb636, matches_sbb637, matches_sbb638, matches_sbb639, matches_sbb640, matches_sbb641, matches_sbb642, matches_sbb643, matches_sbb644, matches_sbb645, matches_sbb646, matches_sbb647, matches_sbb648, matches_sbb649, matches_sbb650, matches_scasb651, matches_scasw652, matches_scasd653, matches_scasq654, matches_seta655, matches_seta656, matches_setae657, matches_setae658, matches_setb659, matches_setb660, matches_setbe661, matches_setbe662, matches_setc663, matches_setc664, matches_sete665, matches_sete666, matches_setg667, matches_setg668, matches_setge669, matches_setge670, matches_setl671, matches_setl672, matches_setle673, matches_setle674, matches_setna675, matches_setna676, matches_setnae677, matches_setnae678, matches_setnb679, matches_setnb680, matches_setnbe681, matches_setnbe682, matches_setnc683, matches_setnc684, matches_setne685, matches_setne686, matches_setng687, matches_setng688, matches_setnge689, matches_setnge690, matches_setnl691, matches_setnl692, matches_setnle693, matches_sldt694, matches_smsw695, matches_stc696, matches_std697, matches_sti698, matches_stosb699, matches_stosw700, matches_stosd701, matches_stosq702, matches_str703, matches_sub704, matches_sub705, matches_sub706, matches_sub707, matches_sub708, matches_sub709, matches_sub710, matches_sub711, matches_sub712, matches_sub713, matches_sub714, matches_sub715, matches_sub716, matches_sub717, matches_sub718, matches_sub719, matches_sub720, matches_sub721, matches_sub722, matches_sub723, matches_sub724, matches_sub725, matches_swapgs726, matches_syscall727, matches_sysenter728, matches_sysexit729, matches_sysexit730, matches_sysret731, matches_sysret732, matches_test733, matches_test734, matches_test735, matches_test736, matches_test737, matches_test738, matches_test739, matches_test740, matches_test741, matches_test742, matches_test743, matches_test744, matches_test745, matches_test746, matches_tzcnt747, matches_tzcnt748, matches_tzcnt749, matches_ud0750, matches_ud1751, matches_ud2752, matches_verr753, matches_verw754, matches_wait755, matches_fwait756, matches_wbinvd757, matches_wrfsbase758, matches_wrfsbase759, matches_wrgsbase760, matches_wrgsbase761, matches_wrmsr762, matches_xabort763, matches_xacquire764, matches_xrelease765, matches_xadd766, matches_xadd767, matches_xadd768, matches_xadd769, matches_xadd770, matches_xchg771, matches_xchg772, matches_xchg773, matches_xchg774, matches_xchg775, matches_xchg776, matches_xchg777, matches_xchg778, matches_xchg779, matches_xchg780, matches_xchg781, matches_xchg782, matches_xchg783, matches_xchg784, matches_xchg785, matches_xchg786, matches_xlatb787, matches_xlatb788, matches_xor789, matches_xor790, matches_xor791, matches_xor792, matches_xor793, matches_xor794, matches_xor795, matches_xor796, matches_xor797, matches_xor798, matches_xor799, matches_xor800, matches_xor801, matches_xor802, matches_xor803, matches_xor804, matches_xor805, matches_xor806, matches_xor807, matches_xor808, matches_xor809, matches_xor810];

pub fn matches(tokens: &Vec<String>) -> Result<Instruction, ParseError> {
    let mut err: (usize, ParseError) = (usize::MAX, ParseError::UnexpectedEOF);
    
    for func in MATCH_FUNCTIONS {
        let instr = func(tokens);
        if instr.is_ok() {
            return Ok(instr.unwrap());
        } else {
            let instr_err = instr.unwrap_err();
            if instr_err.0 < err.0 {
                err = instr_err;
            }
        }
    }
    
    Err(err.1)
}
