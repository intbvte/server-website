#[macro_use]
extern crate rocket;

use std::hash::RandomState;
use std::sync::{Arc, Mutex};
use chrono::{Duration, Local};
use dotenvy::dotenv;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::{Redirect, Responder};
use rocket::serde::json::Json;
use rocket::{Request, State};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::Deserialize;
use sqlx::query;
use thiserror::Error;
use rand::{random, RngCore};
use reqwest::StatusCode;

use crate::app::App;

mod minecraft;
mod app;

struct Discord;

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

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let response = match self {
            Self::SQL(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::Request(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::TokenError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized!".to_string()),
            Self::OptionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Attempted to get a non-none value but found none".to_string(),
            ),
            Self::ParseIntError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::FromRequestPartsError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        response
    }
}

#[derive(Deserialize)]
struct Whitelist {
    username: String,
}

#[derive(Deserialize)]
struct DiscordCallback {
    id: String,
    username: String,
}

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: String,
    minecraft_username: String,
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let app: App = App::new().await;

    sqlx::migrate!().run(&app.db).await.expect("Failed to apply migrations :(");

    rocket::build()
        .manage(app)
        .mount("/backend/", routes![discord_login, discord_callback, minecraft_whitelist, minecraft_whitelist_remove])
        .attach(OAuth2::<Discord>::fairing("discord"))
}

#[get("/login/discord")]
fn discord_login(oauth2: OAuth2<Discord>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["identify"]).unwrap()
}

#[get("/auth/discord")]
async fn discord_callback(app: &State<App>, token: TokenResponse<Discord>, cookies: &CookieJar<'_>) -> Result<Redirect, ApiError> {
    let Some(secs) = token.expires_in() else {
        return Err(ApiError::OptionError);
    };

    let secs: i64 = secs.try_into().unwrap();

    let max_age = Local::now().naive_local() + Duration::seconds(secs);

    let mut u128_pool = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut u128_pool);

    let session_id = u128::from_le_bytes(u128_pool);

    let cookie = Cookie::build(("session_id", session_id.to_string())).build();

    cookies.add_private(
        Cookie::build(("token", token.access_token().to_string()))
            .same_site(SameSite::Lax)
            .build()
    );

    let user = app.https.get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token.access_token()))
        .send()
        .await
        .unwrap()
        .json::<DiscordCallback>()
        .await
        .unwrap();

    let user_id = user.id.parse::<i64>().expect("Failed to read user.id as a i64");

    query!("INSERT INTO users (id)
            VALUES ($1)
            ON CONFLICT (id) DO NOTHING;", user_id)
        .execute(&app.db)
        .await
        .unwrap();

    //fixme change redirect location
    Ok(Redirect::to("/"))
}

#[post("/minecraft/whitelist/add", format = "application/json", data = "<whitelist_data>")]
async fn minecraft_whitelist(app: &State<App>, whitelist_data: Json<Whitelist>) -> (Status, &'static str) {
    minecraft::run_command(
        app,
        format!("whitelist add {}", whitelist_data.username),
        "User was whitelisted successfully",
        format!("A unknown error occurred while whitelisting user {}", whitelist_data.username),
    ).await
}

#[post("/minecraft/whitelist/remove", format = "application/json", data = "<whitelist_data>")]
async fn minecraft_whitelist_remove(app: &State<App>, whitelist_data: Json<Whitelist>) -> (Status, &'static str) {
    minecraft::run_command(
        app,
        format!("whitelist remove {}", whitelist_data.username),
        "User was un-whitelisted successfully",
        format!("A unknown error occurred while un-whitelisting user {}", whitelist_data.username),
    ).await
}