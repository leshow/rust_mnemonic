extern crate getopts;
extern crate lib;
extern crate rand;
extern crate rustc_serialize;

use lib::mnemonic::Mnemonic;
use lib::settings::RuntimeSettings;

use rustc_serialize::hex::ToHex;
use std::iter::repeat;
use std::str::FromStr;
use std::io::prelude::*;
use rand::{OsRng, Rng};
use std::fs::File;
use std::env;
use std::path::Path;
use std::error::Error;
use std::isize;

fn main() {
    /* start handling opts */
    let settings = RuntimeSettings::new(env::args());
    if settings.print_help {
        settings.print_usage();
        return;
    }

    let str_seed: &str = &settings.seed.unwrap_or(String::from_str("seed").unwrap());

    let mut rng = match OsRng::new() {
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS RNG: {}", e)
    };

    let path = Path::new("src/wordslist/english.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut string_from_file = String::new();
    match file.read_to_string(&mut string_from_file) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => println!("read to string_from_file"),
    };

    let words: Vec<_> = string_from_file.split_whitespace().collect();

    //generate corner cases
    for &i in [16usize, 24, 32].iter() {
        for &n in ["00", "7f", "80", "ff"].iter() {
            let corner_chars = repeat(n).take(i).collect();
            process(corner_chars, str_seed, &words);
        }
    }

    //generate random seeds
    for gen_seed in 0usize .. 12 {
        let length = 8 * (gen_seed % 3 + 2);
        let random_chars:String = rng.gen_ascii_chars().take(length).collect();

        process(random_chars, str_seed, &words);
    }
}

fn process(random_chars: String, str_seed: &str, words: &[&str]) {
    println!("random characters: {}",random_chars);

    let mnemonic: Mnemonic = Mnemonic::new(random_chars);
    let mut mnem_words = Vec::new();

    for i in 0usize .. mnemonic.binary_hash.len() / 11 {
        let bin_idx = &mnemonic.binary_hash[i * 11 .. (i + 1) * 11];
        let idx = isize::from_str_radix(bin_idx, 2).unwrap();

        mnem_words.push(words[idx as usize]);
    }

    let str_mnemonic = format!("{:?}",mnem_words);
    println!("mnemonic: {}", str_mnemonic);

    let key_value = mnemonic.to_seed(&str_mnemonic, str_seed); //to_string() on a Vec<&str>?
    println!("key: {}", key_value.to_hex());
}
