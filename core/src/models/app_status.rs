use futures::Future;

use db::{
    app_statuses::{FindById, Insert, Update},
    postgres::PgExecutorAddr,
};
use models::Error;
use schema::app_statuses;
use types::{currency::Crypto, U128};

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "app_statuses"]
pub struct AppStatusPayload {
    pub id: i16,
    pub eth_block_height: Option<Option<U128>>,
    pub btc_block_height: Option<Option<U128>>,
}

#[derive(Identifiable, Queryable, Serialize)]
#[table_name = "app_statuses"]
pub struct AppStatus {
    pub id: i16,
    pub eth_block_height: Option<U128>,
    pub btc_block_height: Option<U128>,
}

impl AppStatus {
    pub fn insert(
        mut payload: AppStatusPayload,
        postgres: &PgExecutorAddr,
    ) -> impl Future<Item = AppStatus, Error = Error> {
        payload.id = 1;

        (*postgres)
            .send(Insert(payload))
            .from_err()
            .and_then(|res| res.map_err(|e| Error::from(e)))
    }

    pub fn find(postgres: &PgExecutorAddr) -> impl Future<Item = AppStatus, Error = Error> {
        (*postgres)
            .send(FindById(1))
            .from_err()
            .and_then(|res| res.map_err(|e| Error::from(e)))
    }

    pub fn update(
        payload: AppStatusPayload,
        postgres: &PgExecutorAddr,
    ) -> impl Future<Item = AppStatus, Error = Error> {
        (*postgres)
            .send(Update(1, payload))
            .from_err()
            .and_then(|res| res.map_err(|e| Error::from(e)))
    }

    pub fn block_height(&self, currency: Crypto) -> Option<U128> {
        match currency {
            Crypto::Btc => self.btc_block_height,
            Crypto::Eth => self.eth_block_height,
        }
    }
}
