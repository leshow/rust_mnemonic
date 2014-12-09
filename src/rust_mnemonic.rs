extern crate "rust-crypto" as rust_crypto;

use std::io::IoResult;
use std::num::{Int, ToPrimitive};
use std::rand::{OsRng, Rng};
use std::io::File;


use rust_crypto::pbkdf2::pbkdf2;
use rust_crypto::sha2::Sha256;
use rust_crypto::digest::Digest;
use rust_crypto::md5::Md5;
use rust_crypto::mac::Mac;
use rust_crypto::hmac::Hmac;


pub struct Mnemonic {
    words: Vec<u8>
}


fn main() {
    let fox = "The quick brown fox jumps over the lazy dog";
    println!("md5:  {}",gen_md5(fox));
    println!("sha256:  {}",gen_sha256(fox));
    // let password = "The quick brown fox jumps over the lazy dog";
    // let mac = Hmac::new(Sha256::new(), password.as_bytes());
    // let stuff = String::from_utf8(mac.to_vec());
    // println!("{}",stuff);

    let mut rng = match OsRng::new() {
      Ok(g) => g,
      Err(e) => panic!("Failed to obtain OS RNG: {}", e)
    };

    let num:u32 = rng.next_u32();
    println!("{}",num);

    let path = Path::new("src/wordslist/english.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.desc),
        Ok(file) => file,
    };
    match file.read_to_string() {
        Err(why) => panic!("couldn't read {}: {}", display, why.desc),
        Ok(string) => print!("{} contains: {}", display, string.words().count()),
    }
}

fn gen_md5(hashme:&str) -> String {
    let mut sh = Md5::new();
    sh.input_str(hashme);

    sh.result_str()
}

fn gen_sha256(hashme:&str) -> String {
    let mut sh = Sha256::new();
    sh.input_str(hashme);

    sh.result_str()
}
