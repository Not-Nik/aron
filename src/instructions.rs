// aron (c) Nikolas Wipper 2022

#[derive(Debug, PartialEq, Clone)]
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

enum Operand {
    Register,
    RegisterDeref,
    ImmediateValue,
}

pub enum AddressingMode {
    Byte,
    Word,
    Dword,
    Qword,
}

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum Opcode {
        // Todo: maybe automate this?
        Mov,
        Push,
        Ret,
        Xor
    }
}

pub struct Instruction {
    opcode: Opcode,
    size: AddressingMode,
    operand1: Option<Operand>,
    operand2: Option<Operand>,
}

impl Instruction {
    /// New (OpCode-Less)
    pub fn new_ocl(opcode: Opcode, size: AddressingMode) -> Self {
        Instruction {
            opcode, size,
            operand1: None, operand2: None
        }
    }
}
