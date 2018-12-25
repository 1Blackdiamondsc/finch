use actix::prelude::*;

use super::{monitor::Monitor, payouter::Payouter};
use core::db::postgres;
use rpc_client::ethereum::RpcClient;
use types::ethereum::Network as EthNetwork;

pub fn run(postgres: postgres::PgExecutorAddr, rpc_client: RpcClient, network: EthNetwork) {
    let pg = postgres.clone();
    let payouter = Arbiter::start(move |_| Payouter::new(pg, rpc_client, network));

    Arbiter::start(move |_| Monitor::new(payouter, postgres));
}
