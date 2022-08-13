// aron (c) Nikolas Wipper 2022

use std::process::exit;
use clap::{command, Arg, App, PossibleValue};

use crate::ObjectFileType;

#[derive(Default)]
pub struct CommandLine {
    pub format: ObjectFileType,
    pub input: String,
    pub output: String,
}

fn help(argv1: String) -> ! {
    println!("Usage: {} [options] filename", argv1);
    println!(" Options:");
    println!("   --help       Prints this message");
    println!("   -f format    Set the output binary format");
    println!("      elf           ELF (64-bit)");
    println!("      macho         Mach-O");
    println!("   -o filename  Set output filename");

    exit(0);
}

pub fn get_app() -> App<'static> {
    let arg_format = Arg::new("format")
             .short('f')
             .long("format")
             .takes_value(true)
             .value_name("FORMAT")
             .help("Set the output binary format")
             .value_parser([
                PossibleValue::new("elf"),
                PossibleValue::new("macho"),
             ])
             .default_value("elf");

    let arg_output = Arg::new("output file")
        .short('o')
        .long("output")
        .takes_value(true)
        .value_name("OUTPUT_FILE")
        .help("The output filename")
        .default_value("a.out");

    let arg_input_file = Arg::new("input file")
        .takes_value(true)
        .value_name("INPUT_FILE")
        .required(true);

    command!()
        .arg(arg_format)
        .arg(arg_output)
        .arg(arg_input_file)
}
