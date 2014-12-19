extern crate crypto;
extern crate getopts;
extern crate serialize;
extern crate core;

use getopts::{reqopt,optflag,getopts,OptGroup};

use std::os;
use std::rand::{task_rng, OsRng, Rng};
use std::io::File;
use std::str;
use serialize::hex::{ToHex, FromHex};

use core::fmt::{Binary};

use crypto::pbkdf2::pbkdf2;
use crypto::sha2::Sha256;
use crypto::digest::Digest;

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
        for take_num in range(0u,8 * (gen_seed % 3 + 2)) {
            //let random_chars: Vec<u8> = task_rng().gen_iter::<u8>().take(take_num).collect(); //http://rustbyexample.com/staging/rand.html
            let random_chars:String = rng.gen_ascii_chars().take(take_num).collect();
            //let random_chars:Vec<char> = task_rng().gen_iter::<char>().take(take_num).collect(); //generates any valid character
            println!("{}",random_chars);
            to_mnemonic(random_chars);
            // let thing = match str::from_utf8(random_chars.as_slice()) {
            //     None => panic!("can't convert"),
            //     Some(x) => x,
            // };
            //println!("{}",thing);
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

fn to_mnemonic(chars:String) {
    let h:String = gen_sha256(chars.as_slice());
    println!("{}",h);
    //get binary string of random seed
    let mut s_two = String::new();
    for &s_byte in chars.as_bytes().iter() {
        //need to pad binary number with correct amount of zeroes
        for char_byte in format!("{:b}",s_byte).chars() {
            s_two.push(char_byte)
        }
    }
    println!("binary of random chars: {}",s_two);
    //get binary str of sha256 hash
    let mut h_two = String::new();
    //let mut vec_two = Vec::new();
    //unwrap can get a result from Result<Vec<u8>> to Vec<u8> for example
    for &h_byte in h.from_hex().unwrap().iter() {
        for char_byte_h in format!("{:b}",h_byte).chars() {
            //let fill_amount = 8 - char_byte_h.len(); need to pad binary number with correct amount of zeroes

            h_two.push(char_byte_h)
        }
        //vec_two.push(format!("0{:b}",h_byte)) //if h_two is a Vec, then this works -- vector of strings with bits
    }
    let length = s_two.len() / 32;
    println!("{}",length);
    //println!("{}",vec_two);
    println!("binary of hash: {}",h_two);//h_two.slice_to( length )
}
