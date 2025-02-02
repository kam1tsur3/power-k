use clap::Parser;

#[derive(Parser)]
#[clap(
  name = "power-k",
  author = "kam1tsur3",
  version = "v0.1",
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
  pub parsed_funcs: Option<Vec<String>>
}

pub fn parse_cmdline () -> Result<ModeAndOptions, String> {
  let arg: AppArg = AppArg::parse();
  let mode = arg.mode.unwrap();
  let func_names = arg.func_names.unwrap();
  let parsed_funcs: Vec<String> = func_names.split(',').map(String::from).collect();

  if MODE_STRS.iter().any(|name| *name == mode) {
    match mode.as_str() {
      "auto" => {
        Ok(ModeAndOptions {
          mode: Mode::AUTO,
          dir: arg.dir.unwrap(),
          parsed_funcs: Some(parsed_funcs),
        })
      }
      "cert" => {
        Ok(ModeAndOptions {
          mode: Mode::CERT,
          dir: arg.dir.unwrap(),
          parsed_funcs: None
        })
      }
      "elf" => {
        Ok(ModeAndOptions {
          mode: Mode::ELF,
          dir: arg.dir.unwrap(),
          parsed_funcs: None
        })
      }
      "func" => {
        Ok(ModeAndOptions {
          mode: Mode::FUNC,
          dir: arg.dir.unwrap(),
          parsed_funcs: Some(parsed_funcs),
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
