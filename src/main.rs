extern crate getopts;

extern crate lib;

use lib::mnemonic::Mnemonic;

use lib::mnemonicbuilder::MnemonicBuilder;
use lib::settings::RuntimeSettings;
use std::env;

fn main() {
    // start handling opts
    let settings = RuntimeSettings::new(env::args());
    if settings.print_help {
        settings.print_usage();
        return;
    }

    let builder = MnemonicBuilder::new().expect("Failed to open wordslist.");

    let mnemonic: Mnemonic = builder.create().expect("Cannot create mnemonic.");
    println!("{:?}", mnemonic);
    println!("{}", mnemonic.to_words(&builder.wordslist).join(" "));
}
