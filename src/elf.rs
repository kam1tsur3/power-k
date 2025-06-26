use std::iter::Scan;
use std::path::Path;
use std::io::Read;
use std::fs::File;
use goblin::elf::Elf;
use goblin::elf::sym::STT_FUNC;
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

fn contains_function(path: &Path, f_name: &str) -> Option<String> {
    if let Ok(mut file) = File::open(path) {
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_ok() {
            if let Ok(elf) = Elf::parse(&buffer) {
                let mut is_internal = false;
                let mut is_external = false;

                for sym in elf.syms.iter() {
                    if let Some(name) = elf.strtab.get(sym.st_name) {
                        if let Ok(name) = name {
                            if name == f_name {
                                if sym.st_shndx != 0 {
                                    is_internal = true;
                                }
                            }
                        }
                    }
                }

                for sym in elf.dynsyms.iter() {
                    if let Some(name) = elf.dynstrtab.get(sym.st_name) {
                        if let Ok(name) = name {
                            if name == f_name {
                                if sym.st_shndx == 0 {
                                    is_external = true;
                                } else if sym.st_shndx != 0 {
                                    is_internal = true;
                                }
                            }
                        }
                    }
                }

                if is_internal && is_external {
                    return Some("internal+external".to_string());
                } else if is_internal {
                    return Some("internal".to_string());
                } else if is_external {
                    return Some("external".to_string());
                }
            }
        }
    }
    None
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
                if let Some(function_type) = contains_function(path, func_name) {
                    println!("[+][FUNC][{}]: Function {}() found in {}.", 
                             function_type.to_uppercase(), func_name, path.display());
                }
            }
        }
    }
}
