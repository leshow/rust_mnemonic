[![Build Status](https://travis-ci.org/leshow/rust_mnemonic.svg?branch=master)](https://travis-ci.org/leshow/rust_mnemonic)

# rust mnemonic (BIP-39)

Note: compiles w/ rust edition 2018 currently (testing it out)

Evan Cameron <mailto:cameron.evan@gmail.com>

Crypto leveraged from ringlibrary: ( https://github.com/briansmith/ring )

mnemonic generation and key derivation follows [BIP-0039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki).

to get started, simply:

```bash
git clone https://github.com/leshow/rust_mnemonic

cd rust_mnemonic

cargo build --release
cargo install
```

Install will provide you with the `rust_mnemonic` binary

run with:

```bash
./target/main

# help
./target/main --help
```

If no seed is provided the seed string will be "seed". The seed string will be used in conjunction with the mnemonic for generating the pbkdf2 key.

The mnemonic is created exactly the same (to the best of my knowledge) as in python-mnemonic (https://github.com/trezor/python-mnemonic) and to the BIP0039 standard, my code has not been vetted by the bitcoin community or any cryptographers however, so use with your own caution.
