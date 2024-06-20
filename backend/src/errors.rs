use std::io::Cursor;

use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("SQL error: {0}")]
    SQL(#[from] sqlx::Error),
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("OAuth token error: {0}")]
    TokenError(#[from] rocket_oauth2::Error),
    #[error("You're not authorized!")]
    Unauthorized,
    #[error("Attempted to get a non-none value but found none")]
    OptionError,
    #[error("Attempted to parse a number to an integer but errored out: {0}")]
    ParseIntError(#[from] std::num::TryFromIntError),
    #[error("Encountered an error trying to convert an infallible value: {0}")]
    FromRequestPartsError(#[from] std::convert::Infallible),
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let response = match self {
            Self::SQL(e) => (Status::InternalServerError, e.to_string()),
            Self::Request(e) => (Status::InternalServerError, e.to_string()),
            Self::TokenError(e) => (Status::InternalServerError, e.to_string()),
            Self::Unauthorized => (Status::Unauthorized, "Unauthorized!".to_string()),
            Self::OptionError => (
                Status::InternalServerError,
                "Attempted to get a non-none value but found none".to_string(),
            ),
            Self::ParseIntError(e) => (Status::InternalServerError, e.to_string()),
            Self::FromRequestPartsError(e) => (Status::InternalServerError, e.to_string()),
        };

        let (status, message) = response;
        let response = Response::build()
            .status(status)
            .sized_body(message.len(), Cursor::new(message))
            .finalize();

        Ok(response)
    }
}