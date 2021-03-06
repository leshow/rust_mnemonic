#[macro_use]
extern crate nom;

mod mnemonic;
mod mnemonicbuilder;
mod settings;

use {
    crate::{mnemonic::Mnemonic, mnemonicbuilder::MnemonicBuilder, settings::RuntimeSettings},
    std::env,
};

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
    println!("{}", mnemonic.to_words(&builder.words_list).join(" "));
}
