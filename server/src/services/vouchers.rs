use futures::future::{err, Future, IntoFuture};

use core::db::postgres::PgExecutorAddr;
use core::payment::Payment;
use core::voucher::Voucher;
use services::Error;
use types::Status as PaymentStatus;

pub fn create(
    payment: Payment,
    postgres: PgExecutorAddr,
) -> Box<Future<Item = String, Error = Error>> {
    let transaction = payment.transaction(postgres.clone()).from_err();
    let store = payment.store(postgres.clone()).from_err();

    match payment.status {
        PaymentStatus::Paid => Box::new(transaction.join(store).and_then(
            move |(transaction, store)| {
                Voucher::new(payment, transaction)
                    .encode(&store.private_key)
                    .into_future()
                    .from_err()
            },
        )),
        _ => Box::new(err(Error::PaymentNotConfirmed)),
    }
}
