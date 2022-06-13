# This script generates a very naive matching algorithm, where, instead of intelligently parsing a line, with several
# branches, every possible instruction is brute-forced and the first one to not return an error is used.
# Todo: return the instruction that uses the least bytes

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

use crate::instructions::Instruction;
use crate::parse::lexer::Token;
use crate::parse::ParseError;
use crate::parse::helpers::*;""", file=types_header)
    funcs = []

    for instruction in instructions:
        print(f"""
fn matches_{instruction.name}{len(funcs) + 1}(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {{
    let mut iter = tokens.iter();
    
    if get_next(&mut iter)? != \"{instruction.name}\" {{ return Err((iter.count(), ParseError::InvalidInstruction)); }}""", file=types_header)

        funcs.append(f"matches_{instruction.name}{len(funcs) + 1}")

        rm = False
        reg = False
        imm = None

        for op in [instruction.op1, instruction.op2]:
            if not op.raw:
                continue

            if op == instruction.op2:
                print("""    if get_next(&mut iter)? != "," { return Err((iter.count(), ParseError::InvalidOperand)); }""", file=types_header)

            # todo: some instructions have two immediate values
            # todo: some instructions have two regs
            if op.is_imm():
                print(f"""    let imm = is_imm_of_size(&mut iter, {op.get_imm_size()})?;""", file=types_header)
                imm = op
            elif op.is_specific_operand():
                print(f"""    if get_next(&mut iter)? != "{op.raw}" {{ return Err((iter.count(), ParseError::InvalidOperand)); }}""", file=types_header)
            elif op.is_unspecific_reg():
                print(f"""    let reg = is_reg_of_size(&mut iter, {op.get_reg_size()})?;""", file=types_header)
                reg = True
            elif op.is_unspecific_rm():
                print(f"""    let rm = is_rm_of_size(&mut iter, {op.get_rm_size()})?;""", file=types_header)
                rm = True
            else:
                raise RuntimeError("Unsupported op type '" + op.raw + "'")

        print(f"""    if iter.next().is_some() {{ return Err((iter.count(), ParseError::ExtraneousTokenAfterInstruction)); }}
    let mut instr = Instruction::new("{instruction.name}".to_string());""", file=types_header)

        if rm:
            print("\n    let m = get_mod_from_rm(&rm);", file=types_header)

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
            print(f"    instr.write_offset(m, rm.0 as u8, {reg} as u8, rm.2);", file=types_header)

        if imm_write:
            print(imm_write, file=types_header)

        print("""
    Ok(instr)""", file=types_header)

        print("}", file=types_header)

    func_list = ", ".join(funcs)

    print(f"""
const MATCH_FUNCTIONS: [fn(&Vec<Token>) -> Result<Instruction, (usize, ParseError)>; {len(funcs)}] = [{func_list}];

pub fn matches(tokens: &Vec<Token>) -> Result<Instruction, (usize, ParseError)> {{
    let mut err: (usize, ParseError) = (tokens.len() - 1, ParseError::InvalidInstruction);
    
    for func in MATCH_FUNCTIONS {{
        let instr = func(tokens);
        if instr.is_ok() {{
            return Ok(instr.unwrap());
        }} else {{
            let instr_err = instr.unwrap_err();
            if instr_err.0 < err.0 {{
                err = instr_err;
            }}
        }}
    }}
    
    Err(err)
}}""", file=types_header)


if __name__ == "__main__":
    main()
