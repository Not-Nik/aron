// aron (c) Nikolas Wipper 2022

mod label;
mod section;

use crate::assembler::section::Section;
use crate::parse::helpers::Relativity;
use crate::parse::{Directive, Line};
use object::write::{Mangling, Relocation, StandardSection, Symbol, SymbolSection};
use object::{
    write, Architecture, BinaryFormat, Endianness, RelocationEncoding, RelocationKind, SectionKind, SymbolFlags,
    SymbolKind, SymbolScope,
};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub struct Module {
    sections: HashMap<String, Section>,
}

pub enum ObjectFileType {
    Elf,
    MachO,
}

impl Module {
    pub fn from_lines(lines: Vec<Line>) -> Self {
        let mut sections = HashMap::new();
        sections.insert("__TEXT,__text".to_string(), Section::new());

        let mut current_section = sections.get_mut("__TEXT,__text").unwrap();

        for line in lines {
            match line {
                Line::Directive(dir) => match dir {
                    Directive::Asciz(string) => {
                        current_section.write_string(string);
                    }
                    Directive::Global(name) => {
                        current_section.label_map.make_global(name);
                    }
                    Directive::Section(name) => {
                        if !sections.contains_key(name.as_str()) {
                            sections.insert(name.clone(), Section::new());
                        }
                        current_section = sections.get_mut(name.as_str()).unwrap()
                    }
                    _ => {}
                },
                Line::Label(label) => current_section.label_map.insert_label(label, current_section.at()),
                Line::Instruction(instr) => {
                    current_section.write_instruction(&instr);
                }
            }
        }

        Module { sections }
    }

    pub fn write_to_file<P: AsRef<Path>>(self, name: P, object_type: ObjectFileType) -> Result<(), Box<dyn Error>> {
        let file = File::options().create(true).write(true).open(name.as_ref())?;

        let binary_format = match object_type {
            ObjectFileType::Elf => BinaryFormat::Elf,
            ObjectFileType::MachO => BinaryFormat::MachO,
        };

        let mut object = write::Object::new(binary_format, Architecture::X86_64, Endianness::Little);

        object.mangling = Mangling::None;

        let mut relocations = Vec::new();

        for (name, sec) in self.sections {
            let mut code = false;

            let section = match &*name {
                "text" | "__TEXT,__text" => {
                    code = true;
                    object.section_id(StandardSection::Text)
                }
                "data" | "__DATA,__data" => object.section_id(StandardSection::Data),
                "rodata" | "__TEXT,__const" | "__DATA,__const" | "__TEXT,__literal4" => object.section_id(StandardSection::ReadOnlyData),
                "rodata.str" | "__TEXT,__cstring" => object.section_id(StandardSection::ReadOnlyString),
                "bss" | "__DATA,__bss" => object.section_id(StandardSection::UninitializedData),
                // Todo: do the other standard sections
                _ => {
                    let mut s = name.split(',');
                    let segment = s.next().unwrap().as_bytes().to_vec();
                    let section = s.next().unwrap().as_bytes().to_vec();

                    object.add_section(segment, section, SectionKind::Unknown)
                }
            };

            object.append_section_data(section, &*sec.bytes, 4 /*todo: read align from directives*/);

            for label in sec.label_map.iter() {
                let symbol = Symbol {
                    name: label.name.into_bytes(),
                    value: label.at as u64,
                    size: 0,
                    kind: if code { SymbolKind::Text } else { SymbolKind::Data },
                    scope: if label.global { SymbolScope::Dynamic } else { SymbolScope::Linkage },
                    weak: false,
                    section: SymbolSection::Absolute,
                    flags: SymbolFlags::None,
                };

                let symbol_id = object.add_symbol(symbol);
                object.set_symbol_data(symbol_id, section, label.at as u64, 0);
            }

            for rel in sec.references {
                relocations.push((section, rel));
            }
        }

        for rel in relocations {
            let to_op = object.symbol_id(rel.1.to.as_bytes());
            let to = if let Some(to_op) = to_op {
                to_op
            } else {
                eprintln!("Implicitly importing '{}'", rel.1.to);
                let symbol = Symbol {
                    name: rel.1.to.into_bytes(),
                    value: 0,
                    size: 0,
                    kind: SymbolKind::Unknown,
                    scope: SymbolScope::Unknown,
                    weak: false,
                    section: SymbolSection::Undefined,
                    flags: SymbolFlags::None,
                };

                object.add_symbol(symbol)
            };

            let kind = match rel.1.rel {
                Relativity::Absolute => RelocationKind::Absolute,
                Relativity::Relative | Relativity::RipRelative => RelocationKind::Relative,
            };

            let encoding = match rel.1.rel {
                Relativity::Absolute => RelocationEncoding::X86Signed,
                Relativity::Relative => RelocationEncoding::X86Branch,
                Relativity::RipRelative => RelocationEncoding::X86RipRelative,
            };

            let relocation = Relocation {
                offset: rel.1.at as u64,
                size: 32, // Todo: This is hardcoded atm
                kind,
                encoding,
                symbol: to,
                addend: match object_type {
                    ObjectFileType::Elf => 0,
                    ObjectFileType::MachO => -4,
                },
            };

            object.add_relocation(rel.0, relocation)?;
        }

        object.write_stream(file)?;

        Ok(())
    }
}
