mod cmdline;
use cmdline::*;

mod elf;
use elf::*;


fn main() {
    let parse_result = parse_cmdline();
    match parse_result {
        Ok(mode_and_opt) => {
            match mode_and_opt.mode {
                Mode::AUTO => {
                    let dir = mode_and_opt.dir;
                    let funcs = mode_and_opt.parsed_funcs.unwrap();
                    println!("[-][AUTO]: Running in auto mode.");
                    println!("[-][AUTO]: Target directory - {}", &dir);
                    println!("[-][AUTO]: Search functions - {:?}", funcs);
                    search_elf(dir.as_str());
                    search_function(dir.as_str(), &funcs);
                }
                Mode::CERT => {
                    println!("[E][CERT]: Not implemented yet, sorry!");
                }
                Mode::ELF => {
                    let dir = mode_and_opt.dir;
                    println!("[-][ELF]: Running in elf mode.");
                    println!("[-][ELF]: Target directory - {}", &dir);
                    search_elf(dir.as_str());
                }
                Mode::FUNC => {
                    let dir = mode_and_opt.dir;
                    let funcs = mode_and_opt.parsed_funcs.unwrap();
                    println!("[-][ELF]: Running in elf mode.");
                    println!("[-][ELF]: Target directory - {}", &dir);
                    println!("[-][AUTO]: Search functions - {:?}", funcs);
                    search_function(dir.as_str(), &funcs);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
