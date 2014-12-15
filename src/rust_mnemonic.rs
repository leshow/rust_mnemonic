extern crate "rust-crypto" as rust_crypto;
extern crate getopts;
extern crate serialize;

use getopts::{reqopt,optflag,getopts,OptGroup};

use std::os;
use std::rand::{task_rng, OsRng, Rng};
use std::io::File;
use std::str;
use serialize::hex::{ToHex, FromHex};

use rust_crypto::pbkdf2::pbkdf2;
use rust_crypto::sha2::Sha256;
use rust_crypto::digest::Digest;
use rust_crypto::md5::Md5;
use rust_crypto::mac::Mac;
use rust_crypto::hmac::Hmac;

struct Mnemonic {
    words: Vec<u8>
}

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
    println!("md5: {}", gen_md5(str_seed));
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
    // for word in words.words() {
    //     println!("{}",word)
    // }

    println!("{}",words.words().count());
    //generate corner cases
    for &i in [16u,24,32].iter() {
        for n in ["00","7f","80","ff"].iter() {
            println!("{}",n.repeat(i))
        }
    }

    //generate random seeds
    for gen_seed in range(0u,12) {
        //let num:u32 = rng.gen_range(0,256);
        // for take_num in [16u,24,32].iter() {
        //     let random_char = match std::char::from_u32(num) {
        //         Some(c) => c,
        //         None => panic!("Couldn't convert to char"),
        //     };
        //     println!("{}",random_char);
        // }
        //

        //task_rng().genIter::<u32>().take(16).
        //gen A-Z,a-z,0-9
        //let random_char:String = task_rng().gen_ascii_chars().take(10).collect();
        for &take_num in [16u,24,32].iter() {
            let random_chars:String = task_rng().gen_ascii_chars().take(take_num).collect();
            //println!("{}",random_chars);
            let hex = to_mnemonic(random_chars);
            println!("{}",hex);
        }
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

fn to_mnemonic(chars:String) -> String {
    let hash = gen_sha256(chars.as_slice());

    hash.to_hex()
}
