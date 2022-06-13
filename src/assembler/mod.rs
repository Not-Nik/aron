// aron (c) Nikolas Wipper 2022

use crate::instructions::Instruction;
use crate::parse::{Directive, Line};
use object::write::{Mangling, StandardSection, Symbol, SymbolSection};
use object::{write, Architecture, BinaryFormat, Endianness, SymbolFlags, SymbolKind, SymbolScope};
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub struct Function {
    name: String,
    bytes: Vec<u8>,
    global: bool,
}

pub struct Module {
    functions: Vec<Function>,
}

pub enum ObjectFileType {
    Elf,
    MachO,
}

impl Function {
    pub fn new() -> Self {
        Function { name: String::new(), bytes: Vec::new(), global: true }
    }

    pub fn set_name(&mut self, name: &String) -> Result<(), ()> {
        if !self.name.is_empty() {
            Err(())
        } else {
            self.name = name.to_string();
            Ok(())
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn make_global(&mut self) {
        self.global = true;
    }

    pub fn write_instruction(&mut self, instruction: &Instruction) {
        // note: maybe don't write instructions if there is no name, bc if we use a later label to call,
        //  these instructions will never be executed. That way we'll basically remove dead instructions
        //  for free. Does this have any other side-effects if we do it before resolving local jumps?
        //  Can we even do it before we do that, bc detecting where a function ends requires that.
        self.bytes.extend(instruction.get_bytes());
    }
}

impl Module {
    pub fn from_lines(lines: Vec<Line>) -> Self {
        let mut functions = Vec::new();

        let mut last_label = String::new();
        let mut current_function = Function::new();

        // Todo: resolve local jumps so we can
        // Todo: keep track of jumps to see if the current instruction is the last in a block

        for line in lines {
            match line {
                Line::Directive(dir) => match dir {
                    Directive::Global(name) => {
                        if &name == current_function.get_name() {
                            current_function.make_global();
                        }
                    }
                    _ => {}
                },
                Line::Label(label) => last_label = label,
                Line::Instruction(instr) => {
                    current_function.write_instruction(&instr);

                    // Todo: don't end function if there is a jump over this instruction
                    if instr.has_name("ret") || instr.has_name("retf") {
                        functions.push(current_function);
                        current_function = Function::new();
                    }
                }
            }

            // ignore result
            let _ = current_function.set_name(&last_label);
        }

        Module { functions }
    }

    pub fn write_to_file<P: AsRef<Path>>(self, name: P, object_type: ObjectFileType) -> Result<(), Box<dyn Error>> {
        let file = File::options().create(true).write(true).open(name.as_ref())?;

        let binary_format = match object_type {
            ObjectFileType::Elf => BinaryFormat::Elf,
            ObjectFileType::MachO => BinaryFormat::MachO,
        };

        let mut object = write::Object::new(binary_format, Architecture::X86_64, Endianness::Little);

        object.mangling = Mangling::None;

        let text_id = object.section_id(StandardSection::Text);

        let mut text_offset = 0;

        for func in self.functions.into_iter() {
            let func_name = func.name;
            let func_data = func.bytes;

            let symbol = Symbol {
                name: func_name.into_bytes(),
                value: text_offset,
                size: func_data.len() as u64,
                kind: SymbolKind::Text,
                scope: if func.global { SymbolScope::Dynamic } else { SymbolScope::Compilation },
                weak: false,
                section: SymbolSection::Absolute,
                flags: SymbolFlags::None,
            };

            let symbol_id = object.add_symbol(symbol);

            text_offset = object.add_symbol_data(
                symbol_id,
                text_id,
                func_data.as_slice(),
                4, /*todo: read align from directives*/
            );
        }

        object.write_stream(file)?;

        Ok(())
    }
}
