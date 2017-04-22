pub use self::MnemonicError::*;
use crypto::digest::Digest;
use crypto::hmac::Hmac;

use crypto::pbkdf2::pbkdf2;
use crypto::sha2::{Sha256, Sha512};

use nom::IResult;
use serde_json;

use std::error::Error;
use std::fmt;
use std::io::Error as ioErr;

static PBKDF2_ROUNDS: u32 = 2048;
static PBKDF2_KEY_LEN: usize = 64;

#[derive(Serialize)]
struct MnemonicResponse<'a> {
    passphrase: &'a str,
}

pub struct Mnemonic {
    pub mnemonic: Vec<u8>,
}

impl Mnemonic {
    pub fn new(chars: &str) -> Mnemonic {
        let h = Mnemonic::from_hex(Mnemonic::gen_sha256(chars)).unwrap();
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

    pub fn to_json(&self, wordslist: &[String]) -> Result<String, MnemonicError> {
        let words = self.to_words(wordslist).join(" ");
        Ok(serde_json::to_string(&MnemonicResponse { passphrase: &words })?,)
    }

    fn gen_sha256(hashme: &str) -> String {
        let mut sh = Sha256::new();
        sh.input_str(hashme);

        sh.result_str()
    }

    fn from_hex(from: String) -> Result<Vec<u8>, MnemonicError> {
        // This may be an overestimate if there is any whitespace
        let mut b = Vec::with_capacity(from.len() / 2);
        let mut modulus = 0;
        let mut buf = 0;

        for (idx, byte) in from.bytes().enumerate() {
            buf <<= 4;

            match byte {
                b'A'...b'F' => buf |= byte - b'A' + 10,
                b'a'...b'f' => buf |= byte - b'a' + 10,
                b'0'...b'9' => buf |= byte - b'0',
                b' ' | b'\r' | b'\n' | b'\t' => {
                    buf >>= 4;
                    continue;
                }
                _ => return Err(InvalidHexCharacter(from.clone(), idx)),
            }

            modulus += 1;
            if modulus == 2 {
                modulus = 0;
                b.push(buf);
            }
        }

        match modulus {
            0 => Ok(b.into_iter().collect()),
            _ => Err(InvalidHexLength),
        }
    }
}

impl fmt::Debug for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Generated: \n random characters: {:?} \n mnemonic: ", String::from_utf8_lossy(&self.mnemonic[..self.mnemonic.len()-1]))
    }
}

#[derive(Debug)]
pub enum MnemonicError {
    Serde(serde_json::Error),
    Io(ioErr),
    InvalidHexLength,
    InvalidHexCharacter(String, usize),
}

impl From<ioErr> for MnemonicError {
    fn from(err: ioErr) -> MnemonicError {
        MnemonicError::Io(err)
    }
}

impl From<serde_json::Error> for MnemonicError {
    fn from(err: serde_json::Error) -> MnemonicError {
        MnemonicError::Serde(err)
    }
}

impl fmt::Display for MnemonicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Io(ref err) => write!(f, "IO error: {}", err),
            Serde(ref err) => write!(f, "Serde serialize error: {}", err),
            InvalidHexCharacter(ref string, idx) => {
                write!(f, "Invalid character in '{}' at position {}", string, idx)
            }
            InvalidHexLength => write!(f, "Invalid input length"),
        }
    }
}

impl Error for MnemonicError {
    fn description(&self) -> &str {
        match *self {
            Io(ref err) => err.description(),
            Serde(ref err) => err.description(),
            InvalidHexCharacter(_, _) => "invalid character",
            InvalidHexLength => "invalid length",
        }
    }
}