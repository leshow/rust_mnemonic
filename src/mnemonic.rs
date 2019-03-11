pub use self::MnemonicError::*;
use {
    nom::IResult,
    ring::{
        digest::{self, Digest},
        pbkdf2,
    },
    serde_derive::Serialize,
    serde_json,
    std::{error::Error, fmt, io::Error as ioErr, num::NonZeroU32},
};

pub const LENGTH: usize = 32;
static PBKDF2_KEY_LEN: usize = 64;

#[derive(Serialize)]
struct MnemonicResponse<'a> {
    passphrase: &'a str,
}

pub struct Mnemonic {
    pub mnemonic: Vec<u8>,
    pub pbkdf2_rounds: NonZeroU32,
}

static DIGEST_ALG: &digest::Algorithm = &digest::SHA512;

impl Mnemonic {
    pub fn new<S: AsRef<str>>(chars: S) -> Mnemonic {
        let h = Mnemonic::gen_sha256(chars.as_ref());
        let length = chars.as_ref().len() / 32;
        let pbkdf2_rounds = NonZeroU32::new(2045).unwrap();

        Mnemonic {
            mnemonic: [chars.as_ref().as_bytes(), &h.as_ref()[..length]].concat(),
            pbkdf2_rounds,
        }
    }

    pub fn to_seed<S, M>(&self, mnemonic: M, seed_value: S) -> Vec<u8>
    where
        S: AsRef<str>,
        M: AsRef<str>,
    {
        let salt = self.salt(seed_value);
        let mut result = vec![0u8; PBKDF2_KEY_LEN];
        pbkdf2::derive(
            DIGEST_ALG,
            self.pbkdf2_rounds,
            &salt,
            mnemonic.as_ref().as_bytes(),
            &mut result,
        );

        result
    }

    fn salt<S: AsRef<str>>(&self, username: S) -> Vec<u8> {
        let m = format!("mnemonic{}", username.as_ref());
        let mut salt = Vec::with_capacity(m.len());
        salt.extend(m.as_bytes());
        salt
    }

    // Some explanation is necessary.. This uses nom's combinator macros to create
    // a function that makes a parser specifically for grabbing bits 11 at
    // a time, dumping in a u16
    pub fn to_words<'a>(&self, wordslist: &'a [String]) -> Vec<&'a str> {
        named!(bit_vec<Vec<u16>>, bits!(many0!(take_bits!(u16, 11))));

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
        Ok(serde_json::to_string(&MnemonicResponse {
            passphrase: &words,
        })?)
    }

    fn gen_sha256<S: AsRef<str>>(s: S) -> Digest {
        digest::digest(&digest::SHA256, s.as_ref().as_bytes())
    }
}

impl fmt::Debug for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Generated: \n random characters: {:?} \n mnemonic: ",
            String::from_utf8_lossy(&self.mnemonic[..self.mnemonic.len() - 1])
        )
    }
}

#[derive(Debug)]
pub enum MnemonicError {
    Serde(serde_json::Error),
    Io(ioErr),
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
        }
    }
}

impl Error for MnemonicError {
    fn description(&self) -> &str {
        match *self {
            Io(ref err) => err.description(),
            Serde(ref err) => err.description(),
        }
    }
}
