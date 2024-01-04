extern crate clap;

use clap::Parser;
use elf::endian::AnyEndian;
use elf::ElfBytes;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let file_data = std::fs::read(args.file).expect("Could not read file.");
    let file = ElfBytes::<AnyEndian>::minimal_parse(file_data.as_slice()).expect("Open test1");

    let common = file.find_common_data().expect("shdrs should parse");
    let (dynsyms, strtab) = (common.dynsyms.unwrap(), common.dynsyms_strs.unwrap());
    for sym in dynsyms.iter() {
        let sym_name = strtab.get(sym.st_name as usize).unwrap();
        if !sym_name.is_empty() && sym.st_value > 0 {
            println!("fun:{}=uninstrumented", sym_name);
            println!("fun:{}=discard", sym_name);
        }
    }
}
