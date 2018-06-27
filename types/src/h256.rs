use std::convert::Into;
use std::fmt;
use std::io::Write;
use std::ops::Deref;
use std::str::{from_utf8, FromStr};

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::types::VarChar;

use rustc_hex::FromHexError;
use web3::types::H256 as _H256;

#[derive(FromSqlRow, AsExpression, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
#[sql_type = "VarChar"]
pub struct H256(pub _H256);

impl fmt::Debug for H256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:#x}", **self)
    }
}

impl fmt::Display for H256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:#x}", **self)
    }
}

impl ToSql<VarChar, Pg> for H256 {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        ToSql::<VarChar, Pg>::to_sql(&format!("{:?}", self), out)
    }
}

impl FromSql<VarChar, Pg> for H256 {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        match from_utf8(bytes) {
            Ok(h256) => match _H256::from_str(&h256[2..]) {
                Ok(h256) => Ok(H256(h256)),
                Err(e) => Err(e.into()),
            },
            Err(e) => Err(e.into()),
        }
    }
}

impl FromStr for H256 {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match _H256::from_str(&s[2..]) {
            Ok(h256) => Ok(H256(h256)),
            Err(e) => Err(e),
        }
    }
}

impl Deref for H256 {
    type Target = _H256;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<_H256> for H256 {
    fn into(self) -> _H256 {
        self.0
    }
}
