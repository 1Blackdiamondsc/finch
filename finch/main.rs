extern crate actix;
#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate openssl;

extern crate rpc_client;
extern crate toml;

extern crate block_processor;
extern crate config;
extern crate core;
extern crate payouter;
extern crate server;
extern crate types;

use actix::prelude::*;
use clap::App;
use openssl::rsa::Rsa;
use std::{env, fs::File, io::prelude::*, path::Path};

use config::Config;
use core::db::postgres;
use rpc_client::{bitcoin::RpcClient as BtcRpcClient, ethereum::RpcClient as EthRpcClient};
use types::currency::Crypto;

fn main() {
    env::set_var(
        "RUST_LOG",
        "info,error,debug,actix_web=info,tokio_reactor=info",
    );
    env_logger::init();

    System::run(move || {
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        let mut settings = String::new();

        File::open(
            matches
                .value_of("settings")
                .unwrap_or(format!("{}/.finch.toml", env!("HOME")).as_str()),
        )
        .and_then(|mut f| f.read_to_string(&mut settings))
        .unwrap();

        let config: Config = toml::from_str(&settings).unwrap();

        if !Path::new(&config.server.private_key_path).exists()
            || !Path::new(&config.server.private_key_path).exists()
        {
            let rsa = Rsa::generate(2048).expect("failed to generate a key pair");
            let private_key = rsa
                .private_key_to_der()
                .expect("failed to generate private key");
            let public_key = rsa
                .public_key_to_der_pkcs1()
                .expect("failed to generate public key");

            let mut priv_key_file = File::create(&config.server.private_key_path)
                .expect("failed to create public key file");
            priv_key_file
                .write(&private_key)
                .expect("failed to write to public key file");

            let mut pub_key_file = File::create(&config.server.public_key_path)
                .expect("failed to create public key file");
            pub_key_file
                .write(&public_key)
                .expect("failed to write to public key file");
        }

        let currencies = {
            if matches.is_present("currencies") {
                values_t!(matches, "currencies", Crypto).unwrap()
            } else {
                vec![Crypto::Btc, Crypto::Eth]
            }
        };

        let postgres_url = config.postgres.clone();
        let pg_pool = postgres::init_pool(&postgres_url);
        let postgres = SyncArbiter::start(4, move || postgres::PgExecutor(pg_pool.clone()));

        let skip_missed_blocks = matches.is_present("skip_missed_blocks");

        for c in currencies {
            match c {
                Crypto::Btc => {
                    use block_processor::bitcoin::service as block_processor;
                    use payouter::bitcoin::service as payouter;

                    let btc_config = config.bitcoin.clone().expect("no bitcoin configuration");

                    let btc_conf = btc_config.clone();
                    let rpc_client = BtcRpcClient::new(
                        &btc_conf.rpc_url,
                        &btc_conf.rpc_user,
                        &btc_conf.rpc_pass,
                    );

                    block_processor::run(
                        postgres.clone(),
                        rpc_client.clone(),
                        btc_config.network,
                        skip_missed_blocks,
                    );
                    payouter::run(postgres.clone(), rpc_client.clone(), btc_config.network);
                }
                Crypto::Eth => {
                    use block_processor::ethereum::service as block_processor;
                    use payouter::ethereum::service as payouter;

                    let eth_config = config.clone().ethereum.expect("no ethereum configuration");

                    let rpc_client = EthRpcClient::new(eth_config.rpc_url);

                    block_processor::run(
                        postgres.clone(),
                        rpc_client.clone(),
                        eth_config.network,
                        skip_missed_blocks,
                    );
                    payouter::run(postgres.clone(), rpc_client.clone(), eth_config.network);
                }
            }
        }

        server::run(postgres, config);
    });
}
