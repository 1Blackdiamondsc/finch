use futures::future::{err, Future, IntoFuture};

use core::db::postgres::PgExecutorAddr;
use core::payment::Payment;
use core::voucher::Voucher;
use services::Error;
use types::PayoutStatus;

pub fn create(
    payment: Payment,
    postgres: &PgExecutorAddr,
) -> Box<Future<Item = String, Error = Error>> {
    let postgres = postgres.clone();

    let transaction = payment.transaction(&postgres).from_err();
    let store = payment.store(&postgres).from_err();

    // TODO: Check PaymentStatus::Paid and confirmation status.
    match payment.payout_status {
        PayoutStatus::PaidOut => Box::new(transaction.join(store).and_then(
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
