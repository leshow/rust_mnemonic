extern crate getopts;
extern crate core;
extern crate mnemonic;
extern crate "rustc-serialize" as serialize;

use mnemonic::Mnemonic;

use serialize::hex::{FromHex, ToHex};
use getopts::{reqopt,optflag,getopts,OptGroup};

use std::os;
use std::iter::repeat;
use std::rand::{OsRng, Rng};
use std::io::File;

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
    //println!("sha256: {}", gen_sha256(str_seed));

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
    //generate corner cases
    for &i in [16us,24,32].iter() {
        for &n in ["00","7f","80","ff"].iter() {
            let corner_chars = repeat(n).take(i).collect();
            process(corner_chars,str_seed,words.as_slice());
        }
    }
    //generate random seeds
    for gen_seed in range(0us,12) {
            let length = 8 * (gen_seed % 3 + 2);
            let random_chars:String = rng.gen_ascii_chars().take(length).collect();
            process(random_chars,str_seed,words.as_slice());
    }
}

fn process(random_chars:String,str_seed:&str,words:&str) {
    println!("{}",random_chars);
    let mnemonic:Mnemonic = Mnemonic::new(random_chars);
    let mut mnem_words = Vec::new();

    for i in range(0us,mnemonic.binary_hash.len() / 11) {
        let bin_idx = mnemonic.binary_hash.slice(i*11,(i+1)*11);
        let idx = std::num::from_str_radix::<isize>(bin_idx, 2).unwrap();
        mnem_words.push(words.as_slice().words().nth(idx as usize).unwrap()); //check for better way of doing this
    }
    let str_mnemonic = format!("{:?}",mnem_words);
    println!("mnemonic: {}", str_mnemonic);
    let key_value = mnemonic.to_seed(str_mnemonic.as_slice(),str_seed); //to_string() on a Vec<&str>?
    println!("key: {}",key_value.as_slice().to_hex());
}
