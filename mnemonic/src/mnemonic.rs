use crypto::pbkdf2::{pbkdf2};
use crypto::sha2::{Sha256, Sha512};
use crypto::hmac::Hmac;
use crypto::digest::Digest;

use serialize::hex::{FromHex, ToHex};

use std::iter::repeat;

static EMPTY:&'static str = "00000000";
static PBKDF2_ROUNDS:u32 = 2048;
static PBKDF2_KEY_LEN:usize = 64;

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

pub fn to_mnemonic(chars:String) -> String {
    let h:String = gen_sha256(chars.as_slice());
    //println!("{}",h);
    //get binary string of random seed
    let s_two:String = to_binary(chars.as_bytes());
    //println!("binary of random chars: {}",s_two);
    //get binary str of sha256 hash
    let h_two:String = to_binary(h.from_hex().unwrap().as_slice());
    //unwrap can get a result from Result<Vec<u8>> to Vec<u8> for example
    let length = s_two.len() / 32;
    //println!("sliced bin of hash: {}",h_two.slice_to( length ));
    let random_hash:String =  s_two + h_two.slice_to( length ).as_slice();
    //println!("concatenated: {}",random_hash);

    random_hash
}

pub fn to_seed(mnemonic:&str, seed_value:&str) -> Vec<u8> {
    let mut mac = Hmac::new(Sha512::new(),mnemonic.as_bytes());
    let mut result:Vec<u8> = repeat(0).take(PBKDF2_KEY_LEN).collect();
    let mut salt:String = String::from_str("mnemonic");
    salt.push_str(seed_value);
    pbkdf2(&mut mac, salt.as_bytes(), PBKDF2_ROUNDS, result.as_mut_slice());

    result
}