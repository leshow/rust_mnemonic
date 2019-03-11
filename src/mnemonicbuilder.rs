#![allow(dead_code)]
use {
    crate::mnemonic::Mnemonic,
    rand::{distributions::Alphanumeric, rngs::OsRng, Rng},
    std::{
        fs::File,
        io::{Error, Read},
        path::Path,
    },
};

pub struct MnemonicBuilder<'a> {
    pub words_list: Vec<String>,
    seed: &'a str,
}

impl<'a> MnemonicBuilder<'a> {
    pub fn new() -> Result<MnemonicBuilder<'a>, Error> {
        let seed: &str = "seed";
        let path = Path::new("src/wordslist/english.txt");
        let mut string_from_file = String::new();

        File::open(&path)?.read_to_string(&mut string_from_file)?;

        let words_list: Vec<String> = string_from_file
            .split_whitespace()
            .map(|s| s.into())
            .collect();

        Ok(MnemonicBuilder { seed, words_list })
    }

    pub fn with_seed(self, seed: &'a str) -> MnemonicBuilder<'a> {
        MnemonicBuilder { seed, ..self }
    }

    pub fn with_words(self, words_list: Vec<String>) -> MnemonicBuilder<'a> {
        MnemonicBuilder { words_list, ..self }
    }

    pub fn create(&self) -> Result<Mnemonic, Error> {
        let random_chars: String = OsRng::new()?
            .sample_iter(&Alphanumeric)
            .take(crate::mnemonic::LENGTH)
            .collect();

        Ok(Mnemonic::new(&random_chars))
    }
}
