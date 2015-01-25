extern crate crypto;
extern crate "rustc-serialize" as rustc_serialize;

use crypto::pbkdf2::{pbkdf2};
use crypto::sha2::{Sha256, Sha512};
use crypto::hmac::Hmac;
use crypto::digest::Digest;
use std::io::File;
use rustc_serialize::hex::{FromHex, ToHex};

use std::iter::repeat;

static EMPTY:&'static str = "00000000";
static PBKDF2_ROUNDS:u32 = 2048;
static PBKDF2_KEY_LEN:usize = 64;

pub struct Mnemonic {
    pub binary_hash:String,
}
impl Mnemonic {
    pub fn new(chars:String) -> Mnemonic {
        let h:String = Mnemonic::gen_sha256(chars.as_slice());
        //get binary string of random seed
        let s_two:String = Mnemonic::to_binary(chars.as_bytes());
        //get binary str of sha256 hash
        let h_two:String = Mnemonic::to_binary(h.from_hex().unwrap().as_slice());
        let length = s_two.len() / 32;
        //concatenate the two binary strings together
        let random_hash:String =  s_two + h_two.slice_to( length ).as_slice();
        let mn = Mnemonic {
            binary_hash: random_hash,
        };

        mn
    }

    pub fn to_seed(&self,mnemonic:&str, seed_value:&str) -> Vec<u8> {
        let mut mac = Hmac::new(Sha512::new(),mnemonic.as_bytes());
        let mut result:Vec<u8> = repeat(0).take(PBKDF2_KEY_LEN).collect();
        let mut salt:String = String::from_str("mnemonic");
        salt.push_str(seed_value);
        pbkdf2(&mut mac, salt.as_bytes(), PBKDF2_ROUNDS, result.as_mut_slice());

        result
    }
    fn gen_sha256(hashme:&str) -> String {
        let mut sh = Sha256::new();
        sh.input_str(hashme);

        sh.result_str()
    }

    fn to_binary(input:&[u8]) -> String {
        let mut s_two = String::new();
        for &s_byte in input.iter() {
            let byte_slice = format!("{:b}",s_byte);
            let mut empty = String::from_str(EMPTY);
            empty.push_str(byte_slice.as_slice());
            let slice = empty.slice_from(empty.len()-8);
            s_two.push_str(slice);
        }

        s_two
    }
}
