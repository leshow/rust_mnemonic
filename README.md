#################
# rust mnemonic #
#################

Evan Cameron <cameron.evan@gmail.com>

Cryptographic leveraged from the rust-crypto library: ( https://github.com/DaGenix/rust-crypto )

mnemonic generation and key derivation follows BIP-0039
( https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki ).

I build rust-mnemonic against the rust nightlies. You can get those here
( http://www.rust-lang.org/install.html ).

to get started, simply:
```
git clone https://github.com/leshow/rust_mnemonic

cd rust_mnemonic

cargo update

cargo build
```
run with:
```
./target/rust-mnemonic -s SEED_STRING
```

If no seed is provided the seed string will be "seed". The seed string will be used in conjunction with the mnemonic for generating the pbkdf2 key.

The mnemonic is created exactly the same (to the best of my knowledge) as in python-mnemonic (https://github.com/trezor/python-mnemonic) and to the BIP0039 standard, my code has not been vetted
by the bitcoin community or any cryptographers however, so use with your won caution. 
