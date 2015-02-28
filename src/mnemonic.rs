use crypto::pbkdf2::{pbkdf2};
use crypto::sha2::{Sha256, Sha512};
use crypto::hmac::Hmac;
use crypto::digest::Digest;
use rustc_serialize::hex::{FromHex};

use std::iter::repeat;

static EMPTY: &'static str = "00000000";
static PBKDF2_ROUNDS: u32 = 2048;
static PBKDF2_KEY_LEN: usize = 64;

pub struct Mnemonic {
    pub binary_hash: String,
}

impl Mnemonic {
    pub fn new(chars:String) -> Mnemonic {
        let h: String = Mnemonic::gen_sha256(&chars[]);

        //get binary string of random seed
        let s_two: String = Mnemonic::to_binary(chars.as_bytes());

        //get binary str of sha256 hash
        let h_two: String = Mnemonic::to_binary(&h.from_hex().unwrap()[]);
        let length = s_two.len() / 32;

        //concatenate the two binary strings together
        let random_hash: String =  s_two + &h_two[.. length][]; //h_two.slice_to( length ).as_slice()

        Mnemonic { binary_hash: random_hash }
    }

    pub fn to_seed(&self, mnemonic: &str, seed_value: &str) -> Vec<u8> {
        let mut mac = Hmac::new(Sha512::new(), mnemonic.as_bytes());

        let mut result: Vec<u8> = vec![0u8; PBKDF2_KEY_LEN];
        let salt = format!("mnemonic{}", seed_value);

        pbkdf2(&mut mac, salt.as_bytes(), PBKDF2_ROUNDS, &mut result);

        result
    }

    fn gen_sha256(hashme: &str) -> String {
        let mut sh = Sha256::new();
        sh.input_str(hashme);

        sh.result_str()
    }

    fn to_binary(input: &[u8]) -> String {
        let mut s_two = String::new();

        for &s_byte in input.iter() {
            let byte_slice = format!("{:b}",s_byte);
            let mut empty = String::from_str(EMPTY);

            empty.push_str(&byte_slice[]);

            let slice = &empty[empty.len()-8 ..];

            s_two.push_str(slice);
        }

        s_two
    }
}
