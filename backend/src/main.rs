#[macro_use]
extern crate rocket;

use std::hash::RandomState;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use chrono::{Duration, Local};
use dotenvy::dotenv;
use rand::{random, RngCore};
use reqwest::StatusCode;
use rocket::{Request, Response, State, time};
use rocket::http::{ContentType, Cookie, CookieJar, SameSite, Status};
use rocket::response::{Redirect, Responder};
use rocket::serde::json::Json;
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::Deserialize;
use sqlx::query;
use thiserror::Error;

use crate::app::App;
use crate::errors::ApiError;

mod minecraft;
mod app;
mod errors;

struct Discord;

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

    let session_id = u128::from_le_bytes(u128_pool).to_string();

    cookies.add_private(
        Cookie::build(("session_id", session_id.clone()))
            .same_site(SameSite::Lax)
            .max_age(time::Duration::seconds(secs))
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

    query!("INSERT INTO sessions (user_id, session_id, expires_at, access_token, refresh_token)
            VALUES ($1, $2, $3, $4, $5)",
        user_id, session_id, max_age.and_utc(), token.access_token(), token.refresh_token())
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