use std::fmt;

use crypto::pbkdf2::pbkdf2;
use crypto::sha2::{Sha256, Sha512};
use crypto::hmac::Hmac;
use crypto::digest::Digest;

use rustc_serialize::hex::FromHex;
use rustc_serialize::json;

use nom::IResult;

static PBKDF2_ROUNDS: u32 = 2048;
static PBKDF2_KEY_LEN: usize = 64;

#[derive(RustcEncodable)]
struct MnemonicResponse {
    passphrase: String,
}

pub struct Mnemonic {
    pub mnemonic: Vec<u8>,
}

impl Mnemonic {
    pub fn new(chars: &str) -> Mnemonic {
        let h = Mnemonic::gen_sha256(chars).from_hex().unwrap();
        let length = chars.len() / 32;

        Mnemonic { mnemonic: [chars.as_ref(), &h[..length]].concat() }
    }

    pub fn to_seed(&self, mnemonic: &str, seed_value: &str) -> Vec<u8> {
        let mut mac = Hmac::new(Sha512::new(), mnemonic.as_bytes());

        let mut result = vec![0u8; PBKDF2_KEY_LEN];
        let salt = format!("mnemonic{}", seed_value);

        pbkdf2(&mut mac, salt.as_bytes(), PBKDF2_ROUNDS, &mut result);

        result
    }

    pub fn to_words<'a>(&'a self, wordslist: &'a [String]) -> Vec<&str> {
        // Some explanation is necessary.. This uses nom's combinator macros to create a function
        // that makes a parser specifically for grabbing bits 11 at a time, dumping in a u16
        named!(bit_vec<Vec<u16> >, bits!(many0!(take_bits!(u16, 11))));

        let mut mnem_words = Vec::new();
        if let IResult::Done(_, bit_sequence) = bit_vec(self.mnemonic.as_slice()) {
            for idx in &bit_sequence {
                mnem_words.push(wordslist[*idx as usize].as_ref());
            }
        }

        mnem_words
    }

    pub fn to_json(&self, wordslist: &[String]) -> String {
        let words = self.to_words(wordslist).join(" ");
        json::encode(&MnemonicResponse { passphrase: words }).unwrap()
    }

    fn gen_sha256(hashme: &str) -> String {
        let mut sh = Sha256::new();
        sh.input_str(hashme);

        sh.result_str()
    }
}

impl fmt::Debug for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Generated: \n random characters: {:?} \n mnemonic: ", String::from_utf8_lossy(&self.mnemonic[..self.mnemonic.len()-1]))
    }
}
