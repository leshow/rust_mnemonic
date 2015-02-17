use std::env::Args;
use getopts::Options;

pub struct RuntimeSettings {
    pub print_help: bool,
    pub program: String,
    pub seed: Option<String>,
}

impl RuntimeSettings {
    pub fn new(args: Args) -> RuntimeSettings {

        let mut opts = Options::new();

        opts.optflag("h", "help", "prints this help menu");
        opts.optopt("s", "seed", "set mnemonic seed", ""); //reqopt would make it required

        let matches = match opts.parse(args) {
            Ok(m) =>  m,
            Err(f) =>  panic!(f.to_string())
        };

        let prog = String::from_str("mnemonic");

        RuntimeSettings {
            print_help: matches.opt_present("h"),
            program: prog,
            seed: matches.opt_str("s"),
        }
    }

    pub fn print_usage(&self) {
        println!("Usage: {} [options]", self.program);
        println!("-s\t\tSeed");
        println!("-h --help\tUsage");
    }
}
