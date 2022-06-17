// aron (c) Nikolas Wipper 2022

use std::process::exit;
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

pub fn parse_command_line() -> CommandLine {
    let mut args = std::env::args().collect::<Vec<String>>();

    if args.is_empty() {
        panic!("Called aron with argc=0");
    }

    let program_name = args.remove(0);

    let mut cline = CommandLine::default();

    while !args.is_empty() {
        let arg = args.remove(0);
        if arg == "--help" {
            help(program_name);
        } else if arg == "-f" {
            if !args.is_empty() {
                let format = args.remove(0);
                cline.format = match &*format {
                    "elf" => ObjectFileType::Elf,
                    "macho" => ObjectFileType::MachO,
                    _ => panic!("Invalid format string '{}'", format)
                }
            } else {
                panic!("Used -f without specifying a format");
            }
        } else if arg == "-o" {
            if !args.is_empty() {
                if cline.output.is_empty() {
                    cline.output = args.remove(0);
                } else {
                    panic!("Specified two output files");
                }
            } else {
                panic!("Used -f without specifying a output");
            }
        } else {
            if cline.input.is_empty() {
                cline.input = arg;
            } else {
                panic!("Specified two input files");
            }
        }
    }

    if cline.input.is_empty() {
        help(program_name);
    }

    if cline.output.is_empty() {
        let last_period = cline.input.rfind('.');
        cline.output = if let Some(last_period) = last_period {
            let mut o = cline.input.clone();
            o.replace_range(last_period.., ".o");
            o
        } else {
            cline.input.clone() + ".o"
        }
    }

    cline
}
