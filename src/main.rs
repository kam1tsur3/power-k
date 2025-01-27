use std::env;
use std::path::Path;
use std::io::Read;
use std::fs::File;

use goblin::elf::Elf;

use walkdir::WalkDir;

fn print_usage() {
    println!("Usage: ./binary <directory> <function_name>");
}

fn is_elf_file(path: &Path) -> bool {
    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0u8; 4];
        if file.read_exact(&mut buffer).is_ok() {
            return &buffer == b"\x7fELF";
        }
    }
    false
}

fn contains_function(path: &Path, f_name: &str) -> bool {
    if let Ok(mut file) = File::open(path) {
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_ok() {
            if let Ok(elf) = Elf::parse(&buffer) {
                for sym in elf.dynsyms.iter() {
                    if let Some(name) = elf.dynstrtab.get(sym.st_name) {
                        if let Ok(name) = name {
                            if name == f_name {
                                return true;
                            }
                        }
                    } 
                }
            }
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let target_dir = &args[1];
    let target_function = &args[2];

    for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() && is_elf_file(path) {
            if contains_function(path, target_function) {
                println!("Function {}() found in {}", target_function, path.display());
            }
        }
    }
}
