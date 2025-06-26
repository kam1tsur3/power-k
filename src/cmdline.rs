use std::io::{Read, BufRead, BufReader};
use std::fs::File;
use std::iter::Scan;
use regex::Regex;
use std::path::Path;
use std::collections::HashSet;

use clap::Parser;

#[derive(Parser)]
#[clap(
  name = "power-k",
  author = "kam1tsur3",
  version = "v0.2",
  about = "A tool for IoT research"
)]
struct AppArg {
  /* arbitrary */

  /// Running mode: ["auto", "pem", "elf", "func"]
  #[clap(short='m', long="mode", default_value="auto")]
  mode: Option<String>,

  /// Using in elf and func modes
  #[clap(short='d', long="directory", default_value="./")]
  dir: Option<String>,

  /// Using in func mode
  #[clap(short='f', long="func_names", default_value="system,strcpy")]
  func_names: Option<String>,
  
  /// Using to config skip directory and filename
  #[clap(short='s', long="skip_config", default_value="")]
  skip_config: Option<String>,
}


pub enum Mode {
  AUTO,
  CERT,
  ELF,
  FUNC
}

static MODE_STRS: [&str; 4] = ["auto", "cert", "elf", "func"];

pub struct ModeAndOptions {
  pub mode: Mode,
  pub dir: String,
  pub parsed_funcs: Option<Vec<String>>,
  pub skip_config: Option<ScanConfig>,
}

#[derive(Default)]
pub struct ScanConfig{
  exclude_extensions: HashSet<String>,
  exclude_directories: HashSet<String>,
  regex_patterns: Vec<Regex>,
}

impl ScanConfig {
  pub fn new() -> Self {
    ScanConfig {
      exclude_extensions: HashSet::new(),
      exclude_directories: HashSet::new(),
      regex_patterns: Vec::new(),
    }
  }

  pub fn from_file(file_path: &str) -> Result<Self, std::io::Error> {

    if file_path.trim().is_empty() {
        return Ok(ScanConfig::new())
    }
    
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", file_path)
        ));
    }
    
    if !path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Path is not a file: {}", file_path)
        ));
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut config = ScanConfig::new();
    let mut section = "";

    for line in reader.lines() {
      let line = line?;
      let trimmed = line.trim();

      if trimmed.is_empty() || trimmed.starts_with('#') {
        continue;
      }

      if trimmed == "[extension]" {
        section = "extension";
        continue;
      } else if trimmed == "[directory]" {
        section = "directory";
        continue;
      } else if trimmed == "[regex]" {
        section = "regex";
        continue;
      }

      match section {
        "extension" => {
          config.exclude_extensions.insert(trimmed.to_string());
        },
        "directory" => {
          config.exclude_directories.insert(trimmed.to_string());
        },
        "regex" => {
          if let Ok(regex) = Regex::new(trimmed) {
            config.regex_patterns.push(regex);
          } else {
            eprintln!("[Warning]: Invalid regex pattern: {}", trimmed);
          }
        },
        _ => {}
      }
    }
    
    if config.regex_patterns.is_empty() {
      config.regex_patterns.push(Regex::new(r"\.so(=.d+)*$").unwrap());
    }

    Ok(config)
  }

  pub fn is_skip_file(&self, path: &Path) -> bool {
    if let Some(parent) = path.parent() {
      let parent_str = parent.to_string_lossy();
      for dir in &self.exclude_directories {
        if parent_str.starts_with(dir) {
          return true;
        }
      }
    }

    if let Some(extension) = path.extension() {
      if let Some(ext_str) = extension.to_str() {
        if self.exclude_extensions.contains(ext_str) {
          return true;
        }
      }
    }

    if let Some(file_name) = path.file_name() {
      if let Some(file_name_str) = file_name.to_str() {
        for pattern in &self.regex_patterns {
          if pattern.is_match(file_name_str) {
            return true;
          }
        }
      }
    }

    false
  }
}

pub fn parse_cmdline () -> Result<ModeAndOptions, String> {
  let arg: AppArg = AppArg::parse();
  let mode = arg.mode.unwrap();
  let func_names = arg.func_names.unwrap();
  let skip_config = ScanConfig::from_file(&arg.skip_config.unwrap()).unwrap();
  let parsed_funcs: Vec<String> = func_names.split(',').map(String::from).collect();

  if MODE_STRS.iter().any(|name| *name == mode) {
    match mode.as_str() {
      "auto" => {
        Ok(ModeAndOptions {
          mode: Mode::AUTO,
          dir: arg.dir.unwrap(),
          parsed_funcs: Some(parsed_funcs),
          skip_config: Some(skip_config),
        })
      }
      "cert" => {
        Ok(ModeAndOptions {
          mode: Mode::CERT,
          dir: arg.dir.unwrap(),
          parsed_funcs: None,
          skip_config: None,
        })
      }
      "elf" => {
        Ok(ModeAndOptions {
          mode: Mode::ELF,
          dir: arg.dir.unwrap(),
          parsed_funcs: None,
          skip_config: Some(skip_config),
        })
      }
      "func" => {
        Ok(ModeAndOptions {
          mode: Mode::FUNC,
          dir: arg.dir.unwrap(),
          parsed_funcs: Some(parsed_funcs),
          skip_config: Some(skip_config),
        })
      }
      _ => {
        Err(String::from("[E]: Unintend Error in parse_cmdline"))
      }
    } 
  } else {
    Err(format!("[E]: Option -m must be one of {:?}", MODE_STRS))
  }
}
