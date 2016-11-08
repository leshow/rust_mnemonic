[![Build Status](https://travis-ci.org/leshow/rust_mnemonic.svg?branch=master)](https://travis-ci.org/leshow/rust_mnemonic)


#################
# rust mnemonic #
#################

Evan Cameron <cameron.evan@gmail.com>

Crypto leveraged from the rust-crypto library: ( https://github.com/DaGenix/rust-crypto )

mnemonic generation and key derivation follows [BIP-0039]
(https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki).

to get started, simply:
```bash
git clone https://github.com/leshow/rust_mnemonic

cd rust_mnemonic

cargo build
```
run with:
```bash
./target/main

# help
./target/main --help
```

If no seed is provided the seed string will be "seed". The seed string will be used in conjunction with the mnemonic for generating the pbkdf2 key.

The mnemonic is created exactly the same (to the best of my knowledge) as in python-mnemonic (https://github.com/trezor/python-mnemonic) and to the BIP0039 standard, my code has not been vetted
by the bitcoin community or any cryptographers however, so use with your own caution.
