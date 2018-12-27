#![crate_name = "hd_keyring"]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate bit_vec;
extern crate byteorder;
extern crate digest;
extern crate hmac;
extern crate rand;
extern crate regex;
extern crate ring;
extern crate ripemd160;
extern crate secp256k1;
extern crate sha2;
extern crate tiny_keccak;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate rust_base58;

extern crate types;

mod bip32;
mod bip39;
mod errors;
mod keyring;
mod wallet;

pub use bip32::{DerivationPath, Index, XKeyPair, Xprv, Xpub};
pub use errors::Error;
pub use keyring::HdKeyring;
pub use wallet::Wallet;
