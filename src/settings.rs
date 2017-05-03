use getopts::Options;
use std::env::Args;
use std::str::FromStr;

pub struct RuntimeSettings {
    pub print_help: bool,
    pub program: String,
    pub seed: Option<String>,
    pub mode: Mode,
}

pub enum Mode {
    Default,
    Num(u8),
}

impl RuntimeSettings {
    pub fn new(args: Args) -> RuntimeSettings {

        let mut opts = Options::new();

        opts.optflag("h", "help", "prints this help menu");
        opts.optopt("s", "seed", "set mnemonic seed", ""); //reqopt would make it required
        opts.optopt("n", "num", "set mnemonic seed", "");

        let matches = match opts.parse(args) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };

        let program = String::from_str("mnemonic").unwrap();

        RuntimeSettings {
            print_help: matches.opt_present("h"),
            program,
            seed: matches.opt_str("s"),
            mode: Mode::Default,
        }
    }

    pub fn print_usage(&self) {
        println!("Usage: {} [options]", self.program);
        println!("-s\t\tSeed");
        println!("-h --help\tUsage");
    }
}
