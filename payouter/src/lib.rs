extern crate actix;
extern crate actix_web;
extern crate byteorder;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate tokio;

extern crate core;
extern crate hd_keyring;
extern crate rpc_client;
extern crate types;

mod errors;
mod monitor;
mod payouter;

pub mod service;
pub use errors::Error;
