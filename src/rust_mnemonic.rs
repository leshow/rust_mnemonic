extern crate crypto;
extern crate getopts;
extern crate serialize;
extern crate core;

use getopts::{reqopt,optflag,getopts,OptGroup};

use std::os;
use std::rand::{OsRng, Rng};
use std::io::File;
use serialize::hex::{FromHex};

use crypto::pbkdf2::{pbkdf2};
use crypto::sha2::{Sha256, Sha512};
//use crypto::mac::{Mac, MacResult};
use crypto::hmac::Hmac;
use crypto::digest::Digest;

static EMPTY:&'static str = "00000000";
static PBKDF2_ROUNDS:u32 = 2048;
static PBKDF2_KEY_LEN:uint = 64;

//getopts help message
fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("-s\t\tSeed");
    println!("-h --help\tUsage");
}


fn main() {
    /* start handling opts */
    let args: Vec<String> = os::args();

    let program = args[0].clone();

    let opts = &[
        reqopt("s", "seed", "set mnemonic seed", ""),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    let seed = match matches.opt_str("s") {
        Some(x) => x,
        None => panic!("No seed given"),
    };
    /* end opts, seed value below */
    let str_seed:&str = seed.as_slice();

    println!("{}", str_seed);
    println!("sha256: {}", gen_sha256(str_seed));

    let mut rng = match OsRng::new() {
      Ok(g) => g,
      Err(e) => panic!("Failed to obtain OS RNG: {}", e)
    };

    let path = Path::new("src/wordslist/english.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.desc),
        Ok(file) => file,
    };
    let words:String = match file.read_to_string() {
        Err(why) => panic!("couldn't read {}: {}", display, why.desc),
        Ok(string) => string,
    };

    println!("{}",words.words().count());
    //generate corner cases
    for &i in [16u,24,32].iter() {
        for n in ["00","7f","80","ff"].iter() {
            println!("{}",n.repeat(i))
        }
    }

    //generate random seeds
    for gen_seed in range(0u,12) {
        for take_num in range(0u,8 * (gen_seed % 3 + 2)) {
            let random_chars:String = rng.gen_ascii_chars().take(take_num).collect();
            println!("{}",random_chars);
            let random_hash = to_mnemonic(random_chars);
            let mut mnemonic = Vec::new();

            for i in range(0u,random_hash.len() / 11) {
                let bin_idx = random_hash.slice(i*11,(i+1)*11);
                let idx = std::num::from_str_radix::<int>(bin_idx, 2).unwrap();
                mnemonic.push(words.words().nth(idx as uint).unwrap()); //check for better way of doing this
            }
            println!("mnemonic: {}",mnemonic.to_string());
            let key_value = to_seed(mnemonic.to_string().as_slice(),str_seed); //to_string() on a Vec<&str>?
            //println!("key: {}",String::from_utf8(key_value).unwrap());
        }
    }

}
// this converts a String to Vec<u8>
// let mut b = Vec::new();
// b.push_all(h.as_bytes());

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

fn to_mnemonic(chars:String) -> String {
    let h:String = gen_sha256(chars.as_slice());
    println!("{}",h);
    //get binary string of random seed
    let s_two:String = to_binary(chars.as_bytes());
    println!("binary of random chars: {}",s_two);
    //get binary str of sha256 hash
    let h_two:String = to_binary(h.from_hex().unwrap().as_slice());
    //unwrap can get a result from Result<Vec<u8>> to Vec<u8> for example
    let length = s_two.len() / 32;
    println!("{}",length);
    println!("sliced bin of hash: {}",h_two.slice_to( length ));
    let random_hash:String =  s_two + h_two.slice_to( length ).as_slice();
    println!("concatenated: {}",random_hash);

    random_hash
}

fn to_seed(mnemonic:&str, seed_value:&str) -> Vec<u8> {
    let mut mac = Hmac::new(Sha512::new(),mnemonic.as_bytes());
    let mut result = Vec::from_elem(PBKDF2_KEY_LEN,0u8);
    let mut salt:String = String::from_str("mnemonic");
    salt.push_str(seed_value);
    pbkdf2(&mut mac, salt.as_bytes(), PBKDF2_ROUNDS, result.as_mut_slice());

    result
}
