// aron (c) Nikolas Wipper 2022

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use clap::{command, Arg, App, PossibleValue};

use crate::ObjectFileType;

#[derive(Default)]
pub struct CommandLine {
    pub format: ObjectFileType,
    pub input: String,
    pub output: String,
}

pub fn get_app() -> App<'static> {
    let arg_format = Arg::new("format")
             .short('f')
             .long("format")
             .takes_value(true)
             .value_name("format")
             .help("Set the output binary format")
             .value_parser([
                PossibleValue::new("elf"),
                PossibleValue::new("macho"),
             ])
             .default_value(ObjectFileType::default().into());

    let arg_output = Arg::new("output file")
        .short('o')
        .long("output")
        .takes_value(true)
        .value_name("filename")
        .help("The output filename")
        .default_value("a.out");

    let arg_input_file = Arg::new("input file")
        .takes_value(true)
        .value_name("filename")
        .required(true);

    command!()
        .arg(arg_format)
        .arg(arg_output)
        .arg(arg_input_file)
}
