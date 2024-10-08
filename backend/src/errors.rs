use std::io::Cursor;

use rocket::http::Status;
use rocket::response::Responder;
use rocket::{Request, Response};
use thiserror::Error;

#[allow(dead_code)]
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
    #[error("You are being rate limited, please try again later!")]
    RateLimited,
    #[error("Attempted to get a non-none value but found none")]
    OptionError,
    #[error("Bad Request")]
    BadRequest,
    #[error("Collision issue in passed value")]
    CollisionError,
    #[error("Attempted to parse a number to an integer but errored out: {0}")]
    ParseIntError(#[from] std::num::TryFromIntError),
    #[error("Attempted to parse a string to an integer but errored out: {0}")]
    ParseStringAsIntError(#[from] std::num::ParseIntError),
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
            Self::RateLimited => (Status::TooManyRequests, "Rate-limited!".to_string()),
            Self::OptionError => (
                Status::InternalServerError,
                "Attempted to get a non-none value but found none".to_string(),
            ),
            Self::BadRequest => (Status::BadRequest, "Bad Request!".to_string()),
            Self::CollisionError => (Status::InternalServerError, "Collision!".to_string()),
            Self::ParseIntError(e) => (Status::InternalServerError, e.to_string()),
            Self::ParseStringAsIntError(e) => (Status::InternalServerError, e.to_string()),
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