mod minecraft;
mod app;

#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::Redirect;
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::Deserialize;
use rocket::serde::json::Json;
use rocket::State;
use crate::app::App;

struct Discord;

#[derive(Deserialize)]
struct Whitelist {
    username: String
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .manage(App::new().await)
        .mount("/backend/", routes![discord_login, discord_callback, minecraft_whitelist, minecraft_whitelist_remove])
        .attach(OAuth2::<Discord>::fairing("discord"))
}

#[get("/login/discord")]
fn discord_login(oauth2: OAuth2<Discord>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["identify", "email"]).unwrap()
}

#[get("/auth/discord")]
fn discord_callback(app: &State<App>, token: TokenResponse<Discord>, cookies: &CookieJar<'_>) -> Redirect {
    cookies.add_private(
        Cookie::build(("token", token.access_token().to_string()))
            .same_site(SameSite::Lax)
            .build()
    );

    Redirect::to("/")
}

#[post("/minecraft/whitelist/add", format = "application/json", data = "<whitelist_data>")]
async fn minecraft_whitelist(app: &State<App>, whitelist_data: Json<Whitelist>) -> (Status, &'static str) {
    minecraft::run_command(
        app,
        format!("whitelist add {}", whitelist_data.username),
        "User was whitelisted successfully",
        format!("A unknown error occurred while whitelisting user {}", whitelist_data.username)
    ).await
}

#[post("/minecraft/whitelist/remove", format = "application/json", data = "<whitelist_data>")]
async fn minecraft_whitelist_remove(app: &State<App>, whitelist_data: Json<Whitelist>) -> (Status, &'static str) {
    minecraft::run_command(
        app,
        format!("whitelist remove {}", whitelist_data.username),
        "User was un-whitelisted successfully",
        format!("A unknown error occurred while un-whitelisting user {}", whitelist_data.username)
    ).await
}