use std::iter::Scan;
use std::path::Path;
use std::io::Read;
use std::fs::File;

use goblin::elf::Elf;
use walkdir::WalkDir;

use crate::cmdline::ScanConfig;

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

pub fn search_elf(dir_name: &str, scan_config: &ScanConfig) {
    for entry in WalkDir::new(dir_name).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() && is_elf_file(path) && !scan_config.is_skip_file(path){
            println!("[+][ELF]: {} is ELF file.", path.display());
        }
    }

}

pub fn search_function(dir_name: &str, func_names: &Vec<String>, scan_config: &ScanConfig) {
    for entry in WalkDir::new(dir_name).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() && is_elf_file(path) && !scan_config.is_skip_file(path){
            for func_name in func_names.iter() {
                if contains_function(path, func_name) {
                    println!("[+][FUNC]: Function {}() found in {}.", func_name, path.display());
                }
            }
        }
    }

}