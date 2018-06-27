use chrono::prelude::*;
use futures::Future;
use serde_json::Value;
use uuid::Uuid;

use db::postgres::PgExecutorAddr;
use db::stores::{FindById, Insert};
use models::user::User;
use models::Error;
use schema::stores;
use types::{H160, PrivateKey, PublicKey};

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "stores"]
pub struct StorePayload {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub owner_id: Option<Uuid>,
    pub private_key: Option<PrivateKey>,
    pub public_key: Option<PublicKey>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub payout_addresses: Vec<H160>,
    pub active: Option<bool>,
}

impl StorePayload {
    pub fn set_created_at(&mut self) {
        self.created_at = Some(Utc::now());
    }

    pub fn set_updated_at(&mut self) {
        self.updated_at = Some(Utc::now());
    }
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[belongs_to(User, foreign_key = "owner_id")]
pub struct Store {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub owner_id: Uuid,
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub payout_addresses: Vec<H160>,
    pub active: bool,
}

impl Store {
    pub fn insert(
        mut payload: StorePayload,
        postgres: PgExecutorAddr,
    ) -> impl Future<Item = Store, Error = Error> {
        payload.set_created_at();
        payload.set_updated_at();

        postgres
            .send(Insert(payload))
            .from_err()
            .and_then(|res| res.map_err(|e| Error::from(e)))
    }

    pub fn find_by_id(
        id: Uuid,
        postgres: PgExecutorAddr,
    ) -> impl Future<Item = Store, Error = Error> {
        postgres
            .send(FindById(id))
            .from_err()
            .and_then(|res| res.map_err(|e| Error::from(e)))
    }

    pub fn export(&self) -> Value {
        json!({
            "id": self.id,
            "name": self.name,
            "description": self.description,
            "created_at": self.created_at.timestamp(),
            "updated_at": self.updated_at.timestamp(),
        })
    }
}
