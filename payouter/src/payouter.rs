use actix::prelude::*;
use actix_web::actix::spawn;
use futures::future::{ok, Future, IntoFuture};
use serde_json;

use errors::Error;
use ethereum_client::{Client, Transaction};

use core::db::postgres::PgExecutorAddr;
use core::db::redis::RedisSubscriberAddr;
use core::payment::{Payment, PaymentPayload};
use hd_keyring::HdKeyring;
use types::{H256, PaymentStatus, U256};

pub type PayouterAddr = Addr<Payouter>;

pub struct Payouter {
    pub subscriber: RedisSubscriberAddr,
    pub postgres: PgExecutorAddr,
    pub ethereum_rpc_url: String,
    pub chain_id: u64,
}

impl Payouter {
    pub fn new(
        pg_addr: PgExecutorAddr,
        subscriber_addr: RedisSubscriberAddr,
        ethereum_rpc_url: String,
        chain_id: u64,
    ) -> Self {
        Payouter {
            subscriber: subscriber_addr,
            postgres: pg_addr,
            ethereum_rpc_url,
            chain_id,
        }
    }

    pub fn payout(&self, payment: Payment) -> impl Future<Item = H256, Error = Error> {
        let eth_client = Client::new(self.ethereum_rpc_url.clone());
        let chain_id = self.chain_id.clone();

        let store = payment.store(&self.postgres).from_err();
        let transaction = payment.transaction(&self.postgres).from_err();

        let gas_price = eth_client.get_gas_price().from_err();
        let nonce = eth_client
            .get_transaction_count(payment.clone().eth_address.unwrap())
            .from_err();

        store
            .join(transaction)
            .join(gas_price)
            .join(nonce)
            .and_then(move |(((store, transaction), gas_price), nonce)| {
                let mut path = store.hd_path.clone();
                let timestamp_nanos = payment.created_at.timestamp_nanos().to_string();
                let second = &timestamp_nanos[..10];
                let nano_second = &timestamp_nanos[10..];

                path.push_str("/");
                path.push_str(second);
                path.push_str("/");
                path.push_str(nano_second);

                HdKeyring::from_mnemonic(&path, &store.mnemonic.clone(), 0)
                    .into_future()
                    .from_err()
                    .and_then(move |keyring| {
                        keyring
                            .get_wallet_by_index(payment.index as u32)
                            .into_future()
                            .from_err()
                            .and_then(move |wallet| {
                                let value =
                                    U256(transaction.value.0 - gas_price.0 * U256::from(21_000).0);

                                let raw_transaction = Transaction {
                                    nonce,
                                    gas_price,
                                    gas: U256::from(21_000),
                                    to: store.payout_addresses[0].clone(),
                                    value,
                                    data: b"".to_vec(),
                                };

                                raw_transaction
                                    .sign(wallet.secret_key, chain_id)
                                    .into_future()
                                    .from_err()
                                    .and_then(move |signed_transaction| {
                                        eth_client
                                            .send_raw_transaction(signed_transaction)
                                            .from_err()
                                    })
                            })
                    })
            })
    }
}

impl Actor for Payouter {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Payout(pub Payment);

impl Handler<Payout> for Payouter {
    type Result = ();

    fn handle(&mut self, Payout(payment): Payout, _: &mut Self::Context) -> Self::Result {
        let postgres = self.postgres.clone();

        spawn(
            self.payout(payment.clone())
                .map_err(|_| ())
                .and_then(move |hash| {
                    println!("Paid out {:?}", hash);
                    let mut payload = PaymentPayload::from(payment.clone());
                    payload.payout_transaction_hash = Some(hash);
                    payload.status = Some(PaymentStatus::PaidOut);

                    Payment::update_by_id(payment.id, payload, &postgres).map_err(|e| {
                        // TODO: Handle error.
                        ()
                    })
                })
                .map(|_| ()),
        );
    }
}