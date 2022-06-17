// aron (c) Nikolas Wipper 2020

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate custom_derive;
extern crate enum_derive;

use crate::assembler::{Module, ObjectFileType};
use crate::cli::parse_command_line;
use crate::parse::parser::parse_lines;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

mod assembler;
mod cli;
mod instructions;
mod number;
mod parse;

fn main() {
    let cline = parse_command_line();

    let path = Path::new(&cline.input);
    if path.extension().unwrap() == OsStr::new("o") {
        eprintln!("Skipping {}, has .o extension", cline.input);
        exit(0);
    }

    let mut file = File::open(&path).unwrap();

    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    let parsed_lines = parse_lines(cline.input.clone(), code);

    if let Ok(parsed_lines) = parsed_lines {
        let module = Module::from_lines(parsed_lines);

        module.write_to_file(Path::new(&cline.output), ObjectFileType::MachO).expect("Couldn't write module");
    } else {
        exit(1);
    }
}
