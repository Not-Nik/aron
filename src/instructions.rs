// aron (c) Nikolas Wipper 2022

use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::str::FromStr;
use Register::*;
use crate::number::Number;

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
        } else {
            Err(())
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Mod {
    NoOffset = 0b00,
    Offset8Bit = 0b01,
    Offset32Bit = 0b10,
    NoDereference = 0b11,
}

use Mod::*;

#[derive(PartialEq, Eq, PartialOrd, Copy, Clone)]
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
            _ => Err(())
        }
    }
}

pub struct Instruction {
    long: bool,
    bytes: Vec<u8>,
    name: String
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Instruction { /*fields omitted*/ }")
    }
}

impl Instruction {
    pub fn new(name: String) -> Self {
        Instruction {
            long: true,
            bytes: Vec::new(),
            name
        }
    }

    pub fn get_bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn has_name(&self, name: &str) -> bool { self.name == name }

    pub fn write_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }
    
    pub fn write_num<I: Number<Output = O>, O: AsRef<[u8]>>(&mut self, value: I) {
        self.bytes.write(value.to_bytes(false).as_ref()).unwrap();
    }

    pub fn write_rex(&mut self, w: bool, rm: u8, reg: u8) {
        let rm_bw: u8 = (rm & 0b1000) >> 1;
        let reg_bw: u8 = (reg & 0b1000) >> 3;
        let rex: u8 = 0b01000000 | (w as u8) << 3 | rm_bw | reg_bw;
        self.write_byte(rex);
    }

    pub fn write_mod(&mut self, mod_: Mod, rm: u8, reg: u8) {
        let mod_rm = (mod_ as u8) << 6 | ((reg & 0b111) << 3) | (rm & 0b111);
        self.write_byte(mod_rm);
    }
}