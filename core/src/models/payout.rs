use std::convert::From;

use chrono::prelude::*;
use futures::Future;
use uuid::Uuid;

use db::payouts::{FindAllConfirmed, Insert, UpdateById};
use db::postgres::PgExecutorAddr;
use models::payment::Payment;
use models::store::Store;
use models::Error;
use schema::payouts;
use types::{Currency, H256, PayoutAction, PayoutStatus, U128};

#[derive(Debug, Insertable, AsChangeset, Serialize)]
#[table_name = "payouts"]
pub struct PayoutPayload {
    pub status: Option<PayoutStatus>,
    pub action: Option<PayoutAction>,
    pub store_id: Option<Uuid>,
    pub payment_id: Option<Uuid>,
    pub typ: Option<Currency>,
    pub eth_block_height_required: Option<U128>,
    pub transaction_hash: Option<H256>,
    pub created_at: Option<DateTime<Utc>>,
}
impl PayoutPayload {
    pub fn set_created_at(&mut self) {
        self.created_at = Some(Utc::now());
    }
}

impl From<Payout> for PayoutPayload {
    fn from(payout: Payout) -> Self {
        PayoutPayload {
            status: Some(payout.status),
            action: Some(payout.action),
            store_id: Some(payout.store_id),
            payment_id: Some(payout.payment_id),
            typ: Some(payout.typ),
            eth_block_height_required: Some(payout.eth_block_height_required),
            transaction_hash: payout.transaction_hash,
            created_at: Some(payout.created_at),
        }
    }
}

#[derive(Debug, Identifiable, Queryable, Associations, Clone, Serialize, Deserialize)]
#[belongs_to(Store, foreign_key = "store_id")]
#[belongs_to(Payment, foreign_key = "payment_id")]
pub struct Payout {
    pub id: Uuid,
    pub status: PayoutStatus,
    pub action: PayoutAction,
    pub store_id: Uuid,
    pub payment_id: Uuid,
    pub typ: Currency,
    pub eth_block_height_required: U128,
    pub transaction_hash: Option<H256>,
    pub created_at: DateTime<Utc>,
}

impl Payout {
    pub fn store(&self, postgres: &PgExecutorAddr) -> impl Future<Item = Store, Error = Error> {
        Store::find_by_id(self.store_id, postgres)
    }

    pub fn payment(&self, postgres: &PgExecutorAddr) -> impl Future<Item = Payment, Error = Error> {
        Payment::find_by_id(self.payment_id, postgres)
    }

    pub fn insert(
        mut payload: PayoutPayload,
        postgres: &PgExecutorAddr,
    ) -> impl Future<Item = Payout, Error = Error> {
        payload.set_created_at();

        (*postgres)
            .send(Insert(payload))
            .from_err()
            .and_then(|res| res.map_err(|e| Error::from(e)))
    }

    pub fn find_all_confirmed(
        block_height: U128,
        postgres: &PgExecutorAddr,
    ) -> impl Future<Item = Vec<Payout>, Error = Error> {
        (*postgres)
            .send(FindAllConfirmed(block_height))
            .from_err()
            .and_then(|res| res.map_err(|e| Error::from(e)))
    }

    pub fn update_by_id(
        id: Uuid,
        payload: PayoutPayload,
        postgres: &PgExecutorAddr,
    ) -> impl Future<Item = Payout, Error = Error> {
        (*postgres)
            .send(UpdateById(id, payload))
            .from_err()
            .and_then(|res| res.map_err(|e| Error::from(e)))
    }
}
