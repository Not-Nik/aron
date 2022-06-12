// aron (c) Nikolas Wipper 2022

use std::mem::size_of;
use std::slice::Iter;
use std::str::FromStr;
use crate::instructions::{Instruction, Mod, Register};
use crate::instructions::Mod::*;
    
fn is_imm_of_size(iter: &mut Iter<String>, size: usize) -> Option<i32> {
    let next = iter.next().unwrap();
    let neg = if next == "-" { -1 } else { 1 };
    
    let num = if next == "-" { iter.next().unwrap().parse::<isize>() } else { next.parse::<isize>() };
    if let Ok(num) = num {
        if (size_of::<usize>() * 8 - num.leading_zeros() as usize) <= size {
            Some(num as i32 * neg)
        } else {
            None
        }
    } else {
        None
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

fn is_reg_of_size(iter: &mut Iter<String>, size: usize) -> Option<Register> {
    let reg = iter.next().unwrap();
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
        Some(Register::from_str(reg.as_str()).unwrap())
    } else {
        None
    }
}

fn is_rm_of_size(iter: &mut Iter<String>, size: usize) -> Option<(Register, Mod, Option<i32>)> {
    let reg_res = is_reg_of_size(&mut iter.clone(), size);
    if let Some(reg_res) = reg_res {
        iter.next();
        return Some((reg_res, NoDereference, None));
    }
    
    let next = iter.next().unwrap();
    if match size {
        8 => next != "byte",
        16 => next != "word",
        32 => next != "dword",
        64 => next != "qword",
        _ => panic!("Invalid size"),
    } { return None; }
    if iter.next().unwrap() != "ptr" { return None; }
    if iter.next().unwrap() != "[" { return None; }
    let reg_res = is_reg_of_size(iter, 0);
    if reg_res.is_none() { return None; }
    
    let next = iter.next().unwrap();
    let mod_byte: Mod;
    let mut off: Option<i32> = None;
    
    if ["+", "-"].contains(&next.as_str()) {
        let off_res = is_imm_of_size(iter, 32);
        if let Some(off_res) = off_res {
            off = Some(off_res * if next.as_str() == "-" { -1 } else { 1 })
        } else {
            return None;
        }
        if iter.next().unwrap() != "]" { return None; }
        
        mod_byte = Offset32Bit;
    } else if next != "]" {
        return None;
    } else {
        mod_byte = NoOffset;
    }
    return Some((reg_res.unwrap(), mod_byte, off));
}

fn matches_aaa1(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "aaa" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("aaa".to_string());
    instr.write_byte(0x37);

    Some(instr)
}

fn matches_aad2(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "aad" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("aad".to_string());
    instr.write_byte(0xD5);
    instr.write_byte(0x0A);

    Some(instr)
}

fn matches_aad3(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "aad" { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("aad".to_string());
    instr.write_byte(0xD5);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_aam4(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "aam" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("aam".to_string());
    instr.write_byte(0xD4);
    instr.write_byte(0x0A);

    Some(instr)
}

fn matches_aam5(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "aam" { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("aam".to_string());
    instr.write_byte(0xD4);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_aas6(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "aas" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("aas".to_string());
    instr.write_byte(0x3F);

    Some(instr)
}

fn matches_adc7(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());
    instr.write_byte(0x14);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_adc8(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());
    instr.write_byte(0x15);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_adc9(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());
    instr.write_byte(0x15);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_adc10(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x15);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_adc11(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_adc12(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 2 as u8);
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_adc13(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_adc14(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_adc15(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_adc16(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_adc17(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_adc18(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_adc19(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x10);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adc20(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x10);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adc21(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x11);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adc22(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x11);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adc23(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x11);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adc24(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x12);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adc25(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x12);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adc26(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x13);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adc27(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x13);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adc28(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adc" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x13);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adcx29(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adcx" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adcx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x66);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adcx30(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adcx" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adcx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x66);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_add31(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());
    instr.write_byte(0x04);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_add32(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());
    instr.write_byte(0x05);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_add33(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());
    instr.write_byte(0x05);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_add34(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x05);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_add35(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_add36(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_add37(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_add38(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_add39(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_add40(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_add41(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_add42(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_add43(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x00);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_add44(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x00);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_add45(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x01);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_add46(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x01);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_add47(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x01);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_add48(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x02);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_add49(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x02);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_add50(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x03);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_add51(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x03);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_add52(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "add" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("add".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x03);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adox53(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adox" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adox".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_adox54(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "adox" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("adox".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_and55(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());
    instr.write_byte(0x24);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_and56(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());
    instr.write_byte(0x25);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_and57(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());
    instr.write_byte(0x25);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_and58(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x25);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_and59(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_and60(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_and61(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_and62(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_and63(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_and64(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_and65(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_and66(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_and67(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x20);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_and68(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x20);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_and69(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x21);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_and70(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x21);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_and71(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x21);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_and72(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x22);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_and73(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x22);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_and74(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x23);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_and75(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x23);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_and76(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "and" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("and".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x23);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_arpl77(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "arpl" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("arpl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x63);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bsf78(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bsf" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bsf".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bsf79(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bsf" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bsf".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bsf80(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bsf" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bsf".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bsr81(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bsr" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bsr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bsr82(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bsr" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bsr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bsr83(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bsr" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bsr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bswap84(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bswap" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bswap".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xC8+reg as u8);

    Some(instr)
}

fn matches_bswap85(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bswap" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bswap".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xC8+reg as u8);

    Some(instr)
}

fn matches_bt86(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bt" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xA3);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bt87(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bt" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xA3);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bt88(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bt" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xA3);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bt89(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bt" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_bt90(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bt" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_bt91(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bt" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_btc92(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btc" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBB);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_btc93(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btc" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBB);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_btc94(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btc" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBB);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_btc95(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btc" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_btc96(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btc" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_btc97(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btc" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_btr98(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btr" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xB3);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_btr99(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btr" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xB3);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_btr100(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btr" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB3);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_btr101(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btr" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_btr102(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btr" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_btr103(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "btr" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("btr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 6 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_bts104(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bts" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bts".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xAB);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bts105(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bts" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bts".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xAB);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bts106(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bts" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bts".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAB);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_bts107(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bts" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bts".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_bts108(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bts" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bts".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_bts109(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "bts" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("bts".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBA);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_call110(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "call" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("call".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_call111(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "call" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("call".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_call112(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "call" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("call".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cbw113(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cbw" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cbw".to_string());
    instr.write_byte(0x98);

    Some(instr)
}

fn matches_cwde114(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cwde" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cwde".to_string());
    instr.write_byte(0x98);

    Some(instr)
}

fn matches_cdqe115(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cdqe" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cdqe".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x98);

    Some(instr)
}

fn matches_clc116(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "clc" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("clc".to_string());
    instr.write_byte(0xF8);

    Some(instr)
}

fn matches_cld117(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cld" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cld".to_string());
    instr.write_byte(0xFC);

    Some(instr)
}

fn matches_cli118(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cli" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cli".to_string());
    instr.write_byte(0xFA);

    Some(instr)
}

fn matches_clts119(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "clts" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("clts".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x06);

    Some(instr)
}

fn matches_cmc120(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmc" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmc".to_string());
    instr.write_byte(0xF5);

    Some(instr)
}

fn matches_cmova121(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmova" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmova".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmova122(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmova" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmova".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmova123(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmova" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmova".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovae124(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovae" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovae".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovae125(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovae" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovae".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovae126(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovae" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovae".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovb127(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovb" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovb128(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovb" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovb129(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovb" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovbe130(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovbe" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovbe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovbe131(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovbe" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovbe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovbe132(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovbe" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovbe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovc133(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovc" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovc134(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovc" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovc135(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovc" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmove136(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmove" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmove".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x44);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmove137(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmove" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmove".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x44);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmove138(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmove" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmove".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x44);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovg139(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovg" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovg140(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovg" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovg141(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovg" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovge142(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovge" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovge".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovge143(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovge" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovge".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovge144(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovge" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovge".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovl145(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovl" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovl146(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovl" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovl147(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovl" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovle148(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovle" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovle".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovle149(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovle" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovle".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovle150(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovle" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovle".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovna151(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovna" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovna".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovna152(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovna" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovna".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovna153(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovna" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovna".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x46);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnae154(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnae" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnae".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnae155(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnae" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnae".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnae156(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnae" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnae".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x42);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnb157(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnb" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnb158(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnb" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnb159(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnb" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnbe160(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnbe" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnbe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnbe161(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnbe" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnbe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnbe162(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnbe" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnbe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x47);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnc163(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnc" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnc164(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnc" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnc165(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnc" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x43);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovne166(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovne" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovne".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovne167(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovne" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovne".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovne168(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovne" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovne".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovng169(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovng" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovng".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovng170(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovng" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovng".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovng171(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovng" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovng".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4E);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnge172(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnge" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnge".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnge173(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnge" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnge".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnge174(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnge" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnge".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4C);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnl175(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnl" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnl176(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnl" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnl177(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnl" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4D);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnle178(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnle" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnle".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnle179(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnle" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnle".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnle180(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnle" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnle".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4F);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovno181(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovno" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovno".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x41);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovno182(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovno" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovno".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x41);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovno183(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovno" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovno".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x41);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnp184(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnp" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnp185(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnp" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnp186(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnp" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovns187(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovns" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovns".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x49);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovns188(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovns" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovns".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x49);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovns189(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovns" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovns".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x49);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnz190(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnz" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnz".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnz191(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnz" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnz".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovnz192(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovnz" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovnz".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x45);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovo193(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovo" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovo".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x40);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovo194(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovo" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovo".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x40);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovo195(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovo" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovo".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x40);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovp196(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovp" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovp197(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovp" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovp198(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovp" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovpe199(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovpe" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovpe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovpe200(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovpe" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovpe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmovpe201(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmovpe" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmovpe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x4A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmp202(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());
    instr.write_byte(0x3C);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_cmp203(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());
    instr.write_byte(0x3D);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_cmp204(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());
    instr.write_byte(0x3D);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_cmp205(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x3D);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_cmp206(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_cmp207(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 7 as u8);
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_cmp208(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_cmp209(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_cmp210(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_cmp211(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_cmp212(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_cmp213(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_cmp214(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x38);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmp215(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x38);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmp216(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x39);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmp217(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x39);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmp218(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x39);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmp219(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x3A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmp220(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x3A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmp221(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x3B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmp222(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x3B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmp223(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmp" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x3B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmpsb224(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmpsb" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmpsb".to_string());
    instr.write_byte(0xA6);

    Some(instr)
}

fn matches_cmpsw225(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmpsw" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmpsw".to_string());
    instr.write_byte(0xA7);

    Some(instr)
}

fn matches_cmpsd226(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmpsd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmpsd".to_string());
    instr.write_byte(0xA7);

    Some(instr)
}

fn matches_cmpsq227(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmpsq" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmpsq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xA7);

    Some(instr)
}

fn matches_cmpxchg228(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmpxchg" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmpxchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xB0);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmpxchg229(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmpxchg" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmpxchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB0);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmpxchg230(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmpxchg" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmpxchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xB1);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmpxchg231(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmpxchg" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmpxchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xB1);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cmpxchg232(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cmpxchg" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cmpxchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB1);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cpuid233(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cpuid" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cpuid".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA2);

    Some(instr)
}

fn matches_crc32234(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "crc32" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("crc32".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF2);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF0);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_crc32235(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "crc32" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("crc32".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF2);
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF0);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_crc32236(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "crc32" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("crc32".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF2);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF1);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_crc32237(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "crc32" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("crc32".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF2);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF1);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_crc32238(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "crc32" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("crc32".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF2);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF0);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_crc32239(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "crc32" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("crc32".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF2);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x38);
    instr.write_byte(0xF1);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_cwd240(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cwd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cwd".to_string());
    instr.write_byte(0x99);

    Some(instr)
}

fn matches_cdq241(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cdq" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cdq".to_string());
    instr.write_byte(0x99);

    Some(instr)
}

fn matches_cqo242(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "cqo" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("cqo".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x99);

    Some(instr)
}

fn matches_daa243(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "daa" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("daa".to_string());
    instr.write_byte(0x27);

    Some(instr)
}

fn matches_das244(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "das" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("das".to_string());
    instr.write_byte(0x2F);

    Some(instr)
}

fn matches_dec245(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "dec" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("dec".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFE);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_dec246(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "dec" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("dec".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 1 as u8);
    instr.write_byte(0xFE);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_dec247(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "dec" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("dec".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_dec248(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "dec" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("dec".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_dec249(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "dec" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("dec".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_dec250(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "dec" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("dec".to_string());
    instr.write_byte(0x48+reg as u8);

    Some(instr)
}

fn matches_dec251(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "dec" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("dec".to_string());
    instr.write_byte(0x48+reg as u8);

    Some(instr)
}

fn matches_div252(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "div" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("div".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_div253(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "div" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("div".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 6 as u8);
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_div254(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "div" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("div".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_div255(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "div" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("div".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_div256(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "div" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("div".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 6 as u8);
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_enter257(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "enter" { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "0" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("enter".to_string());
    instr.write_byte(0xC8);
    instr.write_byte(0x00);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_enter258(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "enter" { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("enter".to_string());
    instr.write_byte(0xC8);
    instr.write_byte(0x01);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_enter259(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "enter" { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("enter".to_string());
    instr.write_byte(0xC8);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_f2xm1260(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "f2xm1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("f2xm1".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF0);

    Some(instr)
}

fn matches_fabs261(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fabs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fabs".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE1);

    Some(instr)
}

fn matches_faddp262(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "faddp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("faddp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xC1);

    Some(instr)
}

fn matches_fchs263(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fchs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fchs".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE0);

    Some(instr)
}

fn matches_fclex264(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fclex" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fclex".to_string());
    instr.write_byte(0x9B);
    instr.write_byte(0xDB);
    instr.write_byte(0xE2);

    Some(instr)
}

fn matches_fnclex265(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fnclex" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fnclex".to_string());
    instr.write_byte(0xDB);
    instr.write_byte(0xE2);

    Some(instr)
}

fn matches_fcom266(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fcom" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fcom".to_string());
    instr.write_byte(0xD8);
    instr.write_byte(0xD1);

    Some(instr)
}

fn matches_fcomp267(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fcomp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fcomp".to_string());
    instr.write_byte(0xD8);
    instr.write_byte(0xD9);

    Some(instr)
}

fn matches_fcompp268(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fcompp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fcompp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xD9);

    Some(instr)
}

fn matches_fcos269(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fcos" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fcos".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFF);

    Some(instr)
}

fn matches_fdecstp270(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fdecstp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fdecstp".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF6);

    Some(instr)
}

fn matches_fdivp271(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fdivp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fdivp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xF9);

    Some(instr)
}

fn matches_fdivrp272(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fdivrp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fdivrp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xF1);

    Some(instr)
}

fn matches_fincstp273(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fincstp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fincstp".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF7);

    Some(instr)
}

fn matches_finit274(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "finit" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("finit".to_string());
    instr.write_byte(0x9B);
    instr.write_byte(0xDB);
    instr.write_byte(0xE3);

    Some(instr)
}

fn matches_fninit275(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fninit" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fninit".to_string());
    instr.write_byte(0xDB);
    instr.write_byte(0xE3);

    Some(instr)
}

fn matches_fld1276(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fld1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fld1".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE8);

    Some(instr)
}

fn matches_fldl2t277(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fldl2t" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fldl2t".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE9);

    Some(instr)
}

fn matches_fldl2e278(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fldl2e" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fldl2e".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xEA);

    Some(instr)
}

fn matches_fldpi279(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fldpi" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fldpi".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xEB);

    Some(instr)
}

fn matches_fldlg2280(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fldlg2" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fldlg2".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xEC);

    Some(instr)
}

fn matches_fldln2281(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fldln2" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fldln2".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xED);

    Some(instr)
}

fn matches_fldz282(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fldz" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fldz".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xEE);

    Some(instr)
}

fn matches_fmulp283(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fmulp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fmulp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xC9);

    Some(instr)
}

fn matches_fnop284(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fnop" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fnop".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xD0);

    Some(instr)
}

fn matches_fpatan285(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fpatan" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fpatan".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF3);

    Some(instr)
}

fn matches_fprem286(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fprem" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fprem".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF8);

    Some(instr)
}

fn matches_fprem1287(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fprem1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fprem1".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF5);

    Some(instr)
}

fn matches_fptan288(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fptan" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fptan".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF2);

    Some(instr)
}

fn matches_frndint289(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "frndint" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("frndint".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFC);

    Some(instr)
}

fn matches_fscale290(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fscale" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fscale".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFD);

    Some(instr)
}

fn matches_fsin291(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fsin" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fsin".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFE);

    Some(instr)
}

fn matches_fsincos292(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fsincos" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fsincos".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFB);

    Some(instr)
}

fn matches_fsqrt293(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fsqrt" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fsqrt".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xFA);

    Some(instr)
}

fn matches_fstsw294(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fstsw" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fstsw".to_string());
    instr.write_byte(0x9B);
    instr.write_byte(0xDF);
    instr.write_byte(0xE0);

    Some(instr)
}

fn matches_fnstsw295(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fnstsw" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fnstsw".to_string());
    instr.write_byte(0xDF);
    instr.write_byte(0xE0);

    Some(instr)
}

fn matches_fsubp296(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fsubp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fsubp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xE9);

    Some(instr)
}

fn matches_fsubrp297(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fsubrp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fsubrp".to_string());
    instr.write_byte(0xDE);
    instr.write_byte(0xE1);

    Some(instr)
}

fn matches_ftst298(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ftst" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ftst".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE4);

    Some(instr)
}

fn matches_fucom299(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fucom" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fucom".to_string());
    instr.write_byte(0xDD);
    instr.write_byte(0xE1);

    Some(instr)
}

fn matches_fucomp300(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fucomp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fucomp".to_string());
    instr.write_byte(0xDD);
    instr.write_byte(0xE9);

    Some(instr)
}

fn matches_fucompp301(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fucompp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fucompp".to_string());
    instr.write_byte(0xDA);
    instr.write_byte(0xE9);

    Some(instr)
}

fn matches_fxam302(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fxam" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fxam".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xE5);

    Some(instr)
}

fn matches_fxch303(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fxch" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fxch".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xC9);

    Some(instr)
}

fn matches_fxtract304(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fxtract" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fxtract".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF4);

    Some(instr)
}

fn matches_fyl2x305(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fyl2x" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fyl2x".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF1);

    Some(instr)
}

fn matches_fyl2xp1306(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fyl2xp1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fyl2xp1".to_string());
    instr.write_byte(0xD9);
    instr.write_byte(0xF9);

    Some(instr)
}

fn matches_hlt307(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "hlt" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("hlt".to_string());
    instr.write_byte(0xF4);

    Some(instr)
}

fn matches_idiv308(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "idiv" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("idiv".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_idiv309(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "idiv" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("idiv".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 7 as u8);
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_idiv310(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "idiv" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("idiv".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_idiv311(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "idiv" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("idiv".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_idiv312(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "idiv" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("idiv".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_imul313(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "imul" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("imul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_imul314(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "imul" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("imul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_imul315(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "imul" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("imul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_imul316(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "imul" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("imul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_imul317(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "imul" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("imul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xAF);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_imul318(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "imul" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("imul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xAF);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_imul319(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "imul" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("imul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAF);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_in320(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "in" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xE4);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_in321(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "in" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xE5);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_in322(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "in" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xE5);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_in323(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "in" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "dx" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xEC);

    Some(instr)
}

fn matches_in324(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "in" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "dx" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xED);

    Some(instr)
}

fn matches_in325(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "in" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "dx" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("in".to_string());
    instr.write_byte(0xED);

    Some(instr)
}

fn matches_inc326(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "inc" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("inc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFE);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_inc327(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "inc" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("inc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xFE);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_inc328(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "inc" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("inc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_inc329(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "inc" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("inc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_inc330(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "inc" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("inc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_inc331(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "inc" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("inc".to_string());
    instr.write_byte(0x40+reg as u8);

    Some(instr)
}

fn matches_inc332(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "inc" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("inc".to_string());
    instr.write_byte(0x40+reg as u8);

    Some(instr)
}

fn matches_insb333(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "insb" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("insb".to_string());
    instr.write_byte(0x6C);

    Some(instr)
}

fn matches_insw334(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "insw" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("insw".to_string());
    instr.write_byte(0x6D);

    Some(instr)
}

fn matches_insd335(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "insd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("insd".to_string());
    instr.write_byte(0x6D);

    Some(instr)
}

fn matches_int336(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "int" { return None; }
    if iter.next().unwrap() != "3" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("int".to_string());
    instr.write_byte(0xCC);

    Some(instr)
}

fn matches_into337(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "into" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("into".to_string());
    instr.write_byte(0xCE);

    Some(instr)
}

fn matches_invd338(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "invd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("invd".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x08);

    Some(instr)
}

fn matches_iret339(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "iret" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("iret".to_string());
    instr.write_byte(0xCF);

    Some(instr)
}

fn matches_iretd340(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "iretd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("iretd".to_string());
    instr.write_byte(0xCF);

    Some(instr)
}

fn matches_iretq341(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "iretq" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("iretq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xCF);

    Some(instr)
}

fn matches_jmp342(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "jmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("jmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_jmp343(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "jmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("jmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_jmp344(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "jmp" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("jmp".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_lahf345(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lahf" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lahf".to_string());
    instr.write_byte(0x9F);

    Some(instr)
}

fn matches_leave346(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "leave" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("leave".to_string());
    instr.write_byte(0xC9);

    Some(instr)
}

fn matches_leave347(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "leave" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("leave".to_string());
    instr.write_byte(0xC9);

    Some(instr)
}

fn matches_leave348(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "leave" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("leave".to_string());
    instr.write_byte(0xC9);

    Some(instr)
}

fn matches_lldt349(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lldt" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lldt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_lmsw350(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lmsw" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lmsw".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_lock351(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lock" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lock".to_string());
    instr.write_byte(0xF0);

    Some(instr)
}

fn matches_lodsb352(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lodsb" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lodsb".to_string());
    instr.write_byte(0xAC);

    Some(instr)
}

fn matches_lodsw353(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lodsw" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lodsw".to_string());
    instr.write_byte(0xAD);

    Some(instr)
}

fn matches_lodsd354(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lodsd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lodsd".to_string());
    instr.write_byte(0xAD);

    Some(instr)
}

fn matches_lodsq355(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lodsq" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lodsq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xAD);

    Some(instr)
}

fn matches_ltr356(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ltr" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ltr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_lzcnt357(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lzcnt" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lzcnt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_lzcnt358(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lzcnt" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lzcnt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_lzcnt359(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "lzcnt" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("lzcnt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBD);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_monitor360(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "monitor" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("monitor".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_byte(0xC8);

    Some(instr)
}

fn matches_mov361(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x88);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mov362(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x88);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mov363(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x89);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mov364(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x89);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mov365(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x89);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mov366(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x8A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mov367(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x8A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mov368(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x8B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mov369(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x8B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mov370(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x8B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mov371(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());
    instr.write_byte(0xB0+reg as u8);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_mov372(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());
    instr.write_rex(false, 0 as u8, reg as u8);
    instr.write_byte(0xB0+reg as u8);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_mov373(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());
    instr.write_byte(0xB8+reg as u8);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_mov374(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());
    instr.write_byte(0xB8+reg as u8);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_mov375(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 64);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0xB8+reg as u8);
    instr.write_num(imm as i64);

    Some(instr)
}

fn matches_mov376(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC6);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_mov377(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xC6);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_mov378(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC7);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_mov379(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC7);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_mov380(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mov" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mov".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xC7);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_movsb381(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movsb" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movsb".to_string());
    instr.write_byte(0xA4);

    Some(instr)
}

fn matches_movsw382(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movsw" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movsw".to_string());
    instr.write_byte(0xA5);

    Some(instr)
}

fn matches_movsd383(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movsd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movsd".to_string());
    instr.write_byte(0xA5);

    Some(instr)
}

fn matches_movsq384(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movsq" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movsq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xA5);

    Some(instr)
}

fn matches_movsx385(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movsx" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movsx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBE);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_movsx386(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movsx" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movsx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBE);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_movsx387(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movsx" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movsx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBE);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_movsx388(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movsx" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movsx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xBF);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_movsx389(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movsx" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movsx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBF);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_movsxd390(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movsxd" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movsxd".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x63);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_movzx391(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movzx" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movzx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xB6);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_movzx392(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movzx" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movzx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xB6);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_movzx393(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movzx" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movzx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB6);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_movzx394(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movzx" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movzx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xB7);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_movzx395(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "movzx" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("movzx".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB7);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mul396(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mul" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mul397(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mul" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mul398(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mul" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mul399(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mul" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mul400(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mul" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mul".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_mwait401(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "mwait" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("mwait".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_byte(0xC9);

    Some(instr)
}

fn matches_neg402(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "neg" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("neg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_neg403(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "neg" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("neg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 3 as u8);
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_neg404(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "neg" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("neg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_neg405(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "neg" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("neg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_neg406(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "neg" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("neg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_not407(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "not" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("not".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_not408(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "not" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("not".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 2 as u8);
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_not409(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "not" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("not".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_not410(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "not" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("not".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_not411(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "not" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("not".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_or412(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());
    instr.write_byte(0x0C);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_or413(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());
    instr.write_byte(0x0D);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_or414(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());
    instr.write_byte(0x0D);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_or415(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x0D);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_or416(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_or417(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 1 as u8);
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_or418(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_or419(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_or420(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_or421(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_or422(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_or423(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_or424(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x08);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_or425(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x08);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_or426(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x09);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_or427(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x09);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_or428(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x09);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_or429(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_or430(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_or431(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_or432(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_or433(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "or" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("or".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_out434(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "out" { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xE6);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_out435(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "out" { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xE7);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_out436(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "out" { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xE7);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_out437(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "out" { return None; }
    if iter.next().unwrap() != "dx" { return None; }
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xEE);

    Some(instr)
}

fn matches_out438(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "out" { return None; }
    if iter.next().unwrap() != "dx" { return None; }
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xEF);

    Some(instr)
}

fn matches_out439(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "out" { return None; }
    if iter.next().unwrap() != "dx" { return None; }
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("out".to_string());
    instr.write_byte(0xEF);

    Some(instr)
}

fn matches_outsb440(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "outsb" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("outsb".to_string());
    instr.write_byte(0x6E);

    Some(instr)
}

fn matches_outsw441(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "outsw" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("outsw".to_string());
    instr.write_byte(0x6F);

    Some(instr)
}

fn matches_outsd442(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "outsd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("outsd".to_string());
    instr.write_byte(0x6F);

    Some(instr)
}

fn matches_pause443(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pause" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pause".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x90);

    Some(instr)
}

fn matches_pop444(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x8F);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_pop445(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x8F);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_pop446(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x8F);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_pop447(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x58+reg as u8);

    Some(instr)
}

fn matches_pop448(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x58+reg as u8);

    Some(instr)
}

fn matches_pop449(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x58+reg as u8);

    Some(instr)
}

fn matches_pop450(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    if iter.next().unwrap() != "ds" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x1F);

    Some(instr)
}

fn matches_pop451(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    if iter.next().unwrap() != "es" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x07);

    Some(instr)
}

fn matches_pop452(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    if iter.next().unwrap() != "ss" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x17);

    Some(instr)
}

fn matches_pop453(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    if iter.next().unwrap() != "fs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA1);

    Some(instr)
}

fn matches_pop454(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    if iter.next().unwrap() != "fs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA1);

    Some(instr)
}

fn matches_pop455(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    if iter.next().unwrap() != "fs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA1);

    Some(instr)
}

fn matches_pop456(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    if iter.next().unwrap() != "gs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA9);

    Some(instr)
}

fn matches_pop457(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    if iter.next().unwrap() != "gs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA9);

    Some(instr)
}

fn matches_pop458(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pop" { return None; }
    if iter.next().unwrap() != "gs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pop".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA9);

    Some(instr)
}

fn matches_popa459(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "popa" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("popa".to_string());
    instr.write_byte(0x61);

    Some(instr)
}

fn matches_popad460(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "popad" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("popad".to_string());
    instr.write_byte(0x61);

    Some(instr)
}

fn matches_popcnt461(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "popcnt" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("popcnt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xB8);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_popcnt462(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "popcnt" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("popcnt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xB8);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_popcnt463(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "popcnt" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("popcnt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xB8);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_popf464(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "popf" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("popf".to_string());
    instr.write_byte(0x9D);

    Some(instr)
}

fn matches_popfd465(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "popfd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("popfd".to_string());
    instr.write_byte(0x9D);

    Some(instr)
}

fn matches_popfq466(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "popfq" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("popfq".to_string());
    instr.write_byte(0x9D);

    Some(instr)
}

fn matches_push467(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_push468(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_push469(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xFF);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_push470(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x50+reg as u8);

    Some(instr)
}

fn matches_push471(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x50+reg as u8);

    Some(instr)
}

fn matches_push472(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x50+reg as u8);

    Some(instr)
}

fn matches_push473(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x6A);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_push474(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x68);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_push475(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x68);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_push476(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    if iter.next().unwrap() != "cs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x0E);

    Some(instr)
}

fn matches_push477(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    if iter.next().unwrap() != "ss" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x16);

    Some(instr)
}

fn matches_push478(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    if iter.next().unwrap() != "ds" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x1E);

    Some(instr)
}

fn matches_push479(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    if iter.next().unwrap() != "es" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x06);

    Some(instr)
}

fn matches_push480(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    if iter.next().unwrap() != "fs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA0);

    Some(instr)
}

fn matches_push481(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "push" { return None; }
    if iter.next().unwrap() != "gs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("push".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xA8);

    Some(instr)
}

fn matches_pusha482(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pusha" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pusha".to_string());
    instr.write_byte(0x60);

    Some(instr)
}

fn matches_pushad483(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pushad" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pushad".to_string());
    instr.write_byte(0x60);

    Some(instr)
}

fn matches_pushf484(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pushf" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pushf".to_string());
    instr.write_byte(0x9C);

    Some(instr)
}

fn matches_pushfd485(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pushfd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pushfd".to_string());
    instr.write_byte(0x9C);

    Some(instr)
}

fn matches_pushfq486(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "pushfq" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("pushfq".to_string());
    instr.write_byte(0x9C);

    Some(instr)
}

fn matches_rcl487(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcl488(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 2 as u8);
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcl489(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcl490(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 2 as u8);
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcl491(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rcl492(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 2 as u8);
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rcl493(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcl494(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcl495(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rcl496(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcl497(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcl498(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcl499(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcl500(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rcl501(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcl" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 2 as u8);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 2 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rcr502(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcr503(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 3 as u8);
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcr504(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcr505(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 3 as u8);
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcr506(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rcr507(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 3 as u8);
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rcr508(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcr509(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcr510(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rcr511(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcr512(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcr513(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcr514(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rcr515(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rcr516(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rcr" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rcr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rol517(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rol518(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rol519(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rol520(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rol521(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rol522(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rol523(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rol524(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rol525(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rol526(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rol527(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rol528(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rol529(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_rol530(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rol531(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rol" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rol".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_ror532(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ror533(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 1 as u8);
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ror534(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ror535(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 1 as u8);
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ror536(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_ror537(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 1 as u8);
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_ror538(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ror539(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ror540(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_ror541(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ror542(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ror543(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ror544(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ror545(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_ror546(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ror" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ror".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 1 as u8);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_rdfsbase547(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdfsbase" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdfsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Some(instr)
}

fn matches_rdfsbase548(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdfsbase" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdfsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Some(instr)
}

fn matches_rdgsbase549(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdgsbase" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdgsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Some(instr)
}

fn matches_rdgsbase550(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdgsbase" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdgsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Some(instr)
}

fn matches_rdmsr551(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdmsr" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdmsr".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x32);

    Some(instr)
}

fn matches_rdpid552(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdpid" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdpid".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Some(instr)
}

fn matches_rdpid553(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdpid" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdpid".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Some(instr)
}

fn matches_rdpmc554(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdpmc" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdpmc".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x33);

    Some(instr)
}

fn matches_rdrand555(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdrand" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdrand".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Some(instr)
}

fn matches_rdrand556(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdrand" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdrand".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Some(instr)
}

fn matches_rdrand557(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdrand" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdrand".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Some(instr)
}

fn matches_rdseed558(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdseed" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdseed".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Some(instr)
}

fn matches_rdseed559(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdseed" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdseed".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Some(instr)
}

fn matches_rdseed560(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdseed" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdseed".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xC7);

    Some(instr)
}

fn matches_rdtsc561(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdtsc" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdtsc".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x31);

    Some(instr)
}

fn matches_rdtscp562(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rdtscp" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rdtscp".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_byte(0xF9);

    Some(instr)
}

fn matches_ret563(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ret" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ret".to_string());
    instr.write_byte(0xC3);

    Some(instr)
}

fn matches_retf564(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "retf" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("retf".to_string());
    instr.write_byte(0xCB);

    Some(instr)
}

fn matches_ret565(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ret" { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ret".to_string());
    instr.write_byte(0xC2);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_ret566(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ret" { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ret".to_string());
    instr.write_byte(0xCA);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_rsm567(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "rsm" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("rsm".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xAA);

    Some(instr)
}

fn matches_sahf568(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sahf" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sahf".to_string());
    instr.write_byte(0x9E);

    Some(instr)
}

fn matches_sal569(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sal570(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sal571(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sal572(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sal573(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sal574(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sal575(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sal576(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sal577(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sal578(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sal579(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sal580(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sal581(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sal582(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sal583(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sal" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sal".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sar584(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sar585(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 7 as u8);
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sar586(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sar587(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 7 as u8);
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sar588(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sar589(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 7 as u8);
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sar590(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sar591(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sar592(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sar593(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sar594(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sar595(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sar596(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sar597(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sar598(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sar" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sar".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 7 as u8);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 7 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_shl599(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shl600(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shl601(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shl602(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shl603(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_shl604(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 4 as u8);
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_shl605(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shl606(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shl607(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_shl608(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shl609(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shl610(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shl611(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shl612(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_shl613(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shl" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 4 as u8);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_shr614(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shr615(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 5 as u8);
    instr.write_byte(0xD0);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shr616(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shr617(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 5 as u8);
    instr.write_byte(0xD2);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shr618(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_shr619(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 5 as u8);
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_shr620(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shr621(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shr622(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_shr623(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shr624(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "1" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0xD1);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shr625(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shr626(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "cl" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0xD3);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_shr627(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_shr628(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "shr" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("shr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sbb629(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());
    instr.write_byte(0x1C);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sbb630(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());
    instr.write_byte(0x1D);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_sbb631(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());
    instr.write_byte(0x1D);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_sbb632(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x1D);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_sbb633(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sbb634(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 3 as u8);
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sbb635(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_sbb636(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_sbb637(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_sbb638(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sbb639(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sbb640(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 3 as u8);
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 3 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sbb641(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x18);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sbb642(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x18);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sbb643(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x19);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sbb644(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x19);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sbb645(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x19);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sbb646(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x1A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sbb647(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x1A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sbb648(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x1B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sbb649(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x1B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sbb650(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sbb" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sbb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x1B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_scasb651(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "scasb" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("scasb".to_string());
    instr.write_byte(0xAE);

    Some(instr)
}

fn matches_scasw652(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "scasw" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("scasw".to_string());
    instr.write_byte(0xAF);

    Some(instr)
}

fn matches_scasd653(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "scasd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("scasd".to_string());
    instr.write_byte(0xAF);

    Some(instr)
}

fn matches_scasq654(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "scasq" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("scasq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xAF);

    Some(instr)
}

fn matches_seta655(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "seta" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("seta".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x97);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_seta656(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "seta" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("seta".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x97);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setae657(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setae" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setae".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setae658(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setae" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setae".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setb659(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setb" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setb660(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setb" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setbe661(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setbe" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setbe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x96);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setbe662(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setbe" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setbe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x96);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setc663(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setc" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setc664(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setc" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sete665(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sete" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sete".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x94);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sete666(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sete" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sete".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x94);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setg667(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setg" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x9F);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setg668(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setg" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9F);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setge669(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setge" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setge".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x9D);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setge670(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setge" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setge".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9D);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setl671(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x9C);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setl672(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9C);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setle673(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setle" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setle".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x9E);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setle674(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setle" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setle".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9E);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setna675(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setna" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setna".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x96);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setna676(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setna" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setna".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x96);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnae677(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnae" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnae".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnae678(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnae" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnae".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x92);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnb679(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnb" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnb680(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnb" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnb".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnbe681(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnbe" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnbe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x97);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnbe682(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnbe" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnbe".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x97);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnc683(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnc" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnc684(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnc" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnc".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x93);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setne685(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setne" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setne".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x95);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setne686(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setne" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setne".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x95);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setng687(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setng" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setng".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x9E);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setng688(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setng" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setng".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9E);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnge689(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnge" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnge".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x9C);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnge690(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnge" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnge".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9C);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnl691(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x9D);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnl692(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnl" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnl".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x9D);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_setnle693(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "setnle" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("setnle".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x9F);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sldt694(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sldt" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sldt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_smsw695(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "smsw" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("smsw".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_stc696(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "stc" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("stc".to_string());
    instr.write_byte(0xF9);

    Some(instr)
}

fn matches_std697(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "std" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("std".to_string());
    instr.write_byte(0xFD);

    Some(instr)
}

fn matches_sti698(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sti" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sti".to_string());
    instr.write_byte(0xFB);

    Some(instr)
}

fn matches_stosb699(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "stosb" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("stosb".to_string());
    instr.write_byte(0xAA);

    Some(instr)
}

fn matches_stosw700(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "stosw" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("stosw".to_string());
    instr.write_byte(0xAB);

    Some(instr)
}

fn matches_stosd701(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "stosd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("stosd".to_string());
    instr.write_byte(0xAB);

    Some(instr)
}

fn matches_stosq702(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "stosq" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("stosq".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xAB);

    Some(instr)
}

fn matches_str703(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "str" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("str".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_mod(m, rm.0 as u8, 1 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sub704(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());
    instr.write_byte(0x2C);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sub705(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());
    instr.write_byte(0x2D);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_sub706(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());
    instr.write_byte(0x2D);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_sub707(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x2D);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_sub708(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sub709(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 5 as u8);
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sub710(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_sub711(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_sub712(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_sub713(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sub714(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sub715(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 5 as u8);
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_sub716(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x28);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sub717(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x28);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sub718(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x29);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sub719(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x29);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sub720(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x29);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sub721(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x2A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sub722(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x2A);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sub723(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x2B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sub724(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x2B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_sub725(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sub" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sub".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x2B);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_swapgs726(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "swapgs" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("swapgs".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x01);
    instr.write_byte(0xF8);

    Some(instr)
}

fn matches_syscall727(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "syscall" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("syscall".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x05);

    Some(instr)
}

fn matches_sysenter728(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sysenter" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sysenter".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x34);

    Some(instr)
}

fn matches_sysexit729(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sysexit" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sysexit".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x35);

    Some(instr)
}

fn matches_sysexit730(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sysexit" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sysexit".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x35);

    Some(instr)
}

fn matches_sysret731(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sysret" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sysret".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x07);

    Some(instr)
}

fn matches_sysret732(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "sysret" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("sysret".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0x07);

    Some(instr)
}

fn matches_test733(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());
    instr.write_byte(0xA8);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_test734(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());
    instr.write_byte(0xA9);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_test735(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());
    instr.write_byte(0xA9);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_test736(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xA9);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_test737(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_test738(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 0 as u8);
    instr.write_byte(0xF6);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_test739(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_test740(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_test741(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 0 as u8);
    instr.write_byte(0xF7);
    instr.write_mod(m, rm.0 as u8, 0 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_test742(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x84);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_test743(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x84);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_test744(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x85);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_test745(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x85);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_test746(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "test" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("test".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x85);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_tzcnt747(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "tzcnt" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("tzcnt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_tzcnt748(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "tzcnt" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("tzcnt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_tzcnt749(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "tzcnt" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("tzcnt".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0xF3);
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xBC);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ud0750(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ud0" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ud0".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0xFF);

    Some(instr)
}

fn matches_ud1751(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ud1" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ud1".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xB9);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_ud2752(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "ud2" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("ud2".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x0B);

    Some(instr)
}

fn matches_verr753(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "verr" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("verr".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_mod(m, rm.0 as u8, 4 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_verw754(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "verw" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("verw".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0x00);
    instr.write_mod(m, rm.0 as u8, 5 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_wait755(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "wait" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("wait".to_string());
    instr.write_byte(0x9B);

    Some(instr)
}

fn matches_fwait756(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "fwait" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("fwait".to_string());
    instr.write_byte(0x9B);

    Some(instr)
}

fn matches_wbinvd757(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "wbinvd" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("wbinvd".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x09);

    Some(instr)
}

fn matches_wrfsbase758(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "wrfsbase" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("wrfsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Some(instr)
}

fn matches_wrfsbase759(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "wrfsbase" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("wrfsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Some(instr)
}

fn matches_wrgsbase760(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "wrgsbase" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("wrgsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Some(instr)
}

fn matches_wrgsbase761(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "wrgsbase" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("wrgsbase".to_string());
    instr.write_byte(0xF3);
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xAE);

    Some(instr)
}

fn matches_wrmsr762(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "wrmsr" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("wrmsr".to_string());
    instr.write_byte(0x0F);
    instr.write_byte(0x30);

    Some(instr)
}

fn matches_xabort763(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xabort" { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xabort".to_string());
    instr.write_byte(0xC6);
    instr.write_byte(0xF8);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_xacquire764(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xacquire" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xacquire".to_string());
    instr.write_byte(0xF2);

    Some(instr)
}

fn matches_xrelease765(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xrelease" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xrelease".to_string());
    instr.write_byte(0xF3);

    Some(instr)
}

fn matches_xadd766(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xadd" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xadd".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xadd767(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xadd" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xadd".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xC0);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xadd768(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xadd" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xadd".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xadd769(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xadd" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xadd".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x0F);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xadd770(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xadd" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xadd".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x0F);
    instr.write_byte(0xC1);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xchg771(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_byte(0x90+reg as u8);

    Some(instr)
}

fn matches_xchg772(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_byte(0x90+reg as u8);

    Some(instr)
}

fn matches_xchg773(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_byte(0x90+reg as u8);

    Some(instr)
}

fn matches_xchg774(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x90+reg as u8);

    Some(instr)
}

fn matches_xchg775(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_byte(0x90+reg as u8);

    Some(instr)
}

fn matches_xchg776(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());
    instr.write_rex(true, 0 as u8, reg as u8);
    instr.write_byte(0x90+reg as u8);

    Some(instr)
}

fn matches_xchg777(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x86);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xchg778(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x86);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xchg779(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x86);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xchg780(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x86);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xchg781(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x87);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xchg782(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x87);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xchg783(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x87);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xchg784(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x87);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xchg785(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x87);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xchg786(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xchg" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xchg".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x87);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xlatb787(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xlatb" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xlatb".to_string());
    instr.write_byte(0xD7);

    Some(instr)
}

fn matches_xlatb788(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xlatb" { return None; }
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xlatb".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0xD7);

    Some(instr)
}

fn matches_xor789(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    if iter.next().unwrap() != "al" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());
    instr.write_byte(0x34);
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_xor790(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    if iter.next().unwrap() != "ax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());
    instr.write_byte(0x35);
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_xor791(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    if iter.next().unwrap() != "eax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());
    instr.write_byte(0x35);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_xor792(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    if iter.next().unwrap() != "rax" { return None; }
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());
    instr.write_rex(true, 0 as u8, 0 as u8);
    instr.write_byte(0x35);
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_xor793(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_xor794(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, 6 as u8);
    instr.write_byte(0x80);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_xor795(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 16);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i16);

    Some(instr)
}

fn matches_xor796(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_xor797(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 32);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 6 as u8);
    instr.write_byte(0x81);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i32);

    Some(instr)
}

fn matches_xor798(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_xor799(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_xor800(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let imm = is_imm_of_size(&mut iter, 8);
    if imm.is_none() { return None; }
    let imm = imm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, 6 as u8);
    instr.write_byte(0x83);
    instr.write_mod(m, rm.0 as u8, 6 as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }
    instr.write_num(imm as i8);

    Some(instr)
}

fn matches_xor801(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x30);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xor802(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x30);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xor803(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x31);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xor804(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x31);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xor805(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x31);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xor806(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x32);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xor807(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let reg = is_reg_of_size(&mut iter, 8);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 8);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(false, rm.0 as u8, reg as u8);
    instr.write_byte(0x32);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xor808(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let reg = is_reg_of_size(&mut iter, 16);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 16);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x33);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xor809(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let reg = is_reg_of_size(&mut iter, 32);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 32);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_byte(0x33);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

fn matches_xor810(tokens: &Vec<String>) -> Option<Instruction> {
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != "xor" { return None; }
    let reg = is_reg_of_size(&mut iter, 64);
    if reg.is_none() { return None; }
    let reg = reg.unwrap();
    if iter.next().unwrap() != "," { return None; }
    let rm = is_rm_of_size(&mut iter, 64);
    if rm.is_none() { return None; }
    let rm = rm.unwrap();
    if iter.next().is_some() { return None; }
    let mut instr = Instruction::new("xor".to_string());

    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };
    instr.write_rex(true, rm.0 as u8, reg as u8);
    instr.write_byte(0x33);
    instr.write_mod(m, rm.0 as u8, reg as u8);

    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }

    Some(instr)
}

pub fn matches(tokens: &Vec<String>) -> Option<Instruction> {
    if let Some(instr) = matches_aaa1(tokens) { Some(instr) }
    else if let Some(instr) = matches_aad2(tokens) { Some(instr) }
    else if let Some(instr) = matches_aad3(tokens) { Some(instr) }
    else if let Some(instr) = matches_aam4(tokens) { Some(instr) }
    else if let Some(instr) = matches_aam5(tokens) { Some(instr) }
    else if let Some(instr) = matches_aas6(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc7(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc8(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc9(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc10(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc11(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc12(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc13(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc14(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc15(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc16(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc17(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc18(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc19(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc20(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc21(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc22(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc23(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc24(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc25(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc26(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc27(tokens) { Some(instr) }
    else if let Some(instr) = matches_adc28(tokens) { Some(instr) }
    else if let Some(instr) = matches_adcx29(tokens) { Some(instr) }
    else if let Some(instr) = matches_adcx30(tokens) { Some(instr) }
    else if let Some(instr) = matches_add31(tokens) { Some(instr) }
    else if let Some(instr) = matches_add32(tokens) { Some(instr) }
    else if let Some(instr) = matches_add33(tokens) { Some(instr) }
    else if let Some(instr) = matches_add34(tokens) { Some(instr) }
    else if let Some(instr) = matches_add35(tokens) { Some(instr) }
    else if let Some(instr) = matches_add36(tokens) { Some(instr) }
    else if let Some(instr) = matches_add37(tokens) { Some(instr) }
    else if let Some(instr) = matches_add38(tokens) { Some(instr) }
    else if let Some(instr) = matches_add39(tokens) { Some(instr) }
    else if let Some(instr) = matches_add40(tokens) { Some(instr) }
    else if let Some(instr) = matches_add41(tokens) { Some(instr) }
    else if let Some(instr) = matches_add42(tokens) { Some(instr) }
    else if let Some(instr) = matches_add43(tokens) { Some(instr) }
    else if let Some(instr) = matches_add44(tokens) { Some(instr) }
    else if let Some(instr) = matches_add45(tokens) { Some(instr) }
    else if let Some(instr) = matches_add46(tokens) { Some(instr) }
    else if let Some(instr) = matches_add47(tokens) { Some(instr) }
    else if let Some(instr) = matches_add48(tokens) { Some(instr) }
    else if let Some(instr) = matches_add49(tokens) { Some(instr) }
    else if let Some(instr) = matches_add50(tokens) { Some(instr) }
    else if let Some(instr) = matches_add51(tokens) { Some(instr) }
    else if let Some(instr) = matches_add52(tokens) { Some(instr) }
    else if let Some(instr) = matches_adox53(tokens) { Some(instr) }
    else if let Some(instr) = matches_adox54(tokens) { Some(instr) }
    else if let Some(instr) = matches_and55(tokens) { Some(instr) }
    else if let Some(instr) = matches_and56(tokens) { Some(instr) }
    else if let Some(instr) = matches_and57(tokens) { Some(instr) }
    else if let Some(instr) = matches_and58(tokens) { Some(instr) }
    else if let Some(instr) = matches_and59(tokens) { Some(instr) }
    else if let Some(instr) = matches_and60(tokens) { Some(instr) }
    else if let Some(instr) = matches_and61(tokens) { Some(instr) }
    else if let Some(instr) = matches_and62(tokens) { Some(instr) }
    else if let Some(instr) = matches_and63(tokens) { Some(instr) }
    else if let Some(instr) = matches_and64(tokens) { Some(instr) }
    else if let Some(instr) = matches_and65(tokens) { Some(instr) }
    else if let Some(instr) = matches_and66(tokens) { Some(instr) }
    else if let Some(instr) = matches_and67(tokens) { Some(instr) }
    else if let Some(instr) = matches_and68(tokens) { Some(instr) }
    else if let Some(instr) = matches_and69(tokens) { Some(instr) }
    else if let Some(instr) = matches_and70(tokens) { Some(instr) }
    else if let Some(instr) = matches_and71(tokens) { Some(instr) }
    else if let Some(instr) = matches_and72(tokens) { Some(instr) }
    else if let Some(instr) = matches_and73(tokens) { Some(instr) }
    else if let Some(instr) = matches_and74(tokens) { Some(instr) }
    else if let Some(instr) = matches_and75(tokens) { Some(instr) }
    else if let Some(instr) = matches_and76(tokens) { Some(instr) }
    else if let Some(instr) = matches_arpl77(tokens) { Some(instr) }
    else if let Some(instr) = matches_bsf78(tokens) { Some(instr) }
    else if let Some(instr) = matches_bsf79(tokens) { Some(instr) }
    else if let Some(instr) = matches_bsf80(tokens) { Some(instr) }
    else if let Some(instr) = matches_bsr81(tokens) { Some(instr) }
    else if let Some(instr) = matches_bsr82(tokens) { Some(instr) }
    else if let Some(instr) = matches_bsr83(tokens) { Some(instr) }
    else if let Some(instr) = matches_bswap84(tokens) { Some(instr) }
    else if let Some(instr) = matches_bswap85(tokens) { Some(instr) }
    else if let Some(instr) = matches_bt86(tokens) { Some(instr) }
    else if let Some(instr) = matches_bt87(tokens) { Some(instr) }
    else if let Some(instr) = matches_bt88(tokens) { Some(instr) }
    else if let Some(instr) = matches_bt89(tokens) { Some(instr) }
    else if let Some(instr) = matches_bt90(tokens) { Some(instr) }
    else if let Some(instr) = matches_bt91(tokens) { Some(instr) }
    else if let Some(instr) = matches_btc92(tokens) { Some(instr) }
    else if let Some(instr) = matches_btc93(tokens) { Some(instr) }
    else if let Some(instr) = matches_btc94(tokens) { Some(instr) }
    else if let Some(instr) = matches_btc95(tokens) { Some(instr) }
    else if let Some(instr) = matches_btc96(tokens) { Some(instr) }
    else if let Some(instr) = matches_btc97(tokens) { Some(instr) }
    else if let Some(instr) = matches_btr98(tokens) { Some(instr) }
    else if let Some(instr) = matches_btr99(tokens) { Some(instr) }
    else if let Some(instr) = matches_btr100(tokens) { Some(instr) }
    else if let Some(instr) = matches_btr101(tokens) { Some(instr) }
    else if let Some(instr) = matches_btr102(tokens) { Some(instr) }
    else if let Some(instr) = matches_btr103(tokens) { Some(instr) }
    else if let Some(instr) = matches_bts104(tokens) { Some(instr) }
    else if let Some(instr) = matches_bts105(tokens) { Some(instr) }
    else if let Some(instr) = matches_bts106(tokens) { Some(instr) }
    else if let Some(instr) = matches_bts107(tokens) { Some(instr) }
    else if let Some(instr) = matches_bts108(tokens) { Some(instr) }
    else if let Some(instr) = matches_bts109(tokens) { Some(instr) }
    else if let Some(instr) = matches_call110(tokens) { Some(instr) }
    else if let Some(instr) = matches_call111(tokens) { Some(instr) }
    else if let Some(instr) = matches_call112(tokens) { Some(instr) }
    else if let Some(instr) = matches_cbw113(tokens) { Some(instr) }
    else if let Some(instr) = matches_cwde114(tokens) { Some(instr) }
    else if let Some(instr) = matches_cdqe115(tokens) { Some(instr) }
    else if let Some(instr) = matches_clc116(tokens) { Some(instr) }
    else if let Some(instr) = matches_cld117(tokens) { Some(instr) }
    else if let Some(instr) = matches_cli118(tokens) { Some(instr) }
    else if let Some(instr) = matches_clts119(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmc120(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmova121(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmova122(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmova123(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovae124(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovae125(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovae126(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovb127(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovb128(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovb129(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovbe130(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovbe131(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovbe132(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovc133(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovc134(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovc135(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmove136(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmove137(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmove138(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovg139(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovg140(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovg141(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovge142(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovge143(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovge144(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovl145(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovl146(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovl147(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovle148(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovle149(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovle150(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovna151(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovna152(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovna153(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnae154(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnae155(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnae156(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnb157(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnb158(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnb159(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnbe160(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnbe161(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnbe162(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnc163(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnc164(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnc165(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovne166(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovne167(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovne168(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovng169(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovng170(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovng171(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnge172(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnge173(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnge174(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnl175(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnl176(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnl177(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnle178(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnle179(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnle180(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovno181(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovno182(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovno183(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnp184(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnp185(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnp186(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovns187(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovns188(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovns189(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnz190(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnz191(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovnz192(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovo193(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovo194(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovo195(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovp196(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovp197(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovp198(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovpe199(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovpe200(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmovpe201(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp202(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp203(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp204(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp205(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp206(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp207(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp208(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp209(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp210(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp211(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp212(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp213(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp214(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp215(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp216(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp217(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp218(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp219(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp220(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp221(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp222(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmp223(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmpsb224(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmpsw225(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmpsd226(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmpsq227(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmpxchg228(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmpxchg229(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmpxchg230(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmpxchg231(tokens) { Some(instr) }
    else if let Some(instr) = matches_cmpxchg232(tokens) { Some(instr) }
    else if let Some(instr) = matches_cpuid233(tokens) { Some(instr) }
    else if let Some(instr) = matches_crc32234(tokens) { Some(instr) }
    else if let Some(instr) = matches_crc32235(tokens) { Some(instr) }
    else if let Some(instr) = matches_crc32236(tokens) { Some(instr) }
    else if let Some(instr) = matches_crc32237(tokens) { Some(instr) }
    else if let Some(instr) = matches_crc32238(tokens) { Some(instr) }
    else if let Some(instr) = matches_crc32239(tokens) { Some(instr) }
    else if let Some(instr) = matches_cwd240(tokens) { Some(instr) }
    else if let Some(instr) = matches_cdq241(tokens) { Some(instr) }
    else if let Some(instr) = matches_cqo242(tokens) { Some(instr) }
    else if let Some(instr) = matches_daa243(tokens) { Some(instr) }
    else if let Some(instr) = matches_das244(tokens) { Some(instr) }
    else if let Some(instr) = matches_dec245(tokens) { Some(instr) }
    else if let Some(instr) = matches_dec246(tokens) { Some(instr) }
    else if let Some(instr) = matches_dec247(tokens) { Some(instr) }
    else if let Some(instr) = matches_dec248(tokens) { Some(instr) }
    else if let Some(instr) = matches_dec249(tokens) { Some(instr) }
    else if let Some(instr) = matches_dec250(tokens) { Some(instr) }
    else if let Some(instr) = matches_dec251(tokens) { Some(instr) }
    else if let Some(instr) = matches_div252(tokens) { Some(instr) }
    else if let Some(instr) = matches_div253(tokens) { Some(instr) }
    else if let Some(instr) = matches_div254(tokens) { Some(instr) }
    else if let Some(instr) = matches_div255(tokens) { Some(instr) }
    else if let Some(instr) = matches_div256(tokens) { Some(instr) }
    else if let Some(instr) = matches_enter257(tokens) { Some(instr) }
    else if let Some(instr) = matches_enter258(tokens) { Some(instr) }
    else if let Some(instr) = matches_enter259(tokens) { Some(instr) }
    else if let Some(instr) = matches_f2xm1260(tokens) { Some(instr) }
    else if let Some(instr) = matches_fabs261(tokens) { Some(instr) }
    else if let Some(instr) = matches_faddp262(tokens) { Some(instr) }
    else if let Some(instr) = matches_fchs263(tokens) { Some(instr) }
    else if let Some(instr) = matches_fclex264(tokens) { Some(instr) }
    else if let Some(instr) = matches_fnclex265(tokens) { Some(instr) }
    else if let Some(instr) = matches_fcom266(tokens) { Some(instr) }
    else if let Some(instr) = matches_fcomp267(tokens) { Some(instr) }
    else if let Some(instr) = matches_fcompp268(tokens) { Some(instr) }
    else if let Some(instr) = matches_fcos269(tokens) { Some(instr) }
    else if let Some(instr) = matches_fdecstp270(tokens) { Some(instr) }
    else if let Some(instr) = matches_fdivp271(tokens) { Some(instr) }
    else if let Some(instr) = matches_fdivrp272(tokens) { Some(instr) }
    else if let Some(instr) = matches_fincstp273(tokens) { Some(instr) }
    else if let Some(instr) = matches_finit274(tokens) { Some(instr) }
    else if let Some(instr) = matches_fninit275(tokens) { Some(instr) }
    else if let Some(instr) = matches_fld1276(tokens) { Some(instr) }
    else if let Some(instr) = matches_fldl2t277(tokens) { Some(instr) }
    else if let Some(instr) = matches_fldl2e278(tokens) { Some(instr) }
    else if let Some(instr) = matches_fldpi279(tokens) { Some(instr) }
    else if let Some(instr) = matches_fldlg2280(tokens) { Some(instr) }
    else if let Some(instr) = matches_fldln2281(tokens) { Some(instr) }
    else if let Some(instr) = matches_fldz282(tokens) { Some(instr) }
    else if let Some(instr) = matches_fmulp283(tokens) { Some(instr) }
    else if let Some(instr) = matches_fnop284(tokens) { Some(instr) }
    else if let Some(instr) = matches_fpatan285(tokens) { Some(instr) }
    else if let Some(instr) = matches_fprem286(tokens) { Some(instr) }
    else if let Some(instr) = matches_fprem1287(tokens) { Some(instr) }
    else if let Some(instr) = matches_fptan288(tokens) { Some(instr) }
    else if let Some(instr) = matches_frndint289(tokens) { Some(instr) }
    else if let Some(instr) = matches_fscale290(tokens) { Some(instr) }
    else if let Some(instr) = matches_fsin291(tokens) { Some(instr) }
    else if let Some(instr) = matches_fsincos292(tokens) { Some(instr) }
    else if let Some(instr) = matches_fsqrt293(tokens) { Some(instr) }
    else if let Some(instr) = matches_fstsw294(tokens) { Some(instr) }
    else if let Some(instr) = matches_fnstsw295(tokens) { Some(instr) }
    else if let Some(instr) = matches_fsubp296(tokens) { Some(instr) }
    else if let Some(instr) = matches_fsubrp297(tokens) { Some(instr) }
    else if let Some(instr) = matches_ftst298(tokens) { Some(instr) }
    else if let Some(instr) = matches_fucom299(tokens) { Some(instr) }
    else if let Some(instr) = matches_fucomp300(tokens) { Some(instr) }
    else if let Some(instr) = matches_fucompp301(tokens) { Some(instr) }
    else if let Some(instr) = matches_fxam302(tokens) { Some(instr) }
    else if let Some(instr) = matches_fxch303(tokens) { Some(instr) }
    else if let Some(instr) = matches_fxtract304(tokens) { Some(instr) }
    else if let Some(instr) = matches_fyl2x305(tokens) { Some(instr) }
    else if let Some(instr) = matches_fyl2xp1306(tokens) { Some(instr) }
    else if let Some(instr) = matches_hlt307(tokens) { Some(instr) }
    else if let Some(instr) = matches_idiv308(tokens) { Some(instr) }
    else if let Some(instr) = matches_idiv309(tokens) { Some(instr) }
    else if let Some(instr) = matches_idiv310(tokens) { Some(instr) }
    else if let Some(instr) = matches_idiv311(tokens) { Some(instr) }
    else if let Some(instr) = matches_idiv312(tokens) { Some(instr) }
    else if let Some(instr) = matches_imul313(tokens) { Some(instr) }
    else if let Some(instr) = matches_imul314(tokens) { Some(instr) }
    else if let Some(instr) = matches_imul315(tokens) { Some(instr) }
    else if let Some(instr) = matches_imul316(tokens) { Some(instr) }
    else if let Some(instr) = matches_imul317(tokens) { Some(instr) }
    else if let Some(instr) = matches_imul318(tokens) { Some(instr) }
    else if let Some(instr) = matches_imul319(tokens) { Some(instr) }
    else if let Some(instr) = matches_in320(tokens) { Some(instr) }
    else if let Some(instr) = matches_in321(tokens) { Some(instr) }
    else if let Some(instr) = matches_in322(tokens) { Some(instr) }
    else if let Some(instr) = matches_in323(tokens) { Some(instr) }
    else if let Some(instr) = matches_in324(tokens) { Some(instr) }
    else if let Some(instr) = matches_in325(tokens) { Some(instr) }
    else if let Some(instr) = matches_inc326(tokens) { Some(instr) }
    else if let Some(instr) = matches_inc327(tokens) { Some(instr) }
    else if let Some(instr) = matches_inc328(tokens) { Some(instr) }
    else if let Some(instr) = matches_inc329(tokens) { Some(instr) }
    else if let Some(instr) = matches_inc330(tokens) { Some(instr) }
    else if let Some(instr) = matches_inc331(tokens) { Some(instr) }
    else if let Some(instr) = matches_inc332(tokens) { Some(instr) }
    else if let Some(instr) = matches_insb333(tokens) { Some(instr) }
    else if let Some(instr) = matches_insw334(tokens) { Some(instr) }
    else if let Some(instr) = matches_insd335(tokens) { Some(instr) }
    else if let Some(instr) = matches_int336(tokens) { Some(instr) }
    else if let Some(instr) = matches_into337(tokens) { Some(instr) }
    else if let Some(instr) = matches_invd338(tokens) { Some(instr) }
    else if let Some(instr) = matches_iret339(tokens) { Some(instr) }
    else if let Some(instr) = matches_iretd340(tokens) { Some(instr) }
    else if let Some(instr) = matches_iretq341(tokens) { Some(instr) }
    else if let Some(instr) = matches_jmp342(tokens) { Some(instr) }
    else if let Some(instr) = matches_jmp343(tokens) { Some(instr) }
    else if let Some(instr) = matches_jmp344(tokens) { Some(instr) }
    else if let Some(instr) = matches_lahf345(tokens) { Some(instr) }
    else if let Some(instr) = matches_leave346(tokens) { Some(instr) }
    else if let Some(instr) = matches_leave347(tokens) { Some(instr) }
    else if let Some(instr) = matches_leave348(tokens) { Some(instr) }
    else if let Some(instr) = matches_lldt349(tokens) { Some(instr) }
    else if let Some(instr) = matches_lmsw350(tokens) { Some(instr) }
    else if let Some(instr) = matches_lock351(tokens) { Some(instr) }
    else if let Some(instr) = matches_lodsb352(tokens) { Some(instr) }
    else if let Some(instr) = matches_lodsw353(tokens) { Some(instr) }
    else if let Some(instr) = matches_lodsd354(tokens) { Some(instr) }
    else if let Some(instr) = matches_lodsq355(tokens) { Some(instr) }
    else if let Some(instr) = matches_ltr356(tokens) { Some(instr) }
    else if let Some(instr) = matches_lzcnt357(tokens) { Some(instr) }
    else if let Some(instr) = matches_lzcnt358(tokens) { Some(instr) }
    else if let Some(instr) = matches_lzcnt359(tokens) { Some(instr) }
    else if let Some(instr) = matches_monitor360(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov361(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov362(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov363(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov364(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov365(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov366(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov367(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov368(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov369(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov370(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov371(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov372(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov373(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov374(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov375(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov376(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov377(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov378(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov379(tokens) { Some(instr) }
    else if let Some(instr) = matches_mov380(tokens) { Some(instr) }
    else if let Some(instr) = matches_movsb381(tokens) { Some(instr) }
    else if let Some(instr) = matches_movsw382(tokens) { Some(instr) }
    else if let Some(instr) = matches_movsd383(tokens) { Some(instr) }
    else if let Some(instr) = matches_movsq384(tokens) { Some(instr) }
    else if let Some(instr) = matches_movsx385(tokens) { Some(instr) }
    else if let Some(instr) = matches_movsx386(tokens) { Some(instr) }
    else if let Some(instr) = matches_movsx387(tokens) { Some(instr) }
    else if let Some(instr) = matches_movsx388(tokens) { Some(instr) }
    else if let Some(instr) = matches_movsx389(tokens) { Some(instr) }
    else if let Some(instr) = matches_movsxd390(tokens) { Some(instr) }
    else if let Some(instr) = matches_movzx391(tokens) { Some(instr) }
    else if let Some(instr) = matches_movzx392(tokens) { Some(instr) }
    else if let Some(instr) = matches_movzx393(tokens) { Some(instr) }
    else if let Some(instr) = matches_movzx394(tokens) { Some(instr) }
    else if let Some(instr) = matches_movzx395(tokens) { Some(instr) }
    else if let Some(instr) = matches_mul396(tokens) { Some(instr) }
    else if let Some(instr) = matches_mul397(tokens) { Some(instr) }
    else if let Some(instr) = matches_mul398(tokens) { Some(instr) }
    else if let Some(instr) = matches_mul399(tokens) { Some(instr) }
    else if let Some(instr) = matches_mul400(tokens) { Some(instr) }
    else if let Some(instr) = matches_mwait401(tokens) { Some(instr) }
    else if let Some(instr) = matches_neg402(tokens) { Some(instr) }
    else if let Some(instr) = matches_neg403(tokens) { Some(instr) }
    else if let Some(instr) = matches_neg404(tokens) { Some(instr) }
    else if let Some(instr) = matches_neg405(tokens) { Some(instr) }
    else if let Some(instr) = matches_neg406(tokens) { Some(instr) }
    else if let Some(instr) = matches_not407(tokens) { Some(instr) }
    else if let Some(instr) = matches_not408(tokens) { Some(instr) }
    else if let Some(instr) = matches_not409(tokens) { Some(instr) }
    else if let Some(instr) = matches_not410(tokens) { Some(instr) }
    else if let Some(instr) = matches_not411(tokens) { Some(instr) }
    else if let Some(instr) = matches_or412(tokens) { Some(instr) }
    else if let Some(instr) = matches_or413(tokens) { Some(instr) }
    else if let Some(instr) = matches_or414(tokens) { Some(instr) }
    else if let Some(instr) = matches_or415(tokens) { Some(instr) }
    else if let Some(instr) = matches_or416(tokens) { Some(instr) }
    else if let Some(instr) = matches_or417(tokens) { Some(instr) }
    else if let Some(instr) = matches_or418(tokens) { Some(instr) }
    else if let Some(instr) = matches_or419(tokens) { Some(instr) }
    else if let Some(instr) = matches_or420(tokens) { Some(instr) }
    else if let Some(instr) = matches_or421(tokens) { Some(instr) }
    else if let Some(instr) = matches_or422(tokens) { Some(instr) }
    else if let Some(instr) = matches_or423(tokens) { Some(instr) }
    else if let Some(instr) = matches_or424(tokens) { Some(instr) }
    else if let Some(instr) = matches_or425(tokens) { Some(instr) }
    else if let Some(instr) = matches_or426(tokens) { Some(instr) }
    else if let Some(instr) = matches_or427(tokens) { Some(instr) }
    else if let Some(instr) = matches_or428(tokens) { Some(instr) }
    else if let Some(instr) = matches_or429(tokens) { Some(instr) }
    else if let Some(instr) = matches_or430(tokens) { Some(instr) }
    else if let Some(instr) = matches_or431(tokens) { Some(instr) }
    else if let Some(instr) = matches_or432(tokens) { Some(instr) }
    else if let Some(instr) = matches_or433(tokens) { Some(instr) }
    else if let Some(instr) = matches_out434(tokens) { Some(instr) }
    else if let Some(instr) = matches_out435(tokens) { Some(instr) }
    else if let Some(instr) = matches_out436(tokens) { Some(instr) }
    else if let Some(instr) = matches_out437(tokens) { Some(instr) }
    else if let Some(instr) = matches_out438(tokens) { Some(instr) }
    else if let Some(instr) = matches_out439(tokens) { Some(instr) }
    else if let Some(instr) = matches_outsb440(tokens) { Some(instr) }
    else if let Some(instr) = matches_outsw441(tokens) { Some(instr) }
    else if let Some(instr) = matches_outsd442(tokens) { Some(instr) }
    else if let Some(instr) = matches_pause443(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop444(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop445(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop446(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop447(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop448(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop449(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop450(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop451(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop452(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop453(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop454(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop455(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop456(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop457(tokens) { Some(instr) }
    else if let Some(instr) = matches_pop458(tokens) { Some(instr) }
    else if let Some(instr) = matches_popa459(tokens) { Some(instr) }
    else if let Some(instr) = matches_popad460(tokens) { Some(instr) }
    else if let Some(instr) = matches_popcnt461(tokens) { Some(instr) }
    else if let Some(instr) = matches_popcnt462(tokens) { Some(instr) }
    else if let Some(instr) = matches_popcnt463(tokens) { Some(instr) }
    else if let Some(instr) = matches_popf464(tokens) { Some(instr) }
    else if let Some(instr) = matches_popfd465(tokens) { Some(instr) }
    else if let Some(instr) = matches_popfq466(tokens) { Some(instr) }
    else if let Some(instr) = matches_push467(tokens) { Some(instr) }
    else if let Some(instr) = matches_push468(tokens) { Some(instr) }
    else if let Some(instr) = matches_push469(tokens) { Some(instr) }
    else if let Some(instr) = matches_push470(tokens) { Some(instr) }
    else if let Some(instr) = matches_push471(tokens) { Some(instr) }
    else if let Some(instr) = matches_push472(tokens) { Some(instr) }
    else if let Some(instr) = matches_push473(tokens) { Some(instr) }
    else if let Some(instr) = matches_push474(tokens) { Some(instr) }
    else if let Some(instr) = matches_push475(tokens) { Some(instr) }
    else if let Some(instr) = matches_push476(tokens) { Some(instr) }
    else if let Some(instr) = matches_push477(tokens) { Some(instr) }
    else if let Some(instr) = matches_push478(tokens) { Some(instr) }
    else if let Some(instr) = matches_push479(tokens) { Some(instr) }
    else if let Some(instr) = matches_push480(tokens) { Some(instr) }
    else if let Some(instr) = matches_push481(tokens) { Some(instr) }
    else if let Some(instr) = matches_pusha482(tokens) { Some(instr) }
    else if let Some(instr) = matches_pushad483(tokens) { Some(instr) }
    else if let Some(instr) = matches_pushf484(tokens) { Some(instr) }
    else if let Some(instr) = matches_pushfd485(tokens) { Some(instr) }
    else if let Some(instr) = matches_pushfq486(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl487(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl488(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl489(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl490(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl491(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl492(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl493(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl494(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl495(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl496(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl497(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl498(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl499(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl500(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcl501(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr502(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr503(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr504(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr505(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr506(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr507(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr508(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr509(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr510(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr511(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr512(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr513(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr514(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr515(tokens) { Some(instr) }
    else if let Some(instr) = matches_rcr516(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol517(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol518(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol519(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol520(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol521(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol522(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol523(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol524(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol525(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol526(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol527(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol528(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol529(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol530(tokens) { Some(instr) }
    else if let Some(instr) = matches_rol531(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror532(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror533(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror534(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror535(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror536(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror537(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror538(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror539(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror540(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror541(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror542(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror543(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror544(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror545(tokens) { Some(instr) }
    else if let Some(instr) = matches_ror546(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdfsbase547(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdfsbase548(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdgsbase549(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdgsbase550(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdmsr551(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdpid552(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdpid553(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdpmc554(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdrand555(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdrand556(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdrand557(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdseed558(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdseed559(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdseed560(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdtsc561(tokens) { Some(instr) }
    else if let Some(instr) = matches_rdtscp562(tokens) { Some(instr) }
    else if let Some(instr) = matches_ret563(tokens) { Some(instr) }
    else if let Some(instr) = matches_retf564(tokens) { Some(instr) }
    else if let Some(instr) = matches_ret565(tokens) { Some(instr) }
    else if let Some(instr) = matches_ret566(tokens) { Some(instr) }
    else if let Some(instr) = matches_rsm567(tokens) { Some(instr) }
    else if let Some(instr) = matches_sahf568(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal569(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal570(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal571(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal572(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal573(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal574(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal575(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal576(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal577(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal578(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal579(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal580(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal581(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal582(tokens) { Some(instr) }
    else if let Some(instr) = matches_sal583(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar584(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar585(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar586(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar587(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar588(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar589(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar590(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar591(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar592(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar593(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar594(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar595(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar596(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar597(tokens) { Some(instr) }
    else if let Some(instr) = matches_sar598(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl599(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl600(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl601(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl602(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl603(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl604(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl605(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl606(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl607(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl608(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl609(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl610(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl611(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl612(tokens) { Some(instr) }
    else if let Some(instr) = matches_shl613(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr614(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr615(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr616(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr617(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr618(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr619(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr620(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr621(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr622(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr623(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr624(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr625(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr626(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr627(tokens) { Some(instr) }
    else if let Some(instr) = matches_shr628(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb629(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb630(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb631(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb632(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb633(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb634(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb635(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb636(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb637(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb638(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb639(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb640(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb641(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb642(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb643(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb644(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb645(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb646(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb647(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb648(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb649(tokens) { Some(instr) }
    else if let Some(instr) = matches_sbb650(tokens) { Some(instr) }
    else if let Some(instr) = matches_scasb651(tokens) { Some(instr) }
    else if let Some(instr) = matches_scasw652(tokens) { Some(instr) }
    else if let Some(instr) = matches_scasd653(tokens) { Some(instr) }
    else if let Some(instr) = matches_scasq654(tokens) { Some(instr) }
    else if let Some(instr) = matches_seta655(tokens) { Some(instr) }
    else if let Some(instr) = matches_seta656(tokens) { Some(instr) }
    else if let Some(instr) = matches_setae657(tokens) { Some(instr) }
    else if let Some(instr) = matches_setae658(tokens) { Some(instr) }
    else if let Some(instr) = matches_setb659(tokens) { Some(instr) }
    else if let Some(instr) = matches_setb660(tokens) { Some(instr) }
    else if let Some(instr) = matches_setbe661(tokens) { Some(instr) }
    else if let Some(instr) = matches_setbe662(tokens) { Some(instr) }
    else if let Some(instr) = matches_setc663(tokens) { Some(instr) }
    else if let Some(instr) = matches_setc664(tokens) { Some(instr) }
    else if let Some(instr) = matches_sete665(tokens) { Some(instr) }
    else if let Some(instr) = matches_sete666(tokens) { Some(instr) }
    else if let Some(instr) = matches_setg667(tokens) { Some(instr) }
    else if let Some(instr) = matches_setg668(tokens) { Some(instr) }
    else if let Some(instr) = matches_setge669(tokens) { Some(instr) }
    else if let Some(instr) = matches_setge670(tokens) { Some(instr) }
    else if let Some(instr) = matches_setl671(tokens) { Some(instr) }
    else if let Some(instr) = matches_setl672(tokens) { Some(instr) }
    else if let Some(instr) = matches_setle673(tokens) { Some(instr) }
    else if let Some(instr) = matches_setle674(tokens) { Some(instr) }
    else if let Some(instr) = matches_setna675(tokens) { Some(instr) }
    else if let Some(instr) = matches_setna676(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnae677(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnae678(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnb679(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnb680(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnbe681(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnbe682(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnc683(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnc684(tokens) { Some(instr) }
    else if let Some(instr) = matches_setne685(tokens) { Some(instr) }
    else if let Some(instr) = matches_setne686(tokens) { Some(instr) }
    else if let Some(instr) = matches_setng687(tokens) { Some(instr) }
    else if let Some(instr) = matches_setng688(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnge689(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnge690(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnl691(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnl692(tokens) { Some(instr) }
    else if let Some(instr) = matches_setnle693(tokens) { Some(instr) }
    else if let Some(instr) = matches_sldt694(tokens) { Some(instr) }
    else if let Some(instr) = matches_smsw695(tokens) { Some(instr) }
    else if let Some(instr) = matches_stc696(tokens) { Some(instr) }
    else if let Some(instr) = matches_std697(tokens) { Some(instr) }
    else if let Some(instr) = matches_sti698(tokens) { Some(instr) }
    else if let Some(instr) = matches_stosb699(tokens) { Some(instr) }
    else if let Some(instr) = matches_stosw700(tokens) { Some(instr) }
    else if let Some(instr) = matches_stosd701(tokens) { Some(instr) }
    else if let Some(instr) = matches_stosq702(tokens) { Some(instr) }
    else if let Some(instr) = matches_str703(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub704(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub705(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub706(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub707(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub708(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub709(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub710(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub711(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub712(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub713(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub714(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub715(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub716(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub717(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub718(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub719(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub720(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub721(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub722(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub723(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub724(tokens) { Some(instr) }
    else if let Some(instr) = matches_sub725(tokens) { Some(instr) }
    else if let Some(instr) = matches_swapgs726(tokens) { Some(instr) }
    else if let Some(instr) = matches_syscall727(tokens) { Some(instr) }
    else if let Some(instr) = matches_sysenter728(tokens) { Some(instr) }
    else if let Some(instr) = matches_sysexit729(tokens) { Some(instr) }
    else if let Some(instr) = matches_sysexit730(tokens) { Some(instr) }
    else if let Some(instr) = matches_sysret731(tokens) { Some(instr) }
    else if let Some(instr) = matches_sysret732(tokens) { Some(instr) }
    else if let Some(instr) = matches_test733(tokens) { Some(instr) }
    else if let Some(instr) = matches_test734(tokens) { Some(instr) }
    else if let Some(instr) = matches_test735(tokens) { Some(instr) }
    else if let Some(instr) = matches_test736(tokens) { Some(instr) }
    else if let Some(instr) = matches_test737(tokens) { Some(instr) }
    else if let Some(instr) = matches_test738(tokens) { Some(instr) }
    else if let Some(instr) = matches_test739(tokens) { Some(instr) }
    else if let Some(instr) = matches_test740(tokens) { Some(instr) }
    else if let Some(instr) = matches_test741(tokens) { Some(instr) }
    else if let Some(instr) = matches_test742(tokens) { Some(instr) }
    else if let Some(instr) = matches_test743(tokens) { Some(instr) }
    else if let Some(instr) = matches_test744(tokens) { Some(instr) }
    else if let Some(instr) = matches_test745(tokens) { Some(instr) }
    else if let Some(instr) = matches_test746(tokens) { Some(instr) }
    else if let Some(instr) = matches_tzcnt747(tokens) { Some(instr) }
    else if let Some(instr) = matches_tzcnt748(tokens) { Some(instr) }
    else if let Some(instr) = matches_tzcnt749(tokens) { Some(instr) }
    else if let Some(instr) = matches_ud0750(tokens) { Some(instr) }
    else if let Some(instr) = matches_ud1751(tokens) { Some(instr) }
    else if let Some(instr) = matches_ud2752(tokens) { Some(instr) }
    else if let Some(instr) = matches_verr753(tokens) { Some(instr) }
    else if let Some(instr) = matches_verw754(tokens) { Some(instr) }
    else if let Some(instr) = matches_wait755(tokens) { Some(instr) }
    else if let Some(instr) = matches_fwait756(tokens) { Some(instr) }
    else if let Some(instr) = matches_wbinvd757(tokens) { Some(instr) }
    else if let Some(instr) = matches_wrfsbase758(tokens) { Some(instr) }
    else if let Some(instr) = matches_wrfsbase759(tokens) { Some(instr) }
    else if let Some(instr) = matches_wrgsbase760(tokens) { Some(instr) }
    else if let Some(instr) = matches_wrgsbase761(tokens) { Some(instr) }
    else if let Some(instr) = matches_wrmsr762(tokens) { Some(instr) }
    else if let Some(instr) = matches_xabort763(tokens) { Some(instr) }
    else if let Some(instr) = matches_xacquire764(tokens) { Some(instr) }
    else if let Some(instr) = matches_xrelease765(tokens) { Some(instr) }
    else if let Some(instr) = matches_xadd766(tokens) { Some(instr) }
    else if let Some(instr) = matches_xadd767(tokens) { Some(instr) }
    else if let Some(instr) = matches_xadd768(tokens) { Some(instr) }
    else if let Some(instr) = matches_xadd769(tokens) { Some(instr) }
    else if let Some(instr) = matches_xadd770(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg771(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg772(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg773(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg774(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg775(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg776(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg777(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg778(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg779(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg780(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg781(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg782(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg783(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg784(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg785(tokens) { Some(instr) }
    else if let Some(instr) = matches_xchg786(tokens) { Some(instr) }
    else if let Some(instr) = matches_xlatb787(tokens) { Some(instr) }
    else if let Some(instr) = matches_xlatb788(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor789(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor790(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor791(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor792(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor793(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor794(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor795(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor796(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor797(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor798(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor799(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor800(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor801(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor802(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor803(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor804(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor805(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor806(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor807(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor808(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor809(tokens) { Some(instr) }
    else if let Some(instr) = matches_xor810(tokens) { Some(instr) }
    else { None }
}
