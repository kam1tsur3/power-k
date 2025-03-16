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
                    let scan_config = mode_and_opt.skip_config.unwrap();
                    println!("[-][AUTO]: Running in auto mode.");
                    println!("[-][AUTO]: Target directory - {}", &dir);
                    println!("[-][AUTO]: Search functions - {:?}", funcs);
                    search_elf(dir.as_str(), &scan_config);
                    search_function(dir.as_str(), &funcs, &scan_config);
                }
                Mode::CERT => {
                    println!("[E][CERT]: Not implemented yet, sorry!");
                }
                Mode::ELF => {
                    let dir = mode_and_opt.dir;
                    let scan_config = mode_and_opt.skip_config.unwrap();
                    println!("[-][ELF]: Running in elf mode.");
                    println!("[-][ELF]: Target directory - {}", &dir);
                    search_elf(dir.as_str(), &scan_config);
                }
                Mode::FUNC => {
                    let dir = mode_and_opt.dir;
                    let funcs = mode_and_opt.parsed_funcs.unwrap();
                    let scan_config = mode_and_opt.skip_config.unwrap();
                    println!("[-][ELF]: Running in elf mode.");
                    println!("[-][ELF]: Target directory - {}", &dir);
                    println!("[-][AUTO]: Search functions - {:?}", funcs);
                    search_function(dir.as_str(), &funcs, &scan_config);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
