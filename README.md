#################
# rust mnemonic #
#################

Evan Cameron <cameron.evan@gmail.com>

Cryptographic functions by the rust-crypto library: ( https://github.com/DaGenix/rust-crypto )


My goal is to have a small cmdline app like python-mnemonic but written in
rust, that follows BIP-0039 ( https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki ).

I've only just started learning rust so go easy on me. I build rust-mnemonic
against the rust nightlies. You can get those here ( http://www.rust-lang.org/install.html ).

**UPDATE**

Dec 23,2014 - should be usable for the purpose of generating a mnemonic now,
pbkdf2 keys still a WIP


to get started, simply:

git clone https://github.com/leshow/rust_mnemonic

cd rust_mnemonic

cargo update

cargo build

run with:
./target/rust-mnemonic -s SEED_STRING

The seed string will be used for generating the pbkdf2 key.

The mnemonic is created exactly the same as in python-mnemonic,
generating a random string, taking it's hash, and splicing
the two together in binary then converting bits to positions in the wordlist.
