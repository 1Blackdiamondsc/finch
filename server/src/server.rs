use std::fs;

use actix::prelude::*;
use actix_web::{middleware, server, App};
use num_cpus;

use db::{postgres, redis};
use types::{PrivateKey, PublicKey};

#[derive(Clone)]
pub struct ServerState {
    pub postgres: postgres::PgExecutorAddr,
    pub redis: redis::RedisExecutorAddr,
    pub jwt_private: PrivateKey,
    pub jwt_public: PublicKey,
}

pub fn run(
    host: String,
    port: String,
    private_key_path: String,
    public_key_path: String,
    postgres_url: String,
    redis_url: String,
    ethereum_url: String,
) {
    System::run(move || {
        let jwt_private = fs::read(private_key_path).expect("Failed to open the private key file.");
        let jwt_public = fs::read(public_key_path).expect("Failed to open the public key file.");

        let pg_pool = postgres::init_pool(&postgres_url);
        let pg_addr = SyncArbiter::start(num_cpus::get() * 4, move || {
            postgres::PgExecutor(pg_pool.clone())
        });

        let redis_addr = SyncArbiter::start(num_cpus::get() * 1, move || {
            redis::RedisExecutor(redis::init_pool(&redis_url))
        });

        server::new(move || {
            App::with_state(ServerState {
                postgres: pg_addr.clone(),
                redis: redis_addr.clone(),
                jwt_private: jwt_private.clone(),
                jwt_public: jwt_public.clone(),
            }).middleware(middleware::Logger::default())
        }).bind(format!("{}:{}", host, port))
            .expect(&format!("Can not bind {}:{}", host, port))
            .start();
    });
}
