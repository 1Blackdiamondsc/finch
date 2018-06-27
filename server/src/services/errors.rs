use actix::MailboxError;
use actix_web::{error, http, HttpResponse};
use data_encoding::DecodeError;
use db::Error as DbError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use jwt::errors::Error as JwtError;
use openssl::error::ErrorStack;
use rustc_hex::FromHexError;
use secp256k1::Error as Secp256k1Error;

use hd_keyring::Error as KeyringError;
use models::Error as ModelError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    ModelError(#[cause] ModelError),
    #[fail(display = "{}", _0)]
    KeyringError(#[cause] KeyringError),
    #[fail(display = "{}", _0)]
    DecodeError(#[cause] DecodeError),
    #[fail(display = "JWT error: {}", _0)]
    JwtError(String),
    #[fail(display = "{}", _0)]
    ErrorStack(#[cause] ErrorStack),
    #[fail(display = "{}", _0)]
    Secp256k1Error(#[cause] Secp256k1Error),
    #[fail(display = "{}", _0)]
    FromHexError(#[cause] FromHexError),
    #[fail(display = "{}", _0)]
    MailboxError(#[cause] MailboxError),
    #[fail(display = "Incorrect password")]
    IncorrectPassword,
    #[fail(display = "Invalid request account")]
    InvalidRequestAccount,
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::IncorrectPassword => HttpResponse::new(http::StatusCode::BAD_REQUEST),

            Error::InvalidRequestAccount => HttpResponse::new(http::StatusCode::FORBIDDEN),

            Error::ModelError(ref e) => match *e {
                ModelError::DbError(ref e) => match *e {
                    DbError::DieselError(ref e) => match *e {
                        DieselError::DatabaseError(ref kind, _) => match kind {
                            DatabaseErrorKind::UniqueViolation => {
                                HttpResponse::new(http::StatusCode::CONFLICT)
                            }
                            _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
                        },
                        DieselError::NotFound => HttpResponse::new(http::StatusCode::NOT_FOUND),
                        _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
                    },
                    _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
                },
                _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
            },

            _ => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<ModelError> for Error {
    fn from(e: ModelError) -> Error {
        Error::ModelError(e)
    }
}

impl From<KeyringError> for Error {
    fn from(e: KeyringError) -> Error {
        Error::KeyringError(e)
    }
}

impl From<DecodeError> for Error {
    fn from(e: DecodeError) -> Error {
        Error::DecodeError(e)
    }
}

impl From<ErrorStack> for Error {
    fn from(e: ErrorStack) -> Error {
        Error::ErrorStack(e)
    }
}

impl From<JwtError> for Error {
    fn from(e: JwtError) -> Error {
        Error::JwtError(e.kind().description().to_owned())
    }
}

impl From<Secp256k1Error> for Error {
    fn from(e: Secp256k1Error) -> Error {
        Error::Secp256k1Error(e)
    }
}

impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Error {
        Error::FromHexError(e)
    }
}

impl From<MailboxError> for Error {
    fn from(e: MailboxError) -> Error {
        Error::MailboxError(e)
    }
}
