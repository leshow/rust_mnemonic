extern crate "rust-crypto" as rust_crypto;

use rust_crypto::sha2::{Sha512, Sha384, Sha512Trunc256, Sha512Trunc224, Sha256, Sha224};
use rust_crypto::digest::Digest;
use rust_crypto::md5::Md5;


fn main() {
  let mut sh = Md5::new();
  sh.input_str("The quick brown fox jumps over the lazy dog");
  let out_str = sh.result_str();
  println!("{}",out_str);


  let mut sh = Sha512::new();
  sh.input_str("");
  let out_str = sh.result_str();
  println!("{}",out_str);
}
