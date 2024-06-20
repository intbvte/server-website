#[macro_use]
extern crate rocket;

use std::env;

use dotenvy::dotenv;
use rocket::State;
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::Deserialize;
use sqlx::query;

use crate::app::App;
use crate::errors::ApiError;

mod minecraft;
mod app;
mod errors;
mod session;

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

#[derive(Deserialize)]
struct DiscordAccessTokenResponse {
    access_token: String,
    _token_type: String,
    expires_in: i64,
    refresh_token: String,
    _scope: String,
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
        .mount("/backend/", routes![discord_login, discord_logout, discord_callback, minecraft_whitelist, minecraft_whitelist_remove])
        .attach(OAuth2::<Discord>::fairing("discord"))
}

#[get("/login/discord")]
async fn discord_login(app: &State<App>, oauth2: OAuth2<Discord>, cookies: &CookieJar<'_>) -> Redirect {
    let session_cookie = cookies.get_private("session_id");

    if let Some(cookie) = session_cookie {
        let session_id = cookie.value().to_owned();

        let session = query!("SELECT * FROM sessions WHERE session_id = $1 AND expired = FALSE AND expires_at > NOW()", session_id)
            .fetch_optional(&app.db)
            .await
            .unwrap();

        if let Some(session) = session {
            let req = app.https.post("https://discord.com/api/oauth2/token")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .basic_auth(
                    env::var("DISCORD_CLIENT_ID").expect("Missing client id"),
                    Some(env::var("DISCORD_CLIENT_SECRET").expect("Missing client secret")),
                )
                .body(format!("grant_type=refresh_token&refresh_token={}", session.refresh_token))
                .send()
                .await
                .unwrap()
                .json::<DiscordAccessTokenResponse>()
                .await
                .unwrap();

            query!("UPDATE sessions SET expired = true WHERE session_id = $1", session_id)
                .execute(&app.db)
                .await
                .unwrap();

            let session_cookie = session::generate_session(app, &req.access_token, &req.refresh_token, req.expires_in).await;
            cookies.add_private(session_cookie);

            return Redirect::to("/");
        }
    };

    oauth2.get_redirect(cookies, &["identify"]).unwrap()
}

#[get("/logout/discord")]
fn discord_logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private("session_id");

    Redirect::to("/")
}

#[get("/auth/discord")]
async fn discord_callback(app: &State<App>, token: TokenResponse<Discord>, cookies: &CookieJar<'_>) -> Result<Redirect, ApiError> {
    let Some(secs) = token.expires_in() else {
        return Err(ApiError::OptionError);
    };

    let user = app.https.get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token.access_token()))
        .send()
        .await
        .unwrap()
        .json::<DiscordCallback>()
        .await
        .unwrap();

    let user_id = user.id.parse::<i64>().expect("Failed to read user.id as a i64");

    query!("INSERT INTO users (discord_id, discord_username)
            VALUES ($1, $2)
            ON CONFLICT (discord_id) DO NOTHING;", user_id, user.username)
        .execute(&app.db)
        .await
        .unwrap();

    let session_cookie = session::generate_session_with_callback(app, user, token.access_token(), token.refresh_token().unwrap(), secs).await;
    cookies.add_private(session_cookie);

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