import re
import csv


class Operand:
    def __init__(self, op: str):
        self.raw = op
        if self.raw:
            self.raw = self.raw.lower()

    def is_imm(self) -> bool:
        return self.raw and self.raw.startswith("imm")

    def get_imm_size(self) -> int:
        if self.raw:
            return int(self.raw[3:])

    def is_specific_operand(self) -> bool:
        return self.raw and self.raw in ["al", "ah", "ax", "eax", "rax", "cl", "ds", "dx", "es", "ss", "gs", "fs", "cs", "0",
                                         "1", "3"]

    def is_unspecific_reg(self) -> bool:
        return self.raw and self.raw.startswith("r") and not self.raw.startswith("r/m") and not self.raw.startswith("rel")

    def get_reg_size(self) -> int:
        if self.raw:
            return int(self.raw[1:])

    def is_unspecific_rm(self) -> bool:
        return self.raw and self.raw.startswith("r/m")

    def get_rm_size(self) -> int:
        if self.raw:
            return int(self.raw[3:])


class InstructionTemplate:
    def __init__(self, opcode, name, op1, op2):
        self.opcode = opcode
        self.name = name.lower()
        self.op1 = Operand(op1)
        self.op2 = Operand(op2)


def main():
    types_header = open("encodings.rs", 'w')

    file = open("x86-csv/x86.csv", mode='r')
    csv_reader = csv.reader(file)

    lines = []
    for line in csv_reader:
        lines.append((line[0], line[1]))
    lines.pop(0)

    instructions = []
    for line in lines:
        instr = line[0]
        opc = line[1]

        if "NP" in opc or "VEX" in opc:
            continue

        unsupported_op_types = ["bnd", "bnd1", "bnd1/m64", "bnd1/m128", "CR0-CR7", "CR8", "DR0-DR7", "k1", "m", "mem",
                                "mm", "mm1", "moffs8","moffs16", "moffs32", "moffs64", "m8", "m16", "m16int", "m2byte",
                                "m32", "m32fp", "m32int", "m512", "m512byte", "m64", "m64f", "m64fp", "m64int",
                                "m80bcd", "m80dec", "m80fp", "m128", "m14/28byte", "m16:16", "m16:32", "m16:64",
                                "m16&16", "m16&32", "m16&64", "m32&32", "m94/108byte", "ptr16:16", "ptr16:32", "reg",
                                "rel", "rel8", "rel16", "rel32", "r16/m16", "r32/m16", "r32/m32", "r64/m16", "r64/m64",
                                "Sreg", "ST(i)", "ST(0)", "vm32y", "vm32z", "vm64z", "xmm", "xmm1", "xmm1/m32",
                                "xmm1/m64", "xmm2", "xmm3/m128", "ymm1"]

        opcode_name_overrides = {
            "CB": "RETF",
            "CA iw": "RETF"
        }

        sp = re.split(" , | ,|, | |,", instr)
        if len(sp) > 3:
            continue
        elif len(sp) == 3:
            if sp[1] in unsupported_op_types or sp[2] in unsupported_op_types:
                continue
            instructions.append(InstructionTemplate(opc, sp[0], sp[1], sp[2]))
        elif len(sp) == 2:
            if sp[1] in unsupported_op_types:
                continue
            instructions.append(InstructionTemplate(opc, sp[0], sp[1], None))
        elif len(sp) == 1:
            if opc in opcode_name_overrides:
                sp[0] = opcode_name_overrides[opc]

            instructions.append(InstructionTemplate(opc, sp[0], None, None))

    print("""// aron (c) Nikolas Wipper 2022

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
}""", file=types_header)
    funcs = []

    for instruction in instructions:
        print(f"""
fn matches_{instruction.name}{len(funcs) + 1}(tokens: &Vec<String>) -> Option<Instruction> {{
    let mut iter = tokens.iter();
    
    if iter.next().unwrap() != \"{instruction.name}\" {{ return None; }}""", file=types_header)

        funcs.append(f"matches_{instruction.name}{len(funcs) + 1}")

        rm = False
        reg = False
        imm = None

        for op in [instruction.op1, instruction.op2]:
            if not op.raw:
                continue

            if op == instruction.op2:
                print("    if iter.next().unwrap() != \",\" { return None; }", file=types_header)

            # todo: some instructions have two immediate values
            # todo: some instructions have two regs
            if op.is_imm():
                print(f"""    let imm = is_imm_of_size(&mut iter, {op.get_imm_size()});
    if imm.is_none() {{ return None; }}
    let imm = imm.unwrap();""", file=types_header)
                imm = op
            elif op.is_specific_operand():
                print(f"    if iter.next().unwrap() != \"{op.raw}\" {{ return None; }}", file=types_header)
            elif op.is_unspecific_reg():
                print(f"""    let reg = is_reg_of_size(&mut iter, {op.get_reg_size()});
    if reg.is_none() {{ return None; }}
    let reg = reg.unwrap();""", file=types_header)
                reg = True
            elif op.is_unspecific_rm():
                print(f"""    let rm = is_rm_of_size(&mut iter, {op.get_rm_size()});
    if rm.is_none() {{ return None; }}
    let rm = rm.unwrap();""", file=types_header)
                rm = True
            else:
                raise RuntimeError("Unsupported op type '" + op.raw + "'")

        print(f"""    if iter.next().is_some() {{ return None; }}
    let mut instr = Instruction::new("{instruction.name}".to_string());""", file=types_header)

        if rm:
            print("""
    let m = if let Some(off) = rm.2 {
        if off < 128 && off > -128 {
            Offset8Bit
        } else {
            Offset32Bit
        }
    } else {
        rm.1
    };""", file=types_header)

        opcode = instruction.opcode \
            .replace("REX +", "REX") \
            .replace("REX.W +", "REX.W") \
            .replace(" +r", "+r") \
            .replace("/", " /") \
            .replace("  ", " ")

        fill_reg = None

        parts = opcode.split(" ")

        for index, p in enumerate(parts):
            if p.startswith("/"):
                fill_reg = parts.pop(index)[1:]

        # if rm doesnt exist, reg doesnt either
        if not reg:
            if fill_reg and fill_reg != "r":
                reg = fill_reg
            else:
                reg = "0"
        else:
            reg = "reg"

        if not rm:
            rm = "0"
        else:
            rm = "rm.0"

        imm_write = None

        for part in parts:
            if part.startswith("REX"):
                w = "true" if part.endswith(".W") else "false"
                print(f"    instr.write_rex({w}, {rm} as u8, {reg} as u8);", file=types_header)
            elif part.startswith("i"):
                imm_write = f"    instr.write_num(imm as i{imm.get_imm_size()});"
            else:
                part = part.replace("rb", "reg as u8").replace("rw", "reg as u8").replace("rd", "reg as u8")
                print(f"    instr.write_byte(0x{part});", file=types_header)

        if rm == "rm.0":
            print(f"    instr.write_mod(m, {rm} as u8, {reg} as u8);", file=types_header)

            print("""
    if let Some(off) = rm.2 {
        if m == Offset32Bit {
            instr.write_num(off);
        } else if m == Offset8Bit {
            instr.write_num(off as i8);
        }
    }""", file=types_header)

        if imm_write:
            print(imm_write, file=types_header)

        print("""
    Some(instr)""", file=types_header)

        print("}", file=types_header)

    print("""
pub fn matches(tokens: &Vec<String>) -> Option<Instruction> {""", file=types_header)

    func_calls = []
    for func in funcs:
        func_calls.append(f"if let Some(instr) = {func}(tokens) {{ Some(instr) }}\n")

    print("    " + "    else ".join(func_calls), file=types_header, end="")
    print("""    else { None }
}""", file=types_header)


if __name__ == "__main__":
    main()
