#![allow(dead_code)]
// crates

// lib

use mnemonic::Mnemonic;
use rand::{OsRng, Rng};
// std
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

static LENGTH: usize = 32;

pub struct MnemonicBuilder<'a> {
    pub wordslist: Vec<String>,
    seed: &'a str,
    bit_length: usize,
}

impl<'a> MnemonicBuilder<'a> {
    pub fn new() -> Result<MnemonicBuilder<'a>, Error> {
        let str_seed: &str = "seed";
        let path = Path::new("src/wordslist/english.txt");
        let mut string_from_file = String::new();

        File::open(&path)?
            .read_to_string(&mut string_from_file)?;

        let words: Vec<String> = string_from_file
            .split_whitespace()
            .map(|s| s.into())
            .collect();

        Ok(
            MnemonicBuilder {
                seed: str_seed,
                wordslist: words,
                bit_length: LENGTH,
            },
        )
    }

    pub fn with_seed(self, new_seed: &'a str) -> MnemonicBuilder<'a> {
        MnemonicBuilder {
            seed: new_seed,
            ..self
        }
    }

    pub fn with_words(self, new_wordslist: Vec<String>) -> MnemonicBuilder<'a> {
        MnemonicBuilder {
            wordslist: new_wordslist,
            ..self
        }
    }

    pub fn create(&self) -> Result<Mnemonic, Error> {
        let random_chars: String = OsRng::new()?
            .gen_ascii_chars()
            .take(self.bit_length)
            .collect();

        Ok(Mnemonic::new(&random_chars))
    }
}
