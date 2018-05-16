#![allow(dead_code)]
use {
    mnemonic::Mnemonic, rand::{OsRng, Rng},
    std::{
        fs::File, io::{Error, Read}, path::Path,
    },
};

static LENGTH: usize = 32;

pub struct MnemonicBuilder<'a> {
    pub words_list: Vec<String>,
    seed: &'a str,
    bit_length: usize,
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

        Ok(MnemonicBuilder {
            seed,
            words_list,
            bit_length: LENGTH,
        })
    }

    pub fn with_seed(self, seed: &'a str) -> MnemonicBuilder<'a> {
        MnemonicBuilder { seed, ..self }
    }

    pub fn with_words(self, words_list: Vec<String>) -> MnemonicBuilder<'a> {
        MnemonicBuilder { words_list, ..self }
    }

    pub fn create(&self) -> Result<Mnemonic, Error> {
        let random_chars: String = OsRng::new()?
            .gen_ascii_chars()
            .take(self.bit_length)
            .collect();

        Ok(Mnemonic::new(&random_chars))
    }
}
