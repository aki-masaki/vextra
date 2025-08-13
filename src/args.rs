const HELP: &str = "\
Vextra

USAGE:
  vextra [OPTIONS] [INPUT]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --output PATH         Sets an output path [default: /out]

ARGS:
  <INPUT> The .vex file to parse
";

#[derive(Debug)]
pub struct AppArgs {
    pub input: std::path::PathBuf,
    pub output: Option<std::path::PathBuf>,
}

pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        output: pargs.opt_value_from_os_str("--output", parse_path)?,
        input: pargs.free_from_str()?,
    };

    pargs.finish();

    Ok(args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}
