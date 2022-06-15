// aron (c) Nikolas Wipper 2022

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io::Write;
use crate::assembler::label::LabelMap;
use crate::instructions::{Instruction, Reference};

pub struct Section {
    pub bytes: Vec<u8>,
    pub references: Vec<Reference>,
    pub label_map: LabelMap,
}

impl Section {
    pub fn new() -> Self {
        Section { bytes: Vec::new(), references: Vec::new(), label_map: LabelMap::new() }
    }

    pub fn at(&self) -> usize {
        self.bytes.len()
    }

    pub fn write_instruction(&mut self, instruction: &Instruction) {
        let reloc_offset = self.bytes.len();

        self.bytes.extend(instruction.get_bytes());
        for r in instruction.get_refs() {
            let new_r = Reference { to: r.to.clone(), at: r.at + reloc_offset, rel: r.rel };
            self.references.push(new_r);
        }
    }

    pub fn write_string(&mut self, string: String) {
        self.bytes.write(string.as_bytes()).unwrap();
        self.bytes.write(&[0]).unwrap();
    }
}
