extern crate getopts;

extern crate lib;

use std::env;

use lib::mnemonicbuilder::MnemonicBuilder;
use lib::mnemonic::Mnemonic;
use lib::settings::RuntimeSettings;

fn main() {
    // start handling opts
    let settings = RuntimeSettings::new(env::args());
    if settings.print_help {
        settings.print_usage();
        return;
    }
    // TODO handle opts
    let builder = MnemonicBuilder::new().expect("Failed to open wordslist.");

    let mnemonic: Mnemonic = builder.create().expect("Cannot create mnemonic.");
    println!("{:?}", mnemonic);
    println!("{}", mnemonic.to_words(&builder.wordslist).join(" "));
}
