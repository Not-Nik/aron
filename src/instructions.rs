// aron (c) Nikolas Wipper 2022

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::number::Number;
use crate::parse::helpers::{Immediate, ImmediateType, Relativity};
use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::str::FromStr;
use Register::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Register {
    Ax = 0,
    Cx,
    Dx,
    Bx,
    Sp,
    Bp,
    Si,
    Di,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    Rip = 0xFF,
}

impl TryFrom<i32> for Register {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Ax),
            1 => Ok(Cx),
            2 => Ok(Dx),
            3 => Ok(Bx),
            4 => Ok(Sp),
            5 => Ok(Bp),
            6 => Ok(Si),
            7 => Ok(Di),
            8 => Ok(R8),
            10 => Ok(R9),
            11 => Ok(R10),
            12 => Ok(R11),
            13 => Ok(R12),
            14 => Ok(R13),
            15 => Ok(R14),
            16 => Ok(R15),
            _ => Err(()),
        }
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ["ah", "al", "ax", "eax", "rax"].contains(&s) {
            Ok(Ax)
        } else if ["bh", "bl", "bx", "ebx", "rbx"].contains(&s) {
            Ok(Bx)
        } else if ["ch", "cl", "cx", "ecx", "rcx"].contains(&s) {
            Ok(Cx)
        } else if ["dh", "dl", "dx", "edx", "rdx"].contains(&s) {
            Ok(Dx)
        } else if ["sil", "si", "esi", "rsi"].contains(&s) {
            Ok(Si)
        } else if ["dil", "di", "edi", "rdi"].contains(&s) {
            Ok(Di)
        } else if ["spl", "sp", "esp", "rsp"].contains(&s) {
            Ok(Sp)
        } else if ["bpl", "bp", "ebp", "rbp"].contains(&s) {
            Ok(Bp)
        } else if ["r8b", "r8w", "r8d", "r8"].contains(&s) {
            Ok(R8)
        } else if ["r9b", "r9w", "r9d", "r9"].contains(&s) {
            Ok(R9)
        } else if ["r10b", "r10w", "r10d", "r10"].contains(&s) {
            Ok(R10)
        } else if ["r11b", "r11w", "r11d", "r11"].contains(&s) {
            Ok(R11)
        } else if ["r12b", "r12w", "r12d", "r12"].contains(&s) {
            Ok(R12)
        } else if ["r13b", "r13w", "r13d", "r13"].contains(&s) {
            Ok(R13)
        } else if ["r14b", "r14w", "r14d", "r14"].contains(&s) {
            Ok(R14)
        } else if ["r15b", "r15w", "r15d", "r15"].contains(&s) {
            Ok(R15)
        } else if ["rip", "eip", "ip"].contains(&s) {
            Ok(Rip)
        } else {
            Err(())
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mod {
    NoOffset = 0b00,
    Offset8Bit = 0b01,
    Offset32Bit = 0b10,
    NoDereference = 0b11,
}

use crate::parse::helpers::Relativity::RipRelative;
use Mod::*;

#[derive(Copy, Clone, Debug, Eq, PartialOrd, PartialEq)]
pub enum Size {
    Byte,
    Word,
    DWord,
    QWord,
}

impl TryFrom<usize> for Size {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use Size::*;

        match value {
            8 => Ok(Byte),
            16 => Ok(Word),
            32 => Ok(DWord),
            64 => Ok(QWord),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for Size {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use Size::*;

        match value.as_str() {
            "byte" => Ok(Byte),
            "word" => Ok(Word),
            "dword" => Ok(DWord),
            "qword" => Ok(QWord),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Opcode {
    Byte(u8),
    Rex { wide: bool },
    // ModPart(u8),
}

#[derive(Debug)]
pub struct Instruction {
    name: String,
    r#mod: Mod,
    opcode: Vec<Opcode>,
    offset: Option<Immediate>,
    reg: Option<Register>,
    rm: Option<Register>,
    imm1: Option<Immediate>,
    imm2: Option<Immediate>,
}

pub struct Reference {
    pub to: String,
    pub at: usize,
    pub rel: Relativity,
}

pub struct EncodedInstruction {
    bytes: Vec<u8>,
    name: String,
    refs: Vec<Reference>,
}

fn to_byte(reg: &Option<Register>) -> u8 {
    if reg == &Some(Rip) {
        0b101
    } else {
        reg.unwrap_or(Ax) as u8
    }
}

impl Instruction {
    pub fn new(
        name: String,
        r#mod: Mod,
        opcode: Vec<Opcode>,
        offset: Option<Immediate>,
        reg: Option<Register>,
        rm: Option<Register>,
        imm1: Option<Immediate>,
        imm2: Option<Immediate>,
    ) -> Self {
        Instruction { name, r#mod, opcode, offset, reg, rm, imm1, imm2 }
    }

    pub fn encode(&self) -> EncodedInstruction {
        let mut encoded = EncodedInstruction::new(self.name.clone());

        for part in &self.opcode {
            match part {
                Opcode::Byte(byte) => encoded.write_byte(*byte),
                Opcode::Rex { wide } => {
                    let rm_bw: u8 = (to_byte(&self.rm) & 0b1000) >> 1;
                    let reg_bw: u8 = (to_byte(&self.reg) & 0b1000) >> 3;
                    let rex: u8 = 0b01000000 | (*wide as u8) << 3 | rm_bw | reg_bw;
                    encoded.write_byte(rex);
                } //Opcode::ModPart(_) => {}
            }
        }

        if self.rm.is_some() || self.reg.is_some() {
            encoded.write_mod(self.r#mod, to_byte(&self.rm), to_byte(&self.reg));
        }

        if self.offset.is_some() {
            if self.r#mod == Offset32Bit || self.offset.as_ref().unwrap().rel == RipRelative {
                encoded.write_imm::<i32, [u8; 4]>(self.offset.as_ref().unwrap());
            } else if self.r#mod == Offset8Bit {
                encoded.write_imm::<i8, [u8; 1]>(self.offset.as_ref().unwrap());
            }
        }
        if self.imm1.is_some() {
            encoded.write_immediate(self.imm1.as_ref().unwrap());
        }
        if self.imm2.is_some() {
            encoded.write_immediate(self.imm2.as_ref().unwrap());
        }

        encoded
    }
}

impl Debug for EncodedInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Instruction { /*fields omitted*/ }")
    }
}

impl EncodedInstruction {
    pub fn new(name: String) -> Self {
        EncodedInstruction { bytes: Vec::new(), name, refs: Vec::new() }
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn get_refs(&self) -> &Vec<Reference> {
        &self.refs
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub fn write_num<I: Number<Output = O>, O: AsRef<[u8]>>(&mut self, value: I) {
        self.bytes.write(value.to_bytes(false).as_ref()).unwrap();
    }

    pub fn write_imm<'a, I: Number<Output = O> + From<i8> + TryFrom<i32>, O: AsRef<[u8]>>(&mut self, imm: &Immediate) {
        let at = self.bytes.len();
        match &imm.typ {
            ImmediateType::Integer(i) => {
                // The unsafe block helps avoid needing Debug for I's TryInto error
                self.write_num::<I, O>(unsafe { (*i).try_into().unwrap_unchecked() });
            }
            ImmediateType::Reference(r) => {
                self.write_num::<I, O>(0.into());
                self.refs.push(Reference { to: r.to_string(), at, rel: imm.rel });
            }
        }
    }

    pub fn write_immediate(&mut self, imm: &Immediate) {
        match &imm.size {
            Size::Byte => self.write_imm::<i8, [u8; 1]>(imm),
            Size::Word => self.write_imm::<i16, [u8; 2]>(imm),
            Size::DWord => self.write_imm::<i32, [u8; 4]>(imm),
            Size::QWord => self.write_imm::<i64, [u8; 8]>(imm),
        };
    }

    fn write_mod(&mut self, r#mod: Mod, rm: u8, reg: u8) {
        let mod_rm = (r#mod as u8) << 6 | ((reg & 0b111) << 3) | (rm & 0b111);
        self.write_byte(mod_rm);
    }
}
