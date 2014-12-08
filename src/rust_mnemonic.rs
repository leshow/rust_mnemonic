extern crate "rust-crypto" as rust_crypto;

use std::io::IoResult;
use std::num::{Int, ToPrimitive};
use std::rand::{OsRng, Rng};


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
  let mut sh = Md5::new();
  sh.input_str("The quick brown fox jumps over the lazy dog");
  let out_str = sh.result_str();
  //println!("{}",out_str);

  // let mut rng = rand::task_rng();
  // if rng.gen() {
  //     println!("int: {}, uint: {}", rng.gen::<int>(), rng.gen::<uint>())
  // }

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


  let mut sh = Sha256::new();
  sh.input_str("");
  let out_str = sh.result_str();
  //println!("{}",out_str);
}
