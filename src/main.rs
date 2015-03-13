#![feature(core)]
#![feature(collections)]

extern crate getopts;
extern crate lib;
extern crate rand;
extern crate "rustc-serialize" as serialize;

use lib::mnemonic::Mnemonic;
use lib::settings::RuntimeSettings;

use serialize::hex::ToHex;
use std::iter::repeat;
use rand::{OsRng, Rng};
use std::fs::File;
use std::io::Read;
use std::env;


fn main() {
    /* start handling opts */
    let settings = RuntimeSettings::new(env::args());
    if settings.print_help {
        settings.print_usage();
        return;
    }

    let str_seed: &str = &settings.seed.unwrap_or(String::from_str("seed"));

    // let str_seed: &str = match settings.seed {
    //     Some(x) => { println!("Seed set to: \"{}\"", x);
    //                  &x },
    //     None => "seed",
    // };

    let mut rng = match OsRng::new() {
        Ok(g) => g,
        Err(e) => panic!("Failed to obtain OS RNG: {}", e)
    };

    let path = Path::new("src/wordslist/english.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let word_backing: String = match file.read_to_string() {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(string) => string,
    };
    let words: Vec<_> = word_backing.words().collect();

    //generate corner cases
    for &i in [16usize, 24, 32].iter() {
        for &n in ["00", "7f", "80", "ff"].iter() {
            let corner_chars = repeat(n).take(i).collect();
            process(corner_chars, str_seed, &words);
        }
    }

    //generate random seeds
    for gen_seed in range(0usize, 12) {
        let length = 8 * (gen_seed % 3 + 2);
        let random_chars:String = rng.gen_ascii_chars().take(length).collect();

        process(random_chars, str_seed, &words);
    }
}

fn process(random_chars: String, str_seed: &str, words: &[&str]) {
    println!("random characters: {}",random_chars);

    let mnemonic: Mnemonic = Mnemonic::new(random_chars);
    let mut mnem_words = Vec::new();

    for i in range(0usize, mnemonic.binary_hash.len() / 11) {
        let bin_idx = &mnemonic.binary_hash[i * 11 .. (i + 1) * 11];
        let idx = std::num::from_str_radix::<isize>(bin_idx, 2).unwrap();

        mnem_words.push(words[idx as usize]);
    }

    let str_mnemonic = format!("{:?}",mnem_words);
    println!("mnemonic: {}", str_mnemonic);

    let key_value = mnemonic.to_seed(&str_mnemonic, str_seed); //to_string() on a Vec<&str>?
    println!("key: {}", key_value.to_hex());
}
