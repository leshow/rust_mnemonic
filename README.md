#################
# rust mnemonic #
#################

Evan Cameron <cameron.evan@gmail.com>

I'll be using the great rust-crypto ( https://github.com/DaGenix/rust-crypto )
for all of the crypto implementations necessary to get this tool to work.


My goal is to have a small cmdline app like python-mnemonic but written in
rust.

I've only just started learning rust so it may be some time before this is usable.

**UPDATE**
Dec 23,2014 - should be usable for the purpose of generating a mnemonic now

simply:

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
