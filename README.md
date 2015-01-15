#################
# rust mnemonic #
#################

Evan Cameron <cameron.evan@gmail.com>

Cryptographic functions by the rust-crypto library: ( https://github.com/DaGenix/rust-crypto )


mnemonic generation and key derivation follows BIP-0039
( https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki ).

I build rust-mnemonic against the rust nightlies. You can get those here
( http://www.rust-lang.org/install.html ).

**UPDATE**

pbkdf2 key derivation and mnemonic generation are both working.


to get started, simply:

git clone https://github.com/leshow/rust_mnemonic

cd rust_mnemonic

cargo update

cargo build

run with:
./target/rust-mnemonic -s SEED_STRING

The seed string will be used in conjunction with the mnemonic for generating the pbkdf2 key.

The mnemonic is created exactly the same as in python-mnemonic,
generating a random string, taking it's hash, and splicing
the two together in binary then converting bits to positions in the wordlist.
